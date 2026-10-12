pub mod telemetry;
pub mod user;
pub mod zed_urls;

use anyhow::{Result, anyhow};
use clock::SystemClock;
use credentials_provider::CredentialsProvider;
use futures::{
    FutureExt,
    future::{self, BoxFuture},
    stream::BoxStream,
};
use gpui::{App, AsyncApp, Entity, Global, WeakEntity};
use http_client::{HttpClientWithUrl, read_proxy_from_env};
use parking_lot::Mutex;
use rpc::proto::{EnvelopedMessage, RequestMessage};
use serde::Deserialize;
use settings::{RegisterSetting, Settings, SettingsContent};
use std::{
    any::TypeId,
    future::Future,
    marker::PhantomData,
    path::PathBuf,
    sync::{Arc, LazyLock, Weak},
};
use telemetry::Telemetry;
use url::Url;

pub use rpc::*;
pub use telemetry_events::Event;
pub use user::*;

static ZED_SERVER_URL: LazyLock<Option<String>> =
    LazyLock::new(|| std::env::var("ZED_SERVER_URL").ok());

pub static ZED_APP_PATH: LazyLock<Option<PathBuf>> =
    LazyLock::new(|| std::env::var("ZED_APP_PATH").ok().map(PathBuf::from));

pub static ZED_ALWAYS_ACTIVE: LazyLock<bool> =
    LazyLock::new(|| std::env::var("ZED_ALWAYS_ACTIVE").is_ok_and(|e| !e.is_empty()));

#[derive(Deserialize, RegisterSetting)]
pub struct ClientSettings {
    pub server_url: String,
}

impl Settings for ClientSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        if let Some(server_url) = &*ZED_SERVER_URL {
            return Self {
                server_url: server_url.clone(),
            };
        }
        Self {
            server_url: content.server_url.clone().unwrap(),
        }
    }
}

#[derive(Deserialize, Default, RegisterSetting)]
pub struct ProxySettings {
    pub proxy: Option<String>,
}

impl ProxySettings {
    pub fn proxy_url(&self) -> Option<Url> {
        self.proxy
            .as_deref()
            .map(str::trim)
            .filter(|input| !input.is_empty())
            .and_then(|input| {
                input
                    .parse::<Url>()
                    .inspect_err(|e| log::error!("Error parsing proxy settings: {}", e))
                    .ok()
            })
            .or_else(read_proxy_from_env)
    }
}

impl Settings for ProxySettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            proxy: content
                .proxy
                .as_deref()
                .map(str::trim)
                .filter(|proxy| !proxy.is_empty())
                .map(ToOwned::to_owned),
        }
    }
}

struct GlobalClient(Arc<Client>);

impl Global for GlobalClient {}

pub struct Client {
    http: Arc<HttpClientWithUrl>,
    telemetry: Arc<Telemetry>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    handler_set: Mutex<ProtoMessageHandlerSet>,
}

pub enum Subscription {
    Entity {
        client: Weak<Client>,
        id: (TypeId, u64),
    },
    Message {
        client: Weak<Client>,
        id: TypeId,
    },
}

impl Drop for Subscription {
    fn drop(&mut self) {
        match self {
            Subscription::Entity { client, id } => {
                if let Some(client) = client.upgrade() {
                    let mut state = client.handler_set.lock();
                    let _ = state.entities_by_type_and_remote_id.remove(id);
                }
            }
            Subscription::Message { client, id } => {
                if let Some(client) = client.upgrade() {
                    let mut state = client.handler_set.lock();
                    let _ = state.entity_types_by_message_type.remove(id);
                    let _ = state.message_handlers.remove(id);
                }
            }
        }
    }
}

pub struct PendingEntitySubscription<T: 'static> {
    client: Arc<Client>,
    remote_id: u64,
    _entity_type: PhantomData<T>,
    consumed: bool,
}

impl<T: 'static> PendingEntitySubscription<T> {
    pub fn set_entity(mut self, entity: &Entity<T>) -> Subscription {
        self.consumed = true;
        let id = (TypeId::of::<T>(), self.remote_id);
        self.client
            .handler_set
            .lock()
            .entities_by_type_and_remote_id
            .insert(
                id,
                EntityMessageSubscriber::Entity {
                    handle: entity.downgrade().into(),
                },
            );
        Subscription::Entity {
            client: Arc::downgrade(&self.client),
            id,
        }
    }
}

impl<T: 'static> Drop for PendingEntitySubscription<T> {
    fn drop(&mut self) {
        if !self.consumed {
            self.client
                .handler_set
                .lock()
                .entities_by_type_and_remote_id
                .remove(&(TypeId::of::<T>(), self.remote_id));
        }
    }
}

#[derive(Copy, Clone, Deserialize, Debug, RegisterSetting)]
pub struct TelemetrySettings {
    pub diagnostics: bool,
    pub metrics: bool,
    pub anthropic_retention: bool,
}

