// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLElement;
use crate::DOMHTMLFormElement;
use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLFieldSetElement")]
    pub struct DOMHTMLFieldSetElement(Object<ffi::WebKitDOMHTMLFieldSetElement, ffi::WebKitDOMHTMLFieldSetElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_field_set_element_get_type(),
    }
}

pub const NONE_DOMHTML_FIELD_SET_ELEMENT: Option<&DOMHTMLFieldSetElement> = None;

pub trait DOMHTMLFieldSetElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_field_set_element_get_form")]
    #[doc(alias = "get_form")]
    fn form(&self) -> Option<DOMHTMLFormElement>;

    #[doc(alias = "form")]
    fn connect_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFieldSetElement>> DOMHTMLFieldSetElementExt for O {
    fn form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_field_set_element_get_form(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_form_trampoline<
            P: IsA<DOMHTMLFieldSetElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFieldSetElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFieldSetElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::form\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_form_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLFieldSetElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLFieldSetElement")
    }
}
