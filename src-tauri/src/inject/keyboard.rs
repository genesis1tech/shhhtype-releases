use anyhow::Result;

/// Inject text character-by-character using CGEvent keyboard simulation.
/// Better for short text; triggers autocomplete in IDEs.
pub fn inject_via_keyboard(text: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        use core_graphics::event::{CGEvent, CGKeyCode};
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        let source =
            CGEventSource::new(CGEventSourceStateID::HIDSystemState).map_err(|_| {
                anyhow::anyhow!("Failed to create CGEventSource")
            })?;

        for ch in text.chars() {
            // Create a keyboard event and set the Unicode string
            let dummy_keycode: CGKeyCode = 0;
            let key_down =
                CGEvent::new_keyboard_event(source.clone(), dummy_keycode, true)
                    .map_err(|_| anyhow::anyhow!("Failed to create key event"))?;

            let mut buf = [0u16; 2];
            let encoded: Vec<u16> = ch.encode_utf16(&mut buf).to_vec();
            key_down.set_string_from_utf16_unchecked(&encoded);

            let key_up =
                CGEvent::new_keyboard_event(source.clone(), dummy_keycode, false)
                    .map_err(|_| anyhow::anyhow!("Failed to create key event"))?;

            key_down.post(core_graphics::event::CGEventTapLocation::HID);
            key_up.post(core_graphics::event::CGEventTapLocation::HID);

            // Small delay between keystrokes to not overwhelm the target app
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        log::info!("Injected {} chars via keyboard simulation", text.len());
    }

    Ok(())
}
