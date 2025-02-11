// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLElement;
use crate::DOMNode;
use crate::DOMObject;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLBaseFontElement")]
    pub struct DOMHTMLBaseFontElement(Object<ffi::WebKitDOMHTMLBaseFontElement, ffi::WebKitDOMHTMLBaseFontElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_base_font_element_get_type(),
    }
}

pub const NONE_DOMHTML_BASE_FONT_ELEMENT: Option<&DOMHTMLBaseFontElement> = None;

pub trait DOMHTMLBaseFontElementExt: 'static {
    #[cfg_attr(feature = "v2_12", deprecated = "Since 2.12")]
    #[doc(alias = "webkit_dom_html_base_font_element_get_color")]
    #[doc(alias = "get_color")]
    fn color(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_12", deprecated = "Since 2.12")]
    #[doc(alias = "webkit_dom_html_base_font_element_get_face")]
    #[doc(alias = "get_face")]
    fn face(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_12", deprecated = "Since 2.12")]
    #[doc(alias = "webkit_dom_html_base_font_element_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_12", deprecated = "Since 2.12")]
    #[doc(alias = "webkit_dom_html_base_font_element_set_color")]
    fn set_color(&self, value: &str);

    #[cfg_attr(feature = "v2_12", deprecated = "Since 2.12")]
    #[doc(alias = "webkit_dom_html_base_font_element_set_face")]
    fn set_face(&self, value: &str);

    #[cfg_attr(feature = "v2_12", deprecated = "Since 2.12")]
    #[doc(alias = "webkit_dom_html_base_font_element_set_size")]
    fn set_size(&self, value: libc::c_long);
}

impl<O: IsA<DOMHTMLBaseFontElement>> DOMHTMLBaseFontElementExt for O {
    fn color(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_font_element_get_color(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn face(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_font_element_get_face(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn size(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_base_font_element_get_size(self.as_ref().to_glib_none().0) }
    }

    fn set_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_font_element_set_color(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_face(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_font_element_set_face(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_size(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_base_font_element_set_size(self.as_ref().to_glib_none().0, value);
        }
    }
}

impl fmt::Display for DOMHTMLBaseFontElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLBaseFontElement")
    }
}
