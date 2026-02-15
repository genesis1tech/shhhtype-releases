use anyhow::Result;

/// Inject text into the focused application via clipboard + Cmd+V.
/// This is the most compatible method (works in ~98% of apps).
pub fn inject_via_clipboard(text: &str) -> Result<()> {
    // 1. Save current clipboard content
    let mut clipboard = arboard::Clipboard::new()?;
    let previous = clipboard.get_text().unwrap_or_default();

    // 2. Set our text to clipboard
    clipboard.set_text(text)?;

    // 3. Simulate Cmd+V keystroke
    simulate_cmd_v()?;

    // 4. Brief delay then restore previous clipboard
    std::thread::sleep(std::time::Duration::from_millis(100));
    let _ = clipboard.set_text(previous);

    log::info!("Injected {} chars via clipboard", text.len());
    Ok(())
}

/// Simulate Cmd+V using macOS CGEvent API.
fn simulate_cmd_v() -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        use core_graphics::event::{CGEvent, CGEventFlags, CGKeyCode};
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        let source =
            CGEventSource::new(CGEventSourceStateID::HIDSystemState).map_err(|_| {
                anyhow::anyhow!("Failed to create CGEventSource")
            })?;

        // 'v' key = keycode 9
        let v_keycode: CGKeyCode = 9;

        let key_down = CGEvent::new_keyboard_event(source.clone(), v_keycode, true)
            .map_err(|_| anyhow::anyhow!("Failed to create key down event"))?;
        key_down.set_flags(CGEventFlags::CGEventFlagCommand);

        let key_up = CGEvent::new_keyboard_event(source, v_keycode, false)
            .map_err(|_| anyhow::anyhow!("Failed to create key up event"))?;
        key_up.set_flags(CGEventFlags::CGEventFlagCommand);

        key_down.post(core_graphics::event::CGEventTapLocation::HID);
        key_up.post(core_graphics::event::CGEventTapLocation::HID);
    }

    Ok(())
}
