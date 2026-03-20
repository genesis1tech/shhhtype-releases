use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// macOS window level for the overlay panel — NSPopUpMenuWindowLevel (101).
/// This ensures the overlay renders above all normal windows, floating panels,
/// and modal dialogs. Combined with NSPanel swizzle, it also appears over
/// full-screen Spaces.
#[cfg(target_os = "macos")]
pub const OVERLAY_WINDOW_LEVEL: i64 = 101;

/// Swizzle a Tauri-created NSWindow into an NSPanel at runtime.
///
/// NSPanel is the only window type macOS allows to appear over full-screen apps.
/// This performs an ISA swizzle (object_setClass) to convert the underlying
/// NSWindow to NSPanel, then sets the NonactivatingPanel style mask so the
/// overlay never steals focus from the user's app.
#[cfg(target_os = "macos")]
pub unsafe fn swizzle_to_nspanel(ns_window: cocoa::base::id) {
    use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
    use objc::runtime::Class;
    use objc::{msg_send, sel, sel_impl};

    extern "C" {
        fn object_setClass(
            obj: *mut objc::runtime::Object,
            cls: *const Class,
        ) -> *mut objc::runtime::Object;
    }

    // Swizzle the window's ISA to NSPanel (a subclass of NSWindow)
    let nspanel_class = Class::get("NSPanel").expect("NSPanel class not found");
    object_setClass(ns_window as *mut objc::runtime::Object, nspanel_class);

    // Add NonactivatingPanel style mask (1 << 7 = 128) to prevent focus stealing
    let current_mask: u64 = msg_send![ns_window, styleMask];
    let non_activating: u64 = 1 << 7; // NSWindowStyleMaskNonactivatingPanel
    let _: () = msg_send![ns_window, setStyleMask: current_mask | non_activating];

    // Sync WindowServer tags after style mask change (private but stable API)
    let _: () = msg_send![ns_window, _setPreventsActivation: true];

    // Collection behaviors: appear in all Spaces including full-screen
    ns_window.setCollectionBehavior_(
        NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle,
    );

    // Set window level
    ns_window.setLevel_(OVERLAY_WINDOW_LEVEL);

    // Remove window shadow for clean transparent look
    let _: () = msg_send![ns_window, setHasShadow: false];

    log::info!("Overlay NSWindow swizzled to NSPanel (level {})", OVERLAY_WINDOW_LEVEL);
}

/// Apply NSVisualEffectView vibrancy to a window for frosted glass appearance.
///
/// Creates a visual effect view with Sidebar material (matching macOS System
/// Settings) and inserts it behind all subviews of the window's content view.
#[cfg(target_os = "macos")]
pub unsafe fn apply_vibrancy(ns_window: cocoa::base::id) {
    use cocoa::base::nil;
    use cocoa::foundation::NSRect;
    use objc::runtime::Class;
    use objc::{msg_send, sel, sel_impl};

    let content_view: cocoa::base::id = msg_send![ns_window, contentView];
    let frame: NSRect = msg_send![content_view, frame];

    let cls = Class::get("NSVisualEffectView").expect("NSVisualEffectView class not found");
    let vev: cocoa::base::id = msg_send![cls, alloc];
    let vev: cocoa::base::id = msg_send![vev, initWithFrame: frame];

    // Material: Sidebar (7) — frosted glass matching System Settings
    let _: () = msg_send![vev, setMaterial: 7_i64];
    // Blending mode: BehindWindow (0)
    let _: () = msg_send![vev, setBlendingMode: 0_i64];
    // State: FollowsWindowActiveState (0) — dims when window inactive
    let _: () = msg_send![vev, setState: 0_i64];
    // Autoresizing mask: width + height sizable (2 | 16 = 18)
    let _: () = msg_send![vev, setAutoresizingMask: 18_u64];

    // Insert behind all existing subviews (NSWindowBelow = -1, relativeTo: nil)
    let _: () = msg_send![content_view, addSubview: vev positioned: -1_i64 relativeTo: nil];

    log::info!("Applied NSVisualEffectView vibrancy (Sidebar material)");
}

/// Apply transparent titlebar styling — extends content behind titlebar,
/// makes it transparent (glass), and hides title text while keeping traffic lights.
#[cfg(target_os = "macos")]
pub unsafe fn apply_transparent_titlebar(ns_window: cocoa::base::id) {
    use objc::{msg_send, sel, sel_impl};

    // Extend content view behind titlebar (NSWindowStyleMaskFullSizeContentView = 1 << 15)
    let current_mask: u64 = msg_send![ns_window, styleMask];
    let _: () = msg_send![ns_window, setStyleMask: current_mask | (1u64 << 15)];
    // Make titlebar transparent (glass effect via vibrancy)
    let _: () = msg_send![ns_window, setTitlebarAppearsTransparent: true];
    // Hide title text (NSWindowTitleHidden = 1)
    let _: () = msg_send![ns_window, setTitleVisibility: 1_i64];
}

