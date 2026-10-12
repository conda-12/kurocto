pub mod telemetry;
pub mod user;
pub mod zed_urls;

use clock::SystemClock;
use credentials_provider::CredentialsProvider;
use gpui::{App, Global};
use http_client::{HttpClientWithUrl, read_proxy_from_env};
use serde::Deserialize;
use settings::{RegisterSetting, Settings, SettingsContent};
use std::{
    path::PathBuf,
    sync::{Arc, LazyLock},
};
use telemetry::Telemetry;
use url::Url;

pub use rpc::*;
pub use telemetry_events::Event;
pub use user::*;

pub static ZED_APP_PATH: LazyLock<Option<PathBuf>> =
    LazyLock::new(|| std::env::var("ZED_APP_PATH").ok().map(PathBuf::from));

pub static ZED_ALWAYS_ACTIVE: LazyLock<bool> =
    LazyLock::new(|| std::env::var("ZED_ALWAYS_ACTIVE").is_ok_and(|e| !e.is_empty()));

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
}

#[derive(Copy, Clone, Deserialize, Debug, RegisterSetting)]
pub struct TelemetrySettings {
    pub metrics: bool,
    pub anthropic_retention: bool,
}

impl settings::Settings for TelemetrySettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let telemetry = content.telemetry.as_ref().unwrap();
        Self {
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
            telemetry: Telemetry::new(clock, cx),
            http,
            credentials_provider: zed_credentials_provider::global(cx),
        })
    }

    pub fn production(cx: &mut App) -> Arc<Self> {
        let clock = Arc::new(clock::RealSystemClock);
        let http = Arc::new(HttpClientWithUrl::new_url(
            cx.http_client(),
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

    pub fn telemetry(&self) -> &Arc<Telemetry> {
        &self.telemetry
    }
}

/// The URL scheme registered with the operating system for links into the application.
pub const ZED_URL_SCHEME: &str = "kurocto";

#[cfg(test)]
mod tests {
    use super::*;

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
}
