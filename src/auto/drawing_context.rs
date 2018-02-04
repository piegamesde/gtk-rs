// This file was generated by gir (0f1d1c1) from gir-files (77d1f70)
// DO NOT EDIT

#[cfg(any(feature = "v3_22", feature = "dox"))]
use Window;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use cairo;
use ffi;
use glib;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DrawingContext(Object<ffi::GdkDrawingContext, ffi::GdkDrawingContextClass>);

    match fn {
        get_type => || ffi::gdk_drawing_context_get_type(),
    }
}

pub trait DrawingContextExt {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_cairo_context(&self) -> Option<cairo::Context>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_clip(&self) -> Option<cairo::Region>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_window(&self) -> Option<Window>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_valid(&self) -> bool;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_clip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DrawingContext> + IsA<glib::object::Object>> DrawingContextExt for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_cairo_context(&self) -> Option<cairo::Context> {
        unsafe {
            from_glib_none(ffi::gdk_drawing_context_get_cairo_context(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_clip(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_full(ffi::gdk_drawing_context_get_clip(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_drawing_context_get_window(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_valid(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_drawing_context_is_valid(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_clip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::clip",
                transmute(notify_clip_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window",
                transmute(notify_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_clip_trampoline<P>(this: *mut ffi::GdkDrawingContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DrawingContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DrawingContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_window_trampoline<P>(this: *mut ffi::GdkDrawingContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DrawingContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DrawingContext::from_glib_borrow(this).downcast_unchecked())
}
