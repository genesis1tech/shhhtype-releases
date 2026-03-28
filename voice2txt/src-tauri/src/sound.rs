// macOS sound feedback using AudioToolbox system sounds.

#[cfg(target_os = "macos")]
#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn AudioServicesPlaySystemSound(id: u32);
}

/// Play a short sound when recording starts.
pub fn play_start_sound() {
    #[cfg(target_os = "macos")]
    unsafe {
        AudioServicesPlaySystemSound(1103); // Tink
    }
}

/// Play a short sound when recording stops.
pub fn play_stop_sound() {
    #[cfg(target_os = "macos")]
    unsafe {
        AudioServicesPlaySystemSound(1107); // Purr
    }
}
