mod audio_settings;
pub use audio_settings::AudioSettings;

mod audio_pipeline;
pub use audio_pipeline::Audio;
pub use audio_pipeline::init;
pub use audio_pipeline::{AudioDeviceInfo, AvailableAudioDevices, ensure_devices_initialized};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Sound {
    AgentDone,
}

impl Sound {
    fn file(&self) -> &'static str {
        match self {
            Self::AgentDone => "agent_done",
        }
    }
}
