// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLElement;
use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLAreaElement")]
    pub struct DOMHTMLAreaElement(Object<ffi::WebKitDOMHTMLAreaElement, ffi::WebKitDOMHTMLAreaElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_area_element_get_type(),
    }
}

pub const NONE_DOMHTML_AREA_ELEMENT: Option<&DOMHTMLAreaElement> = None;

pub trait DOMHTMLAreaElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_alt")]
    #[doc(alias = "get_alt")]
    fn alt(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_coords")]
    #[doc(alias = "get_coords")]
    fn coords(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_hash")]
    #[doc(alias = "get_hash")]
    fn hash(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_host")]
    #[doc(alias = "get_host")]
    fn host(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_hostname")]
    #[doc(alias = "get_hostname")]
    fn hostname(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_href")]
    #[doc(alias = "get_href")]
    fn href(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_no_href")]
    #[doc(alias = "get_no_href")]
    fn is_no_href(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_pathname")]
    #[doc(alias = "get_pathname")]
    fn pathname(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_port")]
    #[doc(alias = "get_port")]
    fn port(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_protocol")]
    #[doc(alias = "get_protocol")]
    fn protocol(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_search")]
    #[doc(alias = "get_search")]
    fn search(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_shape")]
    #[doc(alias = "get_shape")]
    fn shape(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_get_target")]
    #[doc(alias = "get_target")]
    fn target(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_set_alt")]
    fn set_alt(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_set_coords")]
    fn set_coords(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_set_hash")]
    fn set_hash(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_area_element_set_host")]
    fn set_host(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_area_element_set_hostname")]
    fn set_hostname(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_set_href")]
    fn set_href(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_set_no_href")]
    fn set_no_href(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_area_element_set_pathname")]
    fn set_pathname(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_area_element_set_port")]
    fn set_port(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_area_element_set_protocol")]
    fn set_protocol(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_area_element_set_search")]
    fn set_search(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_set_shape")]
    fn set_shape(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_area_element_set_target")]
    fn set_target(&self, value: &str);

    fn set_property_host(&self, host: Option<&str>);

    fn set_property_hostname(&self, hostname: Option<&str>);

    fn set_property_pathname(&self, pathname: Option<&str>);

    fn set_property_port(&self, port: Option<&str>);

    fn set_property_protocol(&self, protocol: Option<&str>);

    fn set_property_search(&self, search: Option<&str>);

    #[doc(alias = "alt")]
    fn connect_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "coords")]
    fn connect_coords_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "hash")]
    fn connect_hash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "host")]
    fn connect_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "hostname")]
    fn connect_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "href")]
    fn connect_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "no-href")]
    fn connect_no_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "pathname")]
    fn connect_pathname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "port")]
    fn connect_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "protocol")]
    fn connect_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "search")]
    fn connect_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "shape")]
    fn connect_shape_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "target")]
    fn connect_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLAreaElement>> DOMHTMLAreaElementExt for O {
    fn alt(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_alt(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn coords(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_coords(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hash(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_hash(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn host(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_host(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hostname(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_hostname(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn href(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_href(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_no_href(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_area_element_get_no_href(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pathname(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_pathname(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn port(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_port(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn protocol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn search(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_search(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shape(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_shape(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn target(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_alt(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_alt(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_coords(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_coords(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_hash(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_hash(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_host(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_host(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_hostname(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_hostname(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_href(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_href(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_no_href(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_no_href(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_pathname(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_pathname(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_port(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_port(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_protocol(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_protocol(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_search(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_search(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_shape(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_shape(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_target(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_property_host(&self, host: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"host\0".as_ptr() as *const _,
                host.to_value().to_glib_none().0,
            );
        }
    }

    fn set_property_hostname(&self, hostname: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"hostname\0".as_ptr() as *const _,
                hostname.to_value().to_glib_none().0,
            );
        }
    }

    fn set_property_pathname(&self, pathname: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"pathname\0".as_ptr() as *const _,
                pathname.to_value().to_glib_none().0,
            );
        }
    }

    fn set_property_port(&self, port: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"port\0".as_ptr() as *const _,
                port.to_value().to_glib_none().0,
            );
        }
    }

    fn set_property_protocol(&self, protocol: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"protocol\0".as_ptr() as *const _,
                protocol.to_value().to_glib_none().0,
            );
        }
    }

    fn set_property_search(&self, search: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"search\0".as_ptr() as *const _,
                search.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alt_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::alt\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alt_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_coords_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_coords_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::coords\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_coords_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_hash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hash_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hash\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hash_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_host_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::host\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_host_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hostname_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hostname\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hostname_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_href_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::href\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_href_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_no_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_no_href_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::no-href\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_no_href_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pathname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pathname_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pathname\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pathname_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_port_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::port\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_port_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_protocol_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::protocol\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_protocol_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_shape_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shape_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shape\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shape_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_target_trampoline<
            P: IsA<DOMHTMLAreaElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLAreaElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLAreaElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::target\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLAreaElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLAreaElement")
    }
}
