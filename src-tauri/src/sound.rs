// macOS sound feedback using NSSound for reliable playback from any thread.

#[cfg(target_os = "macos")]
#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn AudioServicesPlaySystemSound(id: u32);
}

/// Play a short sound when recording starts.
pub fn play_start_sound() {
    #[cfg(target_os = "macos")]
    unsafe {
        use objc::{class, msg_send, sel, sel_impl};
        use objc::runtime::Object;
        let name_str: *mut Object = msg_send![class!(NSString), stringWithUTF8String: b"Pop\0".as_ptr()];
        let sound: *mut Object = msg_send![class!(NSSound), soundNamed: name_str];
        if !sound.is_null() {
            let _: () = msg_send![sound, play];
        } else {
            AudioServicesPlaySystemSound(1057);
        }
    }
}

/// Play a short sound when recording stops.
pub fn play_stop_sound() {
    #[cfg(target_os = "macos")]
    unsafe {
        use objc::{class, msg_send, sel, sel_impl};
        use objc::runtime::Object;
        let name_str: *mut Object = msg_send![class!(NSString), stringWithUTF8String: b"Pop\0".as_ptr()];
        let sound: *mut Object = msg_send![class!(NSSound), soundNamed: name_str];
        if !sound.is_null() {
            let _: () = msg_send![sound, play];
        } else {
            AudioServicesPlaySystemSound(1057);
        }
    }
}
