// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::GeolocationPosition;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitGeolocationManager")]
    pub struct GeolocationManager(Object<ffi::WebKitGeolocationManager, ffi::WebKitGeolocationManagerClass>);

    match fn {
        type_ => || ffi::webkit_geolocation_manager_get_type(),
    }
}

pub const NONE_GEOLOCATION_MANAGER: Option<&GeolocationManager> = None;

pub trait GeolocationManagerExt: 'static {
    #[doc(alias = "webkit_geolocation_manager_failed")]
    fn failed(&self, error_message: &str);

    #[doc(alias = "webkit_geolocation_manager_get_enable_high_accuracy")]
    #[doc(alias = "get_enable_high_accuracy")]
    fn enables_high_accuracy(&self) -> bool;

    #[doc(alias = "webkit_geolocation_manager_update_position")]
    fn update_position(&self, position: &mut GeolocationPosition);

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "start")]
    fn connect_start<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "stop")]
    fn connect_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "enable-high-accuracy")]
    fn connect_enable_high_accuracy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GeolocationManager>> GeolocationManagerExt for O {
    fn failed(&self, error_message: &str) {
        unsafe {
            ffi::webkit_geolocation_manager_failed(self.as_ref().to_glib_none().0, error_message.to_glib_none().0);
        }
    }

    fn enables_high_accuracy(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_geolocation_manager_get_enable_high_accuracy(self.as_ref().to_glib_none().0))
        }
    }

    fn update_position(&self, position: &mut GeolocationPosition) {
        unsafe {
            ffi::webkit_geolocation_manager_update_position(self.as_ref().to_glib_none().0, position.to_glib_none_mut().0);
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn connect_start<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn start_trampoline<P: IsA<GeolocationManager>, F: Fn(&P) -> bool + 'static>(this: *mut ffi::WebKitGeolocationManager, f: glib::ffi::gpointer) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(GeolocationManager::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(start_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn connect_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_trampoline<P: IsA<GeolocationManager>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitGeolocationManager, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(GeolocationManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"stop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(stop_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn connect_enable_high_accuracy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_high_accuracy_trampoline<P: IsA<GeolocationManager>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitGeolocationManager, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(GeolocationManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::enable-high-accuracy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_enable_high_accuracy_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for GeolocationManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GeolocationManager")
    }
}
