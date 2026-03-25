use anyhow::Result;

/// Copy text to clipboard without pasting (for auto_copy feature).
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(text)?;
    log::info!("Copied {} chars to clipboard", text.len());
    Ok(())
}

/// Check if the process has Accessibility permission (needed for CGEvent).
#[cfg(target_os = "macos")]
fn is_accessibility_trusted() -> bool {
    extern "C" {
        fn AXIsProcessTrusted() -> bool;
    }
    unsafe { AXIsProcessTrusted() }
}

/// Inject text into the focused application via clipboard + Cmd+V.
/// This is the most compatible method (works in ~98% of apps).
///
/// Returns an error if accessibility permission is missing. In that case the text
/// remains on the clipboard but Cmd+V was NOT simulated — callers should treat
/// this as a copy-only fallback and NOT update composition state.
pub fn inject_via_clipboard(text: &str) -> Result<()> {
    // 1. Save current clipboard content
    let mut clipboard = arboard::Clipboard::new()?;
    let previous = clipboard.get_text().unwrap_or_default();

    // 2. Set our text to clipboard
    clipboard.set_text(text)?;

    // 3. Check accessibility before attempting CGEvent
    #[cfg(target_os = "macos")]
    if !is_accessibility_trusted() {
        log::warn!("Accessibility not trusted — text copied to clipboard but cannot simulate Cmd+V. Toggle Accessibility in System Settings.");
        return Err(anyhow::anyhow!(
            "Accessibility permission required for text injection. Text copied to clipboard instead. \
             Enable Accessibility in System Settings > Privacy & Security > Accessibility."
        ));
    }

    // 4. Delay to ensure clipboard is ready and target app has focus
    std::thread::sleep(std::time::Duration::from_millis(100));

    // 5. Simulate Cmd+V keystroke
    simulate_cmd_v()?;

    // 6. Wait for the target app to process the paste before restoring
    std::thread::sleep(std::time::Duration::from_millis(500));
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

        key_down.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
        std::thread::sleep(std::time::Duration::from_millis(20));
        key_up.post(core_graphics::event::CGEventTapLocation::AnnotatedSession);
    }

    Ok(())
}
