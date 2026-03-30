use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// macOS window level for the overlay panel — above NSScreenSaverWindowLevel (1000).
/// Level 1001 ensures the overlay renders above ALL other windows including
/// full-screen terminal apps (iTerm2, Kitty, Terminal.app), floating panels,
/// modal dialogs, and screensaver-level windows. Combined with NSPanel swizzle,
/// it also appears over full-screen Spaces.
#[cfg(target_os = "macos")]
pub const OVERLAY_WINDOW_LEVEL: i64 = 1001;

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

/// Try to get the text cursor (caret) position via the macOS Accessibility API.
///
/// Returns (x, y) in Cocoa screen coordinates (Y=0 at bottom) if successful.
/// Requires Accessibility permission (already granted for text injection).
#[cfg(target_os = "macos")]
unsafe fn text_cursor_position() -> Option<(f64, f64)> {
    use core_foundation::base::{CFRelease, CFTypeRef, TCFType};
    use core_foundation::string::CFString;
    use cocoa::appkit::NSScreen;
    use cocoa::foundation::NSArray;
    use objc::{msg_send, sel, sel_impl};

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXUIElementCreateSystemWide() -> CFTypeRef;
        fn AXUIElementCopyAttributeValue(
            element: CFTypeRef,
            attribute: CFTypeRef,
            value: *mut CFTypeRef,
        ) -> i32;
        fn AXUIElementCopyParameterizedAttributeValue(
            element: CFTypeRef,
            attr: CFTypeRef,
            param: CFTypeRef,
            value: *mut CFTypeRef,
        ) -> i32;
        fn AXValueGetValue(value: CFTypeRef, value_type: u32, value_ptr: *mut std::ffi::c_void) -> bool;
    }

    // kAXValueTypeCGRect = 4
    const K_AX_VALUE_CG_RECT: u32 = 4;

    let system = AXUIElementCreateSystemWide();
    if system.is_null() {
        return None;
    }

    // Get the focused application
    let attr_focused_app = CFString::new("AXFocusedApplication");
    let mut focused_app: CFTypeRef = std::ptr::null();
    let err = AXUIElementCopyAttributeValue(system, attr_focused_app.as_CFTypeRef(), &mut focused_app);
    CFRelease(system);
    if err != 0 || focused_app.is_null() {
        log::debug!("text_cursor_position: no focused app (err={})", err);
        return None;
    }

    // Get the focused UI element within that app
    let attr_focused_elem = CFString::new("AXFocusedUIElement");
    let mut focused_elem: CFTypeRef = std::ptr::null();
    let err = AXUIElementCopyAttributeValue(focused_app, attr_focused_elem.as_CFTypeRef(), &mut focused_elem);
    CFRelease(focused_app);
    if err != 0 || focused_elem.is_null() {
        log::debug!("text_cursor_position: no focused element (err={})", err);
        return None;
    }

    // Get the selected text range (tells us where the caret is)
    let attr_selected_range = CFString::new("AXSelectedTextRange");
    let mut range_value: CFTypeRef = std::ptr::null();
    let err = AXUIElementCopyAttributeValue(focused_elem, attr_selected_range.as_CFTypeRef(), &mut range_value);
    if err != 0 || range_value.is_null() {
        log::debug!("text_cursor_position: no selected text range (err={})", err);
        CFRelease(focused_elem);
        return None;
    }

    // Get the screen bounds for that text range (caret position)
    let attr_bounds = CFString::new("AXBoundsForRange");
    let mut bounds_value: CFTypeRef = std::ptr::null();
    let err = AXUIElementCopyParameterizedAttributeValue(
        focused_elem,
        attr_bounds.as_CFTypeRef(),
        range_value,
        &mut bounds_value,
    );
    CFRelease(range_value);
    CFRelease(focused_elem);
    if err != 0 || bounds_value.is_null() {
        log::debug!("text_cursor_position: AXBoundsForRange failed (err={})", err);
        return None;
    }

    // AXBoundsForRange returns a CGRect AXValue (type 4)
    // CGRect in CG coords: Y=0 at top-left of primary screen
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    struct CGRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

    let mut rect = CGRect::default();
    let got_rect = AXValueGetValue(
        bounds_value,
        K_AX_VALUE_CG_RECT,
        &mut rect as *mut _ as *mut std::ffi::c_void,
    );
    CFRelease(bounds_value);

    if !got_rect {
        log::debug!("text_cursor_position: failed to extract CGRect from AXValue");
        return None;
    }

    // Convert from CG coords (Y=0 at top) to Cocoa coords (Y=0 at bottom)
    let screens: cocoa::base::id = NSScreen::screens(cocoa::base::nil);
    if screens.count() == 0 {
        return None;
    }
    let primary: cocoa::base::id = msg_send![screens, objectAtIndex: 0u64];
    let primary_frame: cocoa::foundation::NSRect = msg_send![primary, frame];
    let primary_height = primary_frame.size.height;

    // CG origin is top-left of the caret rect; convert to Cocoa bottom-left
    let cocoa_x = rect.x;
    let cocoa_y = primary_height - rect.y - rect.height;

    log::info!(
        "Text cursor at CG({:.0},{:.0}) size({:.0},{:.0}) → Cocoa({:.0},{:.0})",
        rect.x, rect.y, rect.width, rect.height, cocoa_x, cocoa_y
    );

    Some((cocoa_x, cocoa_y))
}

