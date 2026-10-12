use audio::{AudioDeviceInfo, AvailableAudioDevices};
use cpal::DeviceId;
use gpui::{AnyElement, App, ElementId, ReadGlobal, SharedString, Window};
use settings::{AudioOutputDeviceName, SettingsStore};
use std::str::FromStr;
use ui::{ContextMenu, DropdownMenu, DropdownStyle, FluentBuilder, IconPosition, IntoElement};
use util::ResultExt;

use crate::{SettingField, SettingsFieldMetadata, SettingsUiFile, update_settings_file};

const SYSTEM_DEFAULT: &str = "System Default";

fn get_current_device(
    current_id: Option<&DeviceId>,
    devices: &[AudioDeviceInfo],
) -> Option<AudioDeviceInfo> {
    let current_id = current_id?;
    devices
        .iter()
        .find(|device| device.matches_output(current_id))
        .cloned()
}

fn render_audio_device_dropdown<F>(
    dropdown_id: impl Into<ElementId>,
    current_device_id: Option<DeviceId>,
    on_select: F,
    aria_label: Option<SharedString>,
    aria_description: Option<SharedString>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement
where
    F: Fn(Option<DeviceId>, &mut Window, &mut App) + Clone + 'static,
{
    audio::ensure_devices_initialized(cx);
    let devices = cx.global::<AvailableAudioDevices>().0.clone();
    let current_device = get_current_device(current_device_id.as_ref(), &devices);

    let menu = ContextMenu::build(window, cx, {
        let current_device = current_device.clone();
        move |mut menu, _, _cx| {
            let is_system_default = current_device.is_none();
            menu = menu.toggleable_entry(
                SYSTEM_DEFAULT,
                is_system_default,
                IconPosition::Start,
                None,
                {
                    let on_select = on_select.clone();
                    move |window, cx| {
                        on_select(None, window, cx);
                    }
                },
            );

            for device in devices.iter().filter(|device| device.supports_output()) {
                let is_current = current_device
                    .as_ref()
                    .map(|info| info.matches_output(&device.id))
                    .unwrap_or(false);
                let device_id = device.id.clone();

                menu = menu.toggleable_entry(
                    device.to_string(),
                    is_current,
                    IconPosition::Start,
                    None,
                    {
                        let on_select = on_select.clone();
                        move |window, cx| {
                            on_select(Some(device_id.clone()), window, cx);
                        }
                    },
                );
            }
            menu
        }
    });

    DropdownMenu::new(
        dropdown_id,
        current_device
            .map(|info| info.desc.name().to_string())
            .unwrap_or(SYSTEM_DEFAULT.to_string()),
        menu,
    )
    .style(DropdownStyle::Outlined)
    .full_width(true)
    .when_some(aria_label, |this, label| this.aria_label(label))
    .when_some(aria_description, |this, description| {
        this.aria_description(description)
    })
    .into_any_element()
}

pub fn render_output_audio_device_dropdown(
    field: SettingField<AudioOutputDeviceName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    title: &'static str,
    description: &'static str,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, current_value): (_, Option<&AudioOutputDeviceName>) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_device_id = current_value
        .and_then(|name| name.0.as_ref())
        .and_then(|id| DeviceId::from_str(id).ok());

    render_audio_device_dropdown(
        "output-audio-device-dropdown",
        current_device_id,
        move |device_id, window, cx| {
            let value = device_id.map(|id| AudioOutputDeviceName(Some(id.to_string())));
            update_settings_file(
                file.clone(),
                field.json_path,
                window,
                cx,
                move |settings, app| {
                    (field.write)(settings, value, app);
                },
            )
            .log_err();
        },
        Some(SharedString::new_static(title)),
        (!description.is_empty()).then(|| SharedString::new_static(description)),
        window,
        cx,
    )
}
