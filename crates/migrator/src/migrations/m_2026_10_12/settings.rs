use anyhow::Result;
use serde_json::{Map, Value};

use crate::migrations::migrate_settings;

pub fn remove_zed_edit_prediction_settings(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(settings: &mut Map<String, Value>) -> Result<()> {
    let Some(edit_predictions) = settings
        .get_mut("edit_predictions")
        .and_then(Value::as_object_mut)
    else {
        return Ok(());
    };

    if edit_predictions.get("provider").and_then(Value::as_str) == Some("zed") {
        edit_predictions.insert("provider".to_string(), Value::String("none".to_string()));
    }
    edit_predictions.remove("zed");
    edit_predictions.remove("allow_data_collection");

    if edit_predictions.is_empty() {
        settings.remove("edit_predictions");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn replaces_zed_provider_and_removes_hosted_settings_in_all_scopes() {
        let legacy = json!({
            "edit_predictions": {
                "provider": "zed",
                "mode": "subtle",
                "zed": { "prediction_debounce": 100 },
                "allow_data_collection": "yes"
            }
        });
        let mut value = json!({
            "edit_predictions": legacy["edit_predictions"],
            "linux": legacy,
            "stable": legacy,
            "profiles": {
                "direct": legacy,
                "wrapped": { "settings": legacy }
            }
        });

        remove_zed_edit_prediction_settings(&mut value).expect("migration should succeed");
        for scope in [
            &value,
            &value["linux"],
            &value["stable"],
            &value["profiles"]["direct"],
            &value["profiles"]["wrapped"]["settings"],
        ] {
            assert_eq!(
                scope["edit_predictions"],
                json!({ "provider": "none", "mode": "subtle" })
            );
        }
    }

    #[test]
    fn keeps_other_providers_and_their_settings() {
        let original = json!({
            "edit_predictions": {
                "provider": "ollama",
                "ollama": { "model": "zeta", "prompt_format": "zeta" },
                "mercury": { "prediction_debounce": 10 }
            }
        });
        let mut value = original.clone();

        remove_zed_edit_prediction_settings(&mut value).expect("migration should succeed");
        assert_eq!(value, original);
    }

    #[test]
    fn removes_edit_predictions_when_only_hosted_settings_remain() {
        let mut value = json!({
            "edit_predictions": {
                "zed": { "prediction_debounce": 100 },
                "allow_data_collection": "no"
            },
            "tab_size": 2
        });

        remove_zed_edit_prediction_settings(&mut value).expect("migration should succeed");
        assert_eq!(value, json!({ "tab_size": 2 }));
    }
}
