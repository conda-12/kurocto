use anyhow::{Context as _, Result};
use collections::HashMap;
use cpal::{
    DeviceDescription, DeviceId, Error, ErrorKind, default_host,
    traits::{DeviceTrait, HostTrait},
};
use futures::channel::oneshot;
use gpui::{App, AsyncApp, BorrowAppContext, Global, Task};

use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Source, mixer::Mixer, source::Buffered};
use settings::Settings;
use std::io::Cursor;
use util::ResultExt;

use crate::{AudioSettings, Sound};

pub fn init(_cx: &mut App) {}

// TODO(jk): this is currently cached only once - we should observe and react instead
pub fn ensure_devices_initialized(cx: &mut App) {
    if cx.has_global::<AvailableAudioDevices>() {
        return;
    }
    cx.default_global::<AvailableAudioDevices>();
    let task = cx
        .background_executor()
        .spawn(async move { get_available_audio_devices() });
    cx.spawn(async move |cx: &mut AsyncApp| {
        let devices = task.await;
        cx.update(|cx| cx.set_global(AvailableAudioDevices(devices)));
        cx.refresh();
    })
    .detach();
}

#[derive(Default)]
pub struct Audio {
    output: Option<MixerDeviceSink>,
    output_error_task: Option<Task<()>>,
    source_cache: HashMap<Sound, Buffered<Decoder<Cursor<Vec<u8>>>>>,
}

impl Global for Audio {}

impl Audio {
    fn ensure_output_exists(
        &mut self,
        output_audio_device: Option<DeviceId>,
        cx: &mut App,
    ) -> Result<&Mixer> {
        #[cfg(debug_assertions)]
        log::warn!(
            "Audio does not sound correct without optimizations. Use a release build to debug audio issues"
        );

        if self.output.is_none() {
            let (error_callback, error_task) = output_error_handler(cx);
            self.output = Some(open_output_stream(output_audio_device, error_callback)?);
            self.output_error_task = Some(error_task);
        }

        self.output
            .as_ref()
            .map(MixerDeviceSink::mixer)
            .context("Output stream is not open")
    }

    pub fn play_sound(sound: Sound, cx: &mut App) {
        let output_audio_device = AudioSettings::get_global(cx).output_audio_device.clone();
        cx.update_default_global(|this: &mut Self, cx| {
            let source = this.sound_source(sound, cx).log_err()?;
            let output_mixer = this
                .ensure_output_exists(output_audio_device, cx)
                .context("Could not get output mixer")
                .log_err()?;

            output_mixer.add(source);
            Some(())
        });
    }

    fn close_output(cx: &mut App) {
        cx.update_default_global(|audio: &mut Self, _cx| {
            audio.output_error_task.take();
            audio.output.take();
        });
    }

    fn sound_source(&mut self, sound: Sound, cx: &App) -> Result<impl Source + use<>> {
        if let Some(wav) = self.source_cache.get(&sound) {
            return Ok(wav.clone());
        }

        let path = format!("sounds/{}.wav", sound.file());
        let bytes = cx
            .asset_source()
            .load(&path)?
            .map(anyhow::Ok)
            .with_context(|| format!("No asset available for path {path}"))??
            .into_owned();
        let cursor = Cursor::new(bytes);
        let source = Decoder::new(cursor)?.buffered();

        self.source_cache.insert(sound, source.clone());

        Ok(source)
    }
}

fn resolve_output_device(device_id: Option<&DeviceId>) -> anyhow::Result<cpal::Device> {
    if let Some(id) = device_id {
        if let Some(device) = default_host().device_by_id(id) {
            return Ok(device);
        }
        log::warn!("Selected audio device not found, falling back to default");
    }
    default_host()
        .default_output_device()
        .context("no audio output device available")
}

fn open_output_stream(
    device_id: Option<DeviceId>,
    error_callback: impl FnMut(Error) + Send + 'static,
) -> anyhow::Result<MixerDeviceSink> {
    let device = resolve_output_device(device_id.as_ref())?;
    let mut output_handle = DeviceSinkBuilder::from_device(device)?
        .with_buffer_size(cpal::BufferSize::Default)
        .with_error_callback(error_callback)
        .open_stream()
        .context("Could not open output stream")?;
    output_handle.log_on_drop(false);
    log::info!("Output stream: {:?}", output_handle);

    Ok(output_handle)
}

