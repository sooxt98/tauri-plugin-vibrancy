// use cocoa::base::Class;
use block::Block;
use block::ConcreteBlock;
use cocoa::appkit::CGPoint;
use cocoa::appkit::NSColor;
use cocoa::appkit::NSImage;
use cocoa::appkit::NSView;
use cocoa::appkit::NSViewHeightSizable;
use cocoa::appkit::NSViewWidthSizable;
use cocoa::appkit::NSWindowOrderingMode;
use cocoa::appkit::NSWindowStyleMask;
use cocoa::base::nil;
use cocoa::base::BOOL;
use cocoa::base::NO;
use cocoa::base::YES;
// use cocoa::appkit::NSImage;
use cocoa::quartzcore::CALayer;
// use cocoa::quartzcore::CGColor;
use cocoa::base::id;
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};
use core_graphics::base::CGFloat;
use core_graphics::color::CGColor;
use core_graphics::geometry::CGRect;
use core_graphics::geometry::CGSize;
use objc::msg_send;
use objc::rc::StrongPtr;
use objc::runtime::Class;
// use objc_id::ShareId;
// use crate::geometry::Rect;
use std::thread;
use std::time::Duration;

#[repr(C)]
// #[derive(Clone, Copy, Debug, Default)]
pub struct NSEdgeInsets {
    pub bottom: CGFloat,
    pub left: CGFloat,
    pub right: CGFloat,
    pub top: CGFloat,
}

// TODO: https://github.com/servo/core-foundation-rs/issues/164
// TODO: https://stackoverflow.com/questions/32042385/nsvisualeffectview-with-rounded-corners
// TODO: https://stackoverflow.com/a/34973560/6829093
// TODO: https://www.youtube.com/watch?v=ETzXheoGZ34

// TODO: https://github.com/electron/electron/pull/20360/files

pub unsafe fn set_vibrancy(ns_window: id) {
    // let NSVISUALEFFECTVIEW: Option<&'static Class> = Class::get("NSVisualEffectView");

    // let layer = CALayer::new();
    let view: id = msg_send![ns_window, contentView];

    // let view = StrongPtr::new(msg_send![ns_window, contentView]);
    let blurred_view: id = msg_send![class!(NSVisualEffectView), alloc];

    // layer.set_corner_radius(20.0);
    // layer.set_background_color(Some(CGColor::rgb(0.0,0.0,0.0,0.0)));
    // layer.set_masks_to_bounds(true);

    // let layer : id = msg_send![blurred_view, layer];
    // let frame : id = msg_send![blurred_view, frame];
    // let _: () = msg_send![layer, masksToBounds: YES];
    // let _: () = msg_send![layer, cornerRadius: 20.0];
    // let _: () = msg_send![layer, frame: frame];
    // NSView::setLayer(blurred_view, layer);

    // let (ns_view, cursor_state) = new_view(ns_window);
    // let bounds = ns_window.bounds();
    // let bounds = ns_view.bounds();
    // let zero: CGRect = CGRect::new(&CGPoint::new(0.0, 0.0), &CGSize::new(0.0, 0.0));
    // let rect: CGRect = frame.into();

    let bounds = NSView::bounds(view);
    let _: () = msg_send![blurred_view, initWithFrame: bounds];
    // let _: () = msg_send![blurred_view, setMaterial: 2];
    let _: () = msg_send![
        blurred_view,
        setAutoresizingMask: NSViewWidthSizable | NSViewHeightSizable
    ];

    // let _: () = msg_send![*blurred_view, initWithFrame: zero];

    // let _: () = msg_send![*blurred_view, initWithFrame: bounds];

    // // Blur external background
    // let _: () = msg_send![blurred_view, setBlendingMode:0];
    // // Blur when the window is active
    let _: () = msg_send![blurred_view, setState:1];
    // // Autoresizing: Adapt to superview
    // let _: () = msg_send![blurred_view, setAutoresizingMask: 0x3e];

    // let _: () = msg_send![ns_window, setContentView: blurred_view];

    let _: () = msg_send![view, addSubview:blurred_view.clone() positioned: NSWindowOrderingMode::NSWindowBelow relativeTo: 0];
    // let _: () = msg_send![ns_window,setStyleMask: NSWindowStyleMask::NSFullSizeContentViewWindowMask];

    // let _: () = msg_send![ns_window, setOpaque:NO];

    // let layer : id = msg_send![view, layer];
    // // let frame : id = msg_send![view, frame];
    // let _: () = msg_send![view, setWantsLayer: YES];
    // let _: () = msg_send![layer, masksToBounds: YES];
    // let _: () = msg_send![layer, cornerRadius: 20.0];
    // // let _: () = msg_send![layer, frame: frame];

    // thread::sleep(Duration::from_secs(5));
    // let view : id = msg_send![ns_window, contentView];

    // let _: () = msg_send![view, addSubview:blurred_view.clone() positioned: NSWindowOrderingMode::NSWindowAbove relativeTo: 0];

    // NSView::addSubview_(view, *blurred_view);

    // let _: () = msg_send![*ns_view, addSubview: blurred_view.clone() positioned:    NSWindowOrderingMode::NSWindowBelow relativeTo: 0];

    // ns_visual_effect_view = Some(blurred_view);

    // CGFloat::
    let radius: CGFloat = 12.0;
    // let dimension: CGFloat = 2.0 * radius + 1.0;
    let edge_inserts = NSEdgeInsets {
        bottom: radius,
        left: radius,
        right: radius,
        top: radius,
    };
    let ns_size: NSSize = NSSize::new(bounds.size.width, bounds.size.height);
    // let ns_size: NSSize = NSSize::new(dimension, dimension);
    let ns_image: id = msg_send![class!(NSImage), alloc];
    // let ns_image : id = NSImage::alloc(nil);

    // unsafe fn sum(ns_rect: NSRect) -> BOOL {
    //     YES
    // }

    let mut block = ConcreteBlock::new(|ns_rect: NSRect| {
        // let radius = ns_rect.size.height / 12.0;
        let path: id = msg_send![class!(NSBezierPath), bezierPathWithRoundedRect: ns_rect xRadius: radius yRadius: radius];
        // let black = NSColor::colorWithRed_green_blue_alpha_(nil, 0.0, 0.0, 0.0, 1.0);
        // let _: () = msg_send![black, set];
        let _: () = msg_send![path, fill];
        YES
    });

    let my_speed_ptr: *mut Block<(NSRect,), BOOL> = &mut *block;
    const NSImageResizingModeStretch: u32 = 1;

    // ! Crashed
    // NSImage::initWithSize_flipped_drawingHandler_(ns_image, ns_size, NO, &mut *block);

    // let icon_name = NSString::alloc(nil).init_str("what");
    // NSImage::imageNamed_(ns_image, icon_name);
    // let _: () = msg_send![class!(NSImage), imageNamed: icon_name];

    // let _: () = msg_send![ns_image, imageWithSize:ns_size flipped: NO drawingHandler: my_speed_ptr];
    let mask_image_no_release: id =
        msg_send![class!(NSImage), imageWithSize:ns_size flipped: NO drawingHandler: &mut *block];
    let mask_image: id = msg_send![mask_image_no_release, autorelease];

    let _: () = msg_send![mask_image, setCapInsets: edge_inserts];
    let _: () = msg_send![mask_image, setResizingMode: NSImageResizingModeStretch];
    let _: () = msg_send![blurred_view, setMaskImage: mask_image];
    // let _: () = msg_send![ns_window, setCornerMask:mask_image];
}