impl settings::Settings for TelemetrySettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let telemetry = content.telemetry.as_ref().unwrap();
        Self {
            diagnostics: telemetry.diagnostics.unwrap(),
            metrics: telemetry.metrics.unwrap(),
            anthropic_retention: telemetry.anthropic_retention.unwrap(),
        }
    }
}

impl Client {
    pub fn new(
        clock: Arc<dyn SystemClock>,
        http: Arc<HttpClientWithUrl>,
        cx: &mut App,
    ) -> Arc<Self> {
        Arc::new(Self {
            telemetry: Telemetry::new(clock, http.clone(), cx),
            http,
            credentials_provider: zed_credentials_provider::global(cx),
            handler_set: Default::default(),
        })
    }

    pub fn production(cx: &mut App) -> Arc<Self> {
        let clock = Arc::new(clock::RealSystemClock);
        let http = Arc::new(HttpClientWithUrl::new_url(
            cx.http_client(),
            &ClientSettings::get_global(cx).server_url,
            cx.http_client().proxy().cloned(),
        ));
        Self::new(clock, http, cx)
    }

    pub fn http_client(&self) -> Arc<HttpClientWithUrl> {
        self.http.clone()
    }

    pub fn credentials_provider(&self) -> Arc<dyn CredentialsProvider> {
        self.credentials_provider.clone()
    }

    pub fn global(cx: &App) -> Arc<Self> {
        cx.global::<GlobalClient>().0.clone()
    }

    pub fn set_global(client: Arc<Client>, cx: &mut App) {
        cx.set_global(GlobalClient(client))
    }

    pub fn subscribe_to_entity<T>(
        self: &Arc<Self>,
        remote_id: u64,
    ) -> Result<PendingEntitySubscription<T>>
    where
        T: 'static,
    {
        let id = (TypeId::of::<T>(), remote_id);

        let mut state = self.handler_set.lock();
        anyhow::ensure!(
            !state.entities_by_type_and_remote_id.contains_key(&id),
            "already subscribed to entity"
        );

        state
            .entities_by_type_and_remote_id
            .insert(id, EntityMessageSubscriber::Pending(Default::default()));

        Ok(PendingEntitySubscription {
            client: self.clone(),
            remote_id,
            consumed: false,
            _entity_type: PhantomData,
        })
    }

    #[track_caller]
    pub fn add_message_handler<M, E, H, F>(
        self: &Arc<Self>,
        entity: WeakEntity<E>,
        handler: H,
    ) -> Subscription
    where
        M: EnvelopedMessage,
        E: 'static,
        H: 'static + Sync + Fn(Entity<E>, TypedEnvelope<M>, AsyncApp) -> F + Send + Sync,
        F: 'static + Future<Output = Result<()>>,
    {
        self.add_message_handler_impl(entity, move |entity, message, _, cx| {
            handler(entity, message, cx)
        })
    }

    fn add_message_handler_impl<M, E, H, F>(
        self: &Arc<Self>,
        entity: WeakEntity<E>,
        handler: H,
    ) -> Subscription
    where
        M: EnvelopedMessage,
        E: 'static,
        H: 'static
            + Sync
            + Fn(Entity<E>, TypedEnvelope<M>, AnyProtoClient, AsyncApp) -> F
            + Send
            + Sync,
        F: 'static + Future<Output = Result<()>>,
    {
        let message_type_id = TypeId::of::<M>();
        let mut state = self.handler_set.lock();
        state
            .entities_by_message_type
            .insert(message_type_id, entity.into());

        let prev_handler = state.message_handlers.insert(
            message_type_id,
            Arc::new(move |subscriber, envelope, client, cx| {
                let subscriber = subscriber.downcast::<E>().unwrap();
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                handler(subscriber, *envelope, client, cx).boxed_local()
            }),
        );
        if prev_handler.is_some() {
            let location = std::panic::Location::caller();
            panic!(
                "{}:{} registered handler for the same message {} twice",
                location.file(),
                location.line(),
                std::any::type_name::<M>()
            );
        }

        Subscription::Message {
            client: Arc::downgrade(self),
            id: message_type_id,
        }
    }

    pub fn add_request_handler<M, E, H, F>(
        self: &Arc<Self>,
        entity: WeakEntity<E>,
        handler: H,
    ) -> Subscription
    where
        M: RequestMessage,
        E: 'static,
        H: 'static + Sync + Fn(Entity<E>, TypedEnvelope<M>, AsyncApp) -> F + Send + Sync,
        F: 'static + Future<Output = Result<M::Response>>,
    {
        self.add_message_handler_impl(entity, move |handle, envelope, this, cx| {
            Self::respond_to_request(envelope.receipt(), handler(handle, envelope, cx), this)
        })
    }

