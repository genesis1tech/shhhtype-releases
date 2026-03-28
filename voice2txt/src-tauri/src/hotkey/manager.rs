/// Hotkey activation mode.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum HotkeyMode {
    /// Hold to record, release to stop.
    PushToTalk,
    /// Press once to start, press again to stop.
    Toggle,
}

impl Default for HotkeyMode {
    fn default() -> Self {
        HotkeyMode::PushToTalk
    }
}
