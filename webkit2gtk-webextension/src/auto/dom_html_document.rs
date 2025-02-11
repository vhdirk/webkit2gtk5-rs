// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMDocument;
use crate::DOMEventTarget;
use crate::DOMHTMLCollection;
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
    #[doc(alias = "WebKitDOMHTMLDocument")]
    pub struct DOMHTMLDocument(Object<ffi::WebKitDOMHTMLDocument, ffi::WebKitDOMHTMLDocumentClass>) @extends DOMDocument, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_document_get_type(),
    }
}

pub const NONE_DOMHTML_DOCUMENT: Option<&DOMHTMLDocument> = None;

pub trait DOMHTMLDocumentExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_capture_events")]
    fn capture_events(&self);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_clear")]
    fn clear(&self);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_close")]
    fn close(&self);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_get_alink_color")]
    #[doc(alias = "get_alink_color")]
    fn alink_color(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_get_bg_color")]
    #[doc(alias = "get_bg_color")]
    fn bg_color(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    #[doc(alias = "webkit_dom_html_document_get_compat_mode")]
    #[doc(alias = "get_compat_mode")]
    fn compat_mode(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    #[doc(alias = "webkit_dom_html_document_get_design_mode")]
    #[doc(alias = "get_design_mode")]
    fn design_mode(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(not(feature = "v2_16"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_16"))))]
    #[doc(alias = "webkit_dom_html_document_get_dir")]
    #[doc(alias = "get_dir")]
    fn dir(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    #[doc(alias = "webkit_dom_html_document_get_embeds")]
    #[doc(alias = "get_embeds")]
    fn embeds(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_get_fg_color")]
    #[doc(alias = "get_fg_color")]
    fn fg_color(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_get_height")]
    #[doc(alias = "get_height")]
    fn height(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_get_link_color")]
    #[doc(alias = "get_link_color")]
    fn link_color(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    #[doc(alias = "webkit_dom_html_document_get_plugins")]
    #[doc(alias = "get_plugins")]
    fn plugins(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    #[doc(alias = "webkit_dom_html_document_get_scripts")]
    #[doc(alias = "get_scripts")]
    fn scripts(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_get_vlink_color")]
    #[doc(alias = "get_vlink_color")]
    fn vlink_color(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_get_width")]
    #[doc(alias = "get_width")]
    fn width(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_release_events")]
    fn release_events(&self);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_set_alink_color")]
    fn set_alink_color(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_set_bg_color")]
    fn set_bg_color(&self, value: &str);

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    #[doc(alias = "webkit_dom_html_document_set_design_mode")]
    fn set_design_mode(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(not(feature = "v2_16"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_16"))))]
    #[doc(alias = "webkit_dom_html_document_set_dir")]
    fn set_dir(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_set_fg_color")]
    fn set_fg_color(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_set_link_color")]
    fn set_link_color(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_document_set_vlink_color")]
    fn set_vlink_color(&self, value: &str);

    #[doc(alias = "alink-color")]
    fn connect_alink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "bg-color")]
    fn connect_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "dir")]
    fn connect_dir_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "fg-color")]
    fn connect_fg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "height")]
    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "link-color")]
    fn connect_link_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "vlink-color")]
    fn connect_vlink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "width")]
    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLDocument>> DOMHTMLDocumentExt for O {
    fn capture_events(&self) {
        unsafe {
            ffi::webkit_dom_html_document_capture_events(self.as_ref().to_glib_none().0);
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::webkit_dom_html_document_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn close(&self) {
        unsafe {
            ffi::webkit_dom_html_document_close(self.as_ref().to_glib_none().0);
        }
    }

    fn alink_color(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_alink_color(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn bg_color(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_bg_color(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    fn compat_mode(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_compat_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    fn design_mode(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_design_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(not(feature = "v2_16"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_16"))))]
    fn dir(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_dir(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    fn embeds(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_embeds(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn fg_color(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_fg_color(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn height(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_document_get_height(self.as_ref().to_glib_none().0) }
    }

    fn link_color(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_link_color(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    fn plugins(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_plugins(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    fn scripts(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_scripts(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn vlink_color(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_vlink_color(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn width(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_document_get_width(self.as_ref().to_glib_none().0) }
    }

    fn release_events(&self) {
        unsafe {
            ffi::webkit_dom_html_document_release_events(self.as_ref().to_glib_none().0);
        }
    }

    fn set_alink_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_alink_color(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_bg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_bg_color(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_14"))))]
    fn set_design_mode(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_design_mode(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg(any(not(feature = "v2_16"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_16"))))]
    fn set_dir(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_dir(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_fg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_fg_color(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_link_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_link_color(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_vlink_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_vlink_color(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn connect_alink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alink_color_trampoline<
            P: IsA<DOMHTMLDocument>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLDocument,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLDocument::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::alink-color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alink_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bg_color_trampoline<
            P: IsA<DOMHTMLDocument>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLDocument,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLDocument::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bg-color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bg_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_dir_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dir_trampoline<P: IsA<DOMHTMLDocument>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMHTMLDocument,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLDocument::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dir\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dir_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_fg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fg_color_trampoline<
            P: IsA<DOMHTMLDocument>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLDocument,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLDocument::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::fg-color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fg_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<
            P: IsA<DOMHTMLDocument>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLDocument,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLDocument::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_link_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_link_color_trampoline<
            P: IsA<DOMHTMLDocument>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLDocument,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLDocument::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::link-color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_link_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_vlink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vlink_color_trampoline<
            P: IsA<DOMHTMLDocument>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLDocument,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLDocument::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vlink-color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vlink_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<
            P: IsA<DOMHTMLDocument>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLDocument,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLDocument::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLDocument")
    }
}