/// Compute the overlay origin positioned near the text cursor (caret).
///
/// `text_cursor` is the pre-captured caret position (Cocoa coords) from
/// before GCD dispatch. Falls back to mouse pointer if None.
///
/// Centers the overlay horizontally on the anchor and places it above with
/// a 20px gap. Falls back to below if near the top of the screen.
/// Clamps to the visible frame of the screen containing the anchor.
#[cfg(target_os = "macos")]
unsafe fn cursor_overlay_origin(width: f64, height: f64, text_cursor: Option<(f64, f64)>) -> (f64, f64) {
    use cocoa::appkit::{NSEvent, NSScreen};
    use cocoa::foundation::{NSArray, NSPoint, NSRect};
    use objc::{msg_send, sel, sel_impl};

    // Use pre-captured text cursor, fall back to mouse pointer
    let anchor: NSPoint = if let Some((cx, cy)) = text_cursor {
        log::info!("Overlay positioned at text cursor ({:.0}, {:.0})", cx, cy);
        NSPoint::new(cx, cy)
    } else {
        log::debug!("Text cursor not available, falling back to mouse position");
        NSEvent::mouseLocation(cocoa::base::nil)
    };

    let screens: cocoa::base::id = NSScreen::screens(cocoa::base::nil);
    let count = screens.count();
    let gap = 20.0;

    // Find the screen containing the anchor
    let mut visible: NSRect = cocoa::foundation::NSRect::new(
        cocoa::foundation::NSPoint::new(0.0, 0.0),
        cocoa::foundation::NSSize::new(1920.0, 1080.0),
    );
    for i in 0..count {
        let screen: cocoa::base::id = msg_send![screens, objectAtIndex: i];
        let frame: NSRect = msg_send![screen, frame];
        if anchor.x >= frame.origin.x
            && anchor.x < frame.origin.x + frame.size.width
            && anchor.y >= frame.origin.y
            && anchor.y < frame.origin.y + frame.size.height
        {
            visible = msg_send![screen, visibleFrame];
            break;
        }
    }

    // Center horizontally on anchor
    let mut x = anchor.x - width / 2.0;
    // Place above anchor (Cocoa Y grows upward)
    let mut y = anchor.y + gap;

    // If too close to top, place below anchor instead
    if y + height > visible.origin.y + visible.size.height {
        y = anchor.y - height - gap;
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
    /// Pre-captured text cursor position (Cocoa coords) from calling thread.
    /// Must be captured before GCD dispatch because the focused app changes
    /// once the overlay appears.
    text_cursor: Option<(f64, f64)>,
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
                // Capture text cursor BEFORE dispatching to main thread.
                // Once GCD dispatch runs, our overlay becomes visible and
                // the focused app/element changes — accessibility queries
                // would return our own app instead of the user's text field.
                let text_cursor = if inline {
                    unsafe { text_cursor_position() }
                } else {
                    None
                };
                let ctx = Box::new(OverlayShowCtx {
                    ns_ptr: ns_ptr as usize,
                    inline_pos: inline,
                    text_cursor,
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
                                overlay_ctx.text_cursor,
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