/// Show the overlay window and ensure it stays above all other windows.
///
/// Re-applies the macOS window level and forces the window to the front on
/// every show because macOS may reset window ordering after hide/show cycles.
/// Uses GCD dispatch to main thread because AppKit window operations must
/// happen on the main thread, but this function may be called from background
/// threads (e.g. hotkey handler, rewrite handler).
pub fn show_overlay(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.show();

        #[cfg(target_os = "macos")]
        {
            if let Ok(ns_ptr) = w.ns_window() {
                let ns_ptr_val = ns_ptr as usize;
                // Dispatch window ordering to main thread (AppKit requirement).
                extern "C" fn set_level_and_order(ctx: *mut std::ffi::c_void) {
                    unsafe {
                        use cocoa::appkit::NSWindow;
                        let ns_window = ctx as cocoa::base::id;
                        ns_window.setLevel_(OVERLAY_WINDOW_LEVEL);
                        ns_window.orderFrontRegardless();
                    }
                }
                unsafe {
                    extern "C" {
                        // _dispatch_main_q is the global main queue object.
                        // dispatch_get_main_queue() is a C macro = &_dispatch_main_q.
                        static _dispatch_main_q: u8;
                        fn dispatch_async_f(
                            queue: *const u8,
                            context: *mut std::ffi::c_void,
                            work: extern "C" fn(*mut std::ffi::c_void),
                        );
                    }
                    dispatch_async_f(
                        std::ptr::addr_of!(_dispatch_main_q),
                        ns_ptr_val as *mut std::ffi::c_void,
                        set_level_and_order,
                    );
                }
            }
        }
    }
}

/// Hide the overlay window.
pub fn hide_overlay(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.set_ignore_cursor_events(true);
        let _ = w.hide();
    }
}

/// Enable cursor events on overlay so user can click "Rewrite?" button.
pub fn enable_overlay_interaction(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.set_ignore_cursor_events(false);
    }
}

/// Disable cursor events on overlay (click-through mode).
pub fn disable_overlay_interaction(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        let _ = w.set_ignore_cursor_events(true);
    }
}

/// macOS window level for the settings window — NSFloatingWindowLevel (3).
/// Ensures the settings window appears above all normal application windows.
#[cfg(target_os = "macos")]
pub const SETTINGS_WINDOW_LEVEL: i64 = 3;

/// Toggle the settings window (create on first use).
/// Window uses NSVisualEffectView vibrancy for frosted glass appearance
/// and floats above all other application windows.
pub fn toggle_settings(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        if w.is_visible().unwrap_or(false) {
            let _ = w.hide();
        } else {
            let _ = w.show();
            let _ = w.set_focus();

            // Re-apply floating level on each show (macOS may reset it).
            // Dispatch to main thread for AppKit safety.
            #[cfg(target_os = "macos")]
            {
                if let Ok(ns_ptr) = w.ns_window() {
                    let ns_ptr_val = ns_ptr as usize;
                    extern "C" fn set_settings_level(ctx: *mut std::ffi::c_void) {
                        unsafe {
                            use cocoa::appkit::NSWindow;
                            let ns_window = ctx as cocoa::base::id;
                            ns_window.setLevel_(SETTINGS_WINDOW_LEVEL);
                            ns_window.orderFrontRegardless();
                        }
                    }
                    unsafe {
                        extern "C" {
                            static _dispatch_main_q: u8;
                            fn dispatch_async_f(
                                queue: *const u8,
                                context: *mut std::ffi::c_void,
                                work: extern "C" fn(*mut std::ffi::c_void),
                            );
                        }
                        dispatch_async_f(
                            std::ptr::addr_of!(_dispatch_main_q),
                            ns_ptr_val as *mut std::ffi::c_void,
                            set_settings_level,
                        );
                    }
                }
            }
        }
    } else {
        #[allow(unused_variables)]
        let win = WebviewWindowBuilder::new(
            app,
            "settings",
            WebviewUrl::App("index.html".into()),
        )
        .title("ShhhType Settings")
        .inner_size(700.0, 500.0)
        .resizable(true)
        .transparent(true)
        .center()
        .build();

        #[cfg(target_os = "macos")]
        if let Ok(ref w) = win {
            use cocoa::appkit::NSWindow;
            if let Ok(ns_ptr) = w.ns_window() {
                let ns_window = ns_ptr as cocoa::base::id;
                unsafe {
                    apply_vibrancy(ns_window);
                    apply_transparent_titlebar(ns_window);
                    ns_window.setLevel_(SETTINGS_WINDOW_LEVEL);
                }
            }
        }
    }
}

/// Show the welcome/onboarding window.
/// Window uses NSVisualEffectView vibrancy for frosted glass appearance.
pub fn show_welcome(app: &AppHandle) {
    if app.get_webview_window("welcome").is_some() {
        return;
    }
    #[allow(unused_variables)]
    let win = WebviewWindowBuilder::new(
        app,
        "welcome",
        WebviewUrl::App("index.html".into()),
    )
    .title("Welcome to ShhhType")
    .inner_size(600.0, 520.0)
    .resizable(false)
    .transparent(true)
    .center()
    .build();

    #[cfg(target_os = "macos")]
    if let Ok(ref w) = win {
        if let Ok(ns_ptr) = w.ns_window() {
            let ns_window = ns_ptr as cocoa::base::id;
            unsafe {
                apply_vibrancy(ns_window);
                apply_transparent_titlebar(ns_window);
            }
        }
    }
}