    async fn respond_to_request<T: RequestMessage, F: Future<Output = Result<T::Response>>>(
        receipt: Receipt<T>,
        response: F,
        client: AnyProtoClient,
    ) -> Result<()> {
        match response.await {
            Ok(response) => {
                client.send_response(receipt.message_id, response)?;
                Ok(())
            }
            Err(error) => {
                client.send_response(receipt.message_id, error.to_proto())?;
                Err(error)
            }
        }
    }

    pub fn send<T: EnvelopedMessage>(&self, _message: T) -> Result<()> {
        Err(not_connected())
    }

    pub fn request<T: RequestMessage>(
        &self,
        _request: T,
    ) -> impl Future<Output = Result<T::Response>> + use<T> {
        future::ready(Err(not_connected()))
    }

    pub fn request_envelope<T: RequestMessage>(
        &self,
        _request: T,
    ) -> impl Future<Output = Result<TypedEnvelope<T::Response>>> + use<T> {
        future::ready(Err(not_connected()))
    }

    pub fn telemetry(&self) -> &Arc<Telemetry> {
        &self.telemetry
    }
}

fn not_connected() -> anyhow::Error {
    anyhow!("not connected")
}

impl ProtoClient for Client {
    fn request(
        &self,
        _envelope: proto::Envelope,
        _request_type: &'static str,
    ) -> BoxFuture<'static, Result<proto::Envelope>> {
        future::ready(Err(not_connected())).boxed()
    }

    fn request_stream(
        &self,
        _envelope: proto::Envelope,
        _request_type: &'static str,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<proto::Envelope>>>> {
        future::ready(Err(not_connected())).boxed()
    }

    fn send(&self, _envelope: proto::Envelope, _message_type: &'static str) -> Result<()> {
        Err(not_connected())
    }

    fn send_response(&self, _envelope: proto::Envelope, _message_type: &'static str) -> Result<()> {
        Err(not_connected())
    }

    fn message_handler_set(&self) -> &Mutex<ProtoMessageHandlerSet> {
        &self.handler_set
    }

    fn is_via_collab(&self) -> bool {
        true
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }
}

/// prefix for the zed:// url scheme
pub const ZED_URL_SCHEME: &str = "zed";

#[cfg(test)]
mod tests {
    use super::*;

    use clock::FakeSystemClock;
    use gpui::{AppContext as _, TestAppContext};
    use http_client::FakeHttpClient;
    use settings::SettingsStore;

    #[test]
    fn test_proxy_settings_trims_and_ignores_empty_proxy() {
        let mut content = SettingsContent::default();
        content.proxy = Some("   ".to_owned());
        assert_eq!(ProxySettings::from_settings(&content).proxy, None);

        content.proxy = Some("http://127.0.0.1:10809".to_owned());
        assert_eq!(
            ProxySettings::from_settings(&content).proxy.as_deref(),
            Some("http://127.0.0.1:10809")
        );
    }

    #[gpui::test]
    async fn test_rpc_fails_when_not_connected(cx: &mut TestAppContext) {
        let client = init_test(cx);

        let error = client.request(proto::Ping {}).await.unwrap_err();
        assert_eq!(error.to_string(), "not connected");

        let proto_client = AnyProtoClient::from(client);
        let error = proto_client.request(proto::Ping {}).await.unwrap_err();
        assert_eq!(error.to_string(), "not connected");
        let error = proto_client.send(proto::Ping {}).unwrap_err();
        assert_eq!(error.to_string(), "not connected");
    }

    #[gpui::test]
    async fn test_subscribing_to_entity(cx: &mut TestAppContext) {
        let client = init_test(cx);
        let entity = cx.new(|_| TestEntity);

        let pending_subscription = client.subscribe_to_entity::<TestEntity>(1).unwrap();
        assert!(client.subscribe_to_entity::<TestEntity>(1).is_err());
        drop(pending_subscription);

        let subscription = client
            .subscribe_to_entity::<TestEntity>(1)
            .unwrap()
            .set_entity(&entity);
        assert!(client.subscribe_to_entity::<TestEntity>(1).is_err());
        drop(subscription);

        client.subscribe_to_entity::<TestEntity>(1).unwrap();
    }

    #[gpui::test]
    async fn test_subscribing_after_dropping_subscription(cx: &mut TestAppContext) {
        let client = init_test(cx);
        let entity = cx.new(|_| TestEntity);

        let subscription = client.add_message_handler(
            entity.downgrade(),
            |_, _: TypedEnvelope<proto::Ping>, _| async { Ok(()) },
        );
        drop(subscription);
        let _subscription = client.add_message_handler(
            entity.downgrade(),
            |_, _: TypedEnvelope<proto::Ping>, _| async { Ok(()) },
        );
    }

    struct TestEntity;

    fn init_test(cx: &mut TestAppContext) -> Arc<Client> {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Client::new(
                Arc::new(FakeSystemClock::new()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        })
    }
}