#[derive(Clone, Debug)]
pub struct AudioDeviceInfo {
    pub id: DeviceId,
    pub desc: DeviceDescription,
}

impl AudioDeviceInfo {
    pub fn supports_output(&self) -> bool {
        self.desc.supports_output()
    }

    pub fn matches_output(&self, id: &DeviceId) -> bool {
        &self.id == id && self.supports_output()
    }
}

impl std::fmt::Display for AudioDeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.desc.name(), self.id)
    }
}

fn get_available_audio_devices() -> Vec<AudioDeviceInfo> {
    let Some(devices) = default_host().devices().ok() else {
        return Vec::new();
    };
    devices
        .filter_map(|device| {
            let id = device.id().ok()?;
            let desc = device.description().ok()?;
            Some(AudioDeviceInfo { id, desc })
        })
        .collect()
}

#[derive(Default, Clone, Debug)]
pub struct AvailableAudioDevices(pub Vec<AudioDeviceInfo>);

impl Global for AvailableAudioDevices {}

fn output_error_handler(cx: &mut App) -> (impl FnMut(Error) + Send + 'static + use<>, Task<()>) {
    let (sender, receiver) = oneshot::channel();
    let mut sender = Some(sender);
    let callback = move |error: Error| {
        if matches!(
            error.kind(),
            ErrorKind::Xrun | ErrorKind::DeviceChanged | ErrorKind::RealtimeDenied
        ) {
            return;
        }
        if let Some(sender) = sender.take() {
            sender.send(error).ok();
        }
    };
    let task = cx.spawn(async move |cx| {
        if let Ok(error) = receiver.await {
            log::error!("Audio output stream failed: {error}");
            cx.update(Audio::close_output);
        }
    });
    (callback, task)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn test_output_error_handler(cx: &mut TestAppContext) {
        for error_kind in [
            ErrorKind::DeviceNotAvailable,
            ErrorKind::StreamInvalidated,
            ErrorKind::BackendError,
        ] {
            let mut callback = cx.update(install_output_error_handler);
            for recoverable_kind in [
                ErrorKind::Xrun,
                ErrorKind::DeviceChanged,
                ErrorKind::RealtimeDenied,
            ] {
                callback(Error::new(recoverable_kind));
                cx.run_until_parked();
                cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_some()));
            }

            callback(Error::new(error_kind));
            callback(Error::new(error_kind));
            cx.run_until_parked();
            cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_none()));
        }
    }

    #[gpui::test]
    fn test_output_error_handler_cancellation(cx: &mut TestAppContext) {
        for error_before_close in [false, true] {
            let mut old_callback = cx.update(install_output_error_handler);
            if error_before_close {
                old_callback(Error::new(ErrorKind::DeviceNotAvailable));
            }
            cx.update(Audio::close_output);

            let mut callback = cx.update(install_output_error_handler);
            old_callback(Error::new(ErrorKind::DeviceNotAvailable));
            cx.run_until_parked();
            cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_some()));

            callback(Error::new(ErrorKind::StreamInvalidated));
            cx.run_until_parked();
            cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_none()));
        }

        let callback = cx.update(install_output_error_handler);
        drop(callback);
        cx.run_until_parked();
        cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_some()));
        cx.update(Audio::close_output);
    }

    #[gpui::test]
    fn test_audio_settings_defaults(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            assert_eq!(AudioSettings::get_global(cx).output_audio_device, None);
        });
    }

    #[test]
    fn test_agent_done_sound_asset_exists() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets/sounds")
            .join(format!("{}.wav", Sound::AgentDone.file()));
        assert!(path.is_file(), "missing sound asset {}", path.display());
    }

    fn install_output_error_handler(cx: &mut App) -> impl FnMut(Error) + Send + 'static + use<> {
        let (callback, task) = output_error_handler(cx);
        cx.update_default_global(|audio: &mut Audio, _cx| {
            audio.output_error_task = Some(task);
        });
        callback
    }
}
