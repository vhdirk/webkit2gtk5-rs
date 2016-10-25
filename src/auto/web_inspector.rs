// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct WebInspector(Object<ffi::WebKitWebInspector>);

    match fn {
        get_type => || ffi::webkit_web_inspector_get_type(),
    }
}

impl WebInspector {
    pub fn attach(&self) {
        unsafe {
            ffi::webkit_web_inspector_attach(self.to_glib_none().0);
        }
    }

    pub fn close(&self) {
        unsafe {
            ffi::webkit_web_inspector_close(self.to_glib_none().0);
        }
    }

    pub fn detach(&self) {
        unsafe {
            ffi::webkit_web_inspector_detach(self.to_glib_none().0);
        }
    }

    pub fn get_attached_height(&self) -> u32 {
        unsafe {
            ffi::webkit_web_inspector_get_attached_height(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_8")]
    pub fn get_can_attach(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_inspector_get_can_attach(self.to_glib_none().0))
        }
    }

    pub fn get_inspected_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_web_inspector_get_inspected_uri(self.to_glib_none().0))
        }
    }

    //pub fn get_web_view(&self) -> /*Ignored*/Option<WebViewBase> {
    //    unsafe { TODO: call ffi::webkit_web_inspector_get_web_view() }
    //}

    pub fn is_attached(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_inspector_is_attached(self.to_glib_none().0))
        }
    }

    pub fn show(&self) {
        unsafe {
            ffi::webkit_web_inspector_show(self.to_glib_none().0);
        }
    }

    pub fn connect_attach<F: Fn(&WebInspector) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebInspector) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "attach",
                transmute(attach_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_bring_to_front<F: Fn(&WebInspector) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebInspector) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "bring-to-front",
                transmute(bring_to_front_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_closed<F: Fn(&WebInspector) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebInspector) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "closed",
                transmute(closed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_detach<F: Fn(&WebInspector) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebInspector) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "detach",
                transmute(detach_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_open_window<F: Fn(&WebInspector) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&WebInspector) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "open-window",
                transmute(open_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn attach_trampoline(this: *mut ffi::WebKitWebInspector, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebInspector) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn bring_to_front_trampoline(this: *mut ffi::WebKitWebInspector, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebInspector) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn closed_trampoline(this: *mut ffi::WebKitWebInspector, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&WebInspector) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn detach_trampoline(this: *mut ffi::WebKitWebInspector, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebInspector) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn open_window_trampoline(this: *mut ffi::WebKitWebInspector, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&WebInspector) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}
