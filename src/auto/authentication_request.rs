// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
use AuthenticationScheme;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use Credential;
use ffi;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct AuthenticationRequest(Object<ffi::WebKitAuthenticationRequest, ffi::WebKitAuthenticationRequestClass, AuthenticationRequestClass>);

    match fn {
        get_type => || ffi::webkit_authentication_request_get_type(),
    }
}

pub const NONE_AUTHENTICATION_REQUEST: Option<&AuthenticationRequest> = None;

pub trait AuthenticationRequestExt: 'static {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn can_save_credentials(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn cancel(&self);

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_host(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_port(&self) -> u32;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_proposed_credential(&self) -> Option<Credential>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_realm(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_scheme(&self) -> AuthenticationScheme;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_for_proxy(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_retry(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AuthenticationRequest>> AuthenticationRequestExt for O {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn can_save_credentials(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_can_save_credentials(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn cancel(&self) {
        unsafe {
            ffi::webkit_authentication_request_cancel(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_host(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_host(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_port(&self) -> u32 {
        unsafe {
            ffi::webkit_authentication_request_get_port(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_proposed_credential(&self) -> Option<Credential> {
        unsafe {
            from_glib_full(ffi::webkit_authentication_request_get_proposed_credential(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_realm(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_realm(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_scheme(&self) -> AuthenticationScheme {
        unsafe {
            from_glib(ffi::webkit_authentication_request_get_scheme(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_for_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_for_proxy(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_retry(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_retry(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"cancelled\0".as_ptr() as *const _,
                transmute(cancelled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_2", feature = "dox"))]
unsafe extern "C" fn cancelled_trampoline<P>(this: *mut ffi::WebKitAuthenticationRequest, f: glib_ffi::gpointer)
where P: IsA<AuthenticationRequest> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AuthenticationRequest::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for AuthenticationRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AuthenticationRequest")
    }
}
