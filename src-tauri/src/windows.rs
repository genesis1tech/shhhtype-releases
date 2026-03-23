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

/// Compute the overlay origin positioned near the mouse cursor.
///
/// Centers the overlay horizontally on the cursor and places it above the cursor
/// with a 20px gap. Falls back to below cursor if near the top of the screen.
/// Clamps to the visible frame of the screen containing the cursor.
#[cfg(target_os = "macos")]
unsafe fn cursor_overlay_origin(width: f64, height: f64) -> (f64, f64) {
    use cocoa::appkit::{NSEvent, NSScreen};
    use cocoa::foundation::{NSArray, NSPoint, NSRect};
    use objc::{msg_send, sel, sel_impl};

    // mouseLocation returns Cocoa coords (Y=0 at bottom of primary screen)
    let cursor: NSPoint = NSEvent::mouseLocation(cocoa::base::nil);
    let screens: cocoa::base::id = NSScreen::screens(cocoa::base::nil);
    let count = screens.count();
    let gap = 20.0;

    // Find the screen containing the cursor
    let mut visible: NSRect = cocoa::foundation::NSRect::new(
        cocoa::foundation::NSPoint::new(0.0, 0.0),
        cocoa::foundation::NSSize::new(1920.0, 1080.0),
    );
    for i in 0..count {
        let screen: cocoa::base::id = msg_send![screens, objectAtIndex: i];
        let frame: NSRect = msg_send![screen, frame];
        if cursor.x >= frame.origin.x
            && cursor.x < frame.origin.x + frame.size.width
            && cursor.y >= frame.origin.y
            && cursor.y < frame.origin.y + frame.size.height
        {
            visible = msg_send![screen, visibleFrame];
            break;
        }
    }

    // Center horizontally on cursor
    let mut x = cursor.x - width / 2.0;
    // Place above cursor (Cocoa Y grows upward)
    let mut y = cursor.y + gap;

    // If too close to top, place below cursor instead
    if y + height > visible.origin.y + visible.size.height {
        y = cursor.y - height - gap;
    }

    // Clamp to visible frame
    if x < visible.origin.x {
        x = visible.origin.x;
    }
    if x + width > visible.origin.x + visible.size.width {
        x = visible.origin.x + visible.size.width - width;
    }
    if y < visible.origin.y {
        y = visible.origin.y;
    }
    if y + height > visible.origin.y + visible.size.height {
        y = visible.origin.y + visible.size.height - height;
    }

    (x, y)
}

/// Context passed through GCD dispatch for show_overlay.
#[cfg(target_os = "macos")]
struct OverlayShowCtx {
    ns_ptr: usize,
    inline_pos: bool,
}

/// Show the overlay window and ensure it stays above all other windows.
///
/// When `inline` is true, positions the overlay near the mouse cursor.
/// Otherwise, positions at top-center of the cursor's current screen.
///
/// Re-applies the macOS window level and forces the window to the front on
/// every show because macOS may reset window ordering after hide/show cycles.
/// Uses GCD dispatch to main thread because AppKit window operations must
/// happen on the main thread, but this function may be called from background
/// threads (e.g. hotkey handler, rewrite handler).
pub fn show_overlay(app: &AppHandle, inline: bool) {
    if let Some(w) = app.get_webview_window("overlay") {
        #[cfg(target_os = "macos")]
        {
            if let Ok(ns_ptr) = w.ns_window() {
                let ctx = Box::new(OverlayShowCtx {
                    ns_ptr: ns_ptr as usize,
                    inline_pos: inline,
                });
                let ctx_ptr = Box::into_raw(ctx) as *mut std::ffi::c_void;
                // Dispatch window ordering to main thread (AppKit requirement).
                // Use orderFrontRegardless + setLevel_ directly instead of Tauri's
                // show() which calls makeKeyAndOrderFront: and steals focus from the
                // user's active app — especially on first show after launch.
                extern "C" fn show_without_focus(ctx: *mut std::ffi::c_void) {
                    unsafe {
                        use cocoa::appkit::NSWindow;
                        use cocoa::foundation::NSPoint;
                        use objc::runtime::Class;
                        use objc::{msg_send, sel, sel_impl};
                        let overlay_ctx = Box::from_raw(ctx as *mut OverlayShowCtx);
                        let ns_window = overlay_ctx.ns_ptr as cocoa::base::id;

                        if overlay_ctx.inline_pos {
                            let frame: cocoa::foundation::NSRect =
                                msg_send![ns_window, frame];
                            let (x, y) = cursor_overlay_origin(
                                frame.size.width,
                                frame.size.height,
                            );
                            let origin = NSPoint::new(x, y);
                            let _: () = msg_send![ns_window, setFrameOrigin: origin];
                        } else {
                            // Top-center of the screen containing the cursor
                            use cocoa::appkit::{NSEvent, NSScreen};
                            use cocoa::foundation::NSArray;

                            let cursor: cocoa::foundation::NSPoint =
                                NSEvent::mouseLocation(cocoa::base::nil);
                            let screens: cocoa::base::id =
                                NSScreen::screens(cocoa::base::nil);
                            let count = screens.count();
                            let first_screen: cocoa::base::id =
                                msg_send![screens, objectAtIndex: 0u64];
                            let mut screen_frame: cocoa::foundation::NSRect =
                                msg_send![first_screen, visibleFrame];
                            for i in 0..count {
                                let screen: cocoa::base::id =
                                    msg_send![screens, objectAtIndex: i];
                                let frame: cocoa::foundation::NSRect =
                                    msg_send![screen, frame];
                                if cursor.x >= frame.origin.x
                                    && cursor.x
                                        < frame.origin.x + frame.size.width
                                    && cursor.y >= frame.origin.y
                                    && cursor.y
                                        < frame.origin.y + frame.size.height
                                {
                                    screen_frame = msg_send![screen, visibleFrame];
                                    break;
                                }
                            }
                            let win_frame: cocoa::foundation::NSRect =
                                msg_send![ns_window, frame];
                            let x = screen_frame.origin.x
                                + (screen_frame.size.width - win_frame.size.width)
                                    / 2.0;
                            // Top of visible frame (Cocoa Y grows upward)
                            let y = screen_frame.origin.y
                                + screen_frame.size.height
                                - win_frame.size.height
                                - 20.0;
                            let origin = NSPoint::new(x, y);
                            let _: () = msg_send![ns_window, setFrameOrigin: origin];
                        }

                        ns_window.setLevel_(OVERLAY_WINDOW_LEVEL);
                        // setIsVisible shows the window without making it key
                        let _: () = msg_send![ns_window, setIsVisible: true];
                        // orderFrontRegardless brings it to front without activating
                        ns_window.orderFrontRegardless();
                        // Force-deactivate our app so the previously focused app stays active
                        let ns_app: cocoa::base::id = msg_send![
                            Class::get("NSApplication").unwrap(),
                            sharedApplication
                        ];
                        let _: () = msg_send![ns_app, deactivate];
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
                        ctx_ptr,
                        show_without_focus,
                    );
                }
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = w.show();
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
