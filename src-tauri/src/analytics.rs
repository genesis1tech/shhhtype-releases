use crate::license::get_machine_id;
use std::sync::LazyLock;

const POSTHOG_URL: &str = "https://us.i.posthog.com/capture/";
const POSTHOG_API_KEY: &str = "phc_W7QVI64kLyNgP23d0lb925XSUQB6GOIES5r3gyh6Jzq";

/// Reuse a single HTTP client across requests (saves TLS handshake).
static HTTP_CLIENT: LazyLock<reqwest::blocking::Client> = LazyLock::new(|| {
    reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("Failed to build PostHog HTTP client")
});

/// Track an event to PostHog. Fire-and-forget — spawns a background thread,
/// never blocks, and silently logs errors without propagating them.
pub fn track(event: &str, properties: serde_json::Value) {
    let event = event.to_string();
    let properties = properties;

    std::thread::spawn(move || {
        let distinct_id = get_machine_id();

        // Merge user properties with standard PostHog properties
        let mut props = match properties {
            serde_json::Value::Object(map) => map,
            _ => serde_json::Map::new(),
        };
        props.insert("$app_version".into(), env!("CARGO_PKG_VERSION").into());
        props.insert("$os".into(), std::env::consts::OS.into());
        props.insert("$arch".into(), std::env::consts::ARCH.into());

        let body = serde_json::json!({
            "api_key": POSTHOG_API_KEY,
            "event": event,
            "distinct_id": distinct_id,
            "properties": props,
        });

        match HTTP_CLIENT.post(POSTHOG_URL).json(&body).send() {
            Ok(resp) if resp.status().is_success() => {
                log::info!("PostHog: tracked {}", event);
            }
            Ok(resp) => {
                log::warn!("PostHog: {} returned {}", event, resp.status());
            }
            Err(e) => {
                log::warn!("PostHog: failed to track {}: {}", event, e);
            }
        }
    });
}
