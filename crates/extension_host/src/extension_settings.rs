use collections::HashMap;
use extension::{
    DownloadFileCapability, ExtensionCapability, NpmInstallPackageCapability, ProcessExecCapability,
};
use settings::{RegisterSetting, Settings};
use std::sync::Arc;

#[derive(Debug, Default, Clone, RegisterSetting)]
pub struct ExtensionSettings {
    /// The base URL of the extension registry, without a trailing slash.
    ///
    /// `None` means that no extension registry is configured, in which case the
    /// extension store must not make any network requests.
    pub registry_url: Option<String>,
    /// The extensions that should be automatically installed from the extension registry.
    ///
    /// Default: {}
    pub auto_install_extensions: HashMap<Arc<str>, bool>,
    pub auto_update_extensions: HashMap<Arc<str>, bool>,
    pub suggest_extensions: bool,
    pub granted_capabilities: Vec<ExtensionCapability>,
}

impl ExtensionSettings {
    /// Returns whether the given extension should be auto-installed.
    pub fn should_auto_install(&self, extension_id: &str) -> bool {
        self.auto_install_extensions
            .get(extension_id)
            .copied()
            .unwrap_or(true)
    }

    pub fn should_auto_update(&self, extension_id: &str) -> bool {
        self.auto_update_extensions
            .get(extension_id)
            .copied()
            .unwrap_or(true)
    }
}

impl Settings for ExtensionSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            registry_url: content
                .extension
                .extension_registry_url
                .as_deref()
                .map(|registry_url| registry_url.trim().trim_end_matches('/'))
                .filter(|registry_url| !registry_url.is_empty())
                .map(str::to_string),
            auto_install_extensions: content.extension.auto_install_extensions.clone(),
            auto_update_extensions: content.extension.auto_update_extensions.clone(),
            suggest_extensions: content.extension.suggest_extensions,
            granted_capabilities: content
                .extension
                .granted_extension_capabilities
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|capability| match capability {
                    settings::ExtensionCapabilityContent::ProcessExec { command, args } => {
                        ExtensionCapability::ProcessExec(ProcessExecCapability { command, args })
                    }
                    settings::ExtensionCapabilityContent::DownloadFile { host, path } => {
                        ExtensionCapability::DownloadFile(DownloadFileCapability { host, path })
                    }
                    settings::ExtensionCapabilityContent::NpmInstallPackage { package } => {
                        ExtensionCapability::NpmInstallPackage(NpmInstallPackageCapability {
                            package,
                        })
                    }
                })
                .collect(),
        }
    }
}
