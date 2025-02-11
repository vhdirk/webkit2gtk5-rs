// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLElement;
use crate::DOMHTMLFormElement;
use crate::DOMHTMLOptionsCollection;
use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLSelectElement")]
    pub struct DOMHTMLSelectElement(Object<ffi::WebKitDOMHTMLSelectElement, ffi::WebKitDOMHTMLSelectElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_select_element_get_type(),
    }
}

pub const NONE_DOMHTML_SELECT_ELEMENT: Option<&DOMHTMLSelectElement> = None;

pub trait DOMHTMLSelectElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_add")]
    fn add(
        &self,
        element: &impl IsA<DOMHTMLElement>,
        before: &impl IsA<DOMHTMLElement>,
    ) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_autofocus")]
    #[doc(alias = "get_autofocus")]
    fn is_autofocus(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_disabled")]
    #[doc(alias = "get_disabled")]
    fn is_disabled(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_form")]
    #[doc(alias = "get_form")]
    fn form(&self) -> Option<DOMHTMLFormElement>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_multiple")]
    #[doc(alias = "get_multiple")]
    fn is_multiple(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_options")]
    #[doc(alias = "get_options")]
    fn options(&self) -> Option<DOMHTMLOptionsCollection>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_select_type")]
    #[doc(alias = "get_select_type")]
    fn select_type(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_selected_index")]
    #[doc(alias = "get_selected_index")]
    fn selected_index(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_get_will_validate")]
    #[doc(alias = "get_will_validate")]
    fn is_will_validate(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_item")]
    fn item(&self, index: libc::c_ulong) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_named_item")]
    fn named_item(&self, name: &str) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_remove")]
    fn remove(&self, index: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_set_autofocus")]
    fn set_autofocus(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_set_disabled")]
    fn set_disabled(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_set_length")]
    fn set_length(&self, value: libc::c_ulong) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_set_multiple")]
    fn set_multiple(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_set_name")]
    fn set_name(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_set_selected_index")]
    fn set_selected_index(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_set_size")]
    fn set_size(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_select_element_set_value")]
    fn set_value(&self, value: &str);

    #[doc(alias = "type")]
    fn type_(&self) -> Option<glib::GString>;

    #[doc(alias = "autofocus")]
    fn connect_autofocus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "disabled")]
    fn connect_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "form")]
    fn connect_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "multiple")]
    fn connect_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "options")]
    fn connect_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "selected-index")]
    fn connect_selected_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "size")]
    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "type")]
    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "will-validate")]
    fn connect_will_validate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLSelectElement>> DOMHTMLSelectElementExt for O {
    fn add(
        &self,
        element: &impl IsA<DOMHTMLElement>,
        before: &impl IsA<DOMHTMLElement>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_select_element_add(
                self.as_ref().to_glib_none().0,
                element.as_ref().to_glib_none().0,
                before.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn is_autofocus(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_autofocus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_disabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_select_element_get_form(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn length(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_html_select_element_get_length(self.as_ref().to_glib_none().0) }
    }

    fn is_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_multiple(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn options(&self) -> Option<DOMHTMLOptionsCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_options(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn select_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_select_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn selected_index(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_select_element_get_selected_index(self.as_ref().to_glib_none().0)
        }
    }

    fn size(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_select_element_get_size(self.as_ref().to_glib_none().0) }
    }

    fn value(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_will_validate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_will_validate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_select_element_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_select_element_named_item(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn remove(&self, index: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_select_element_remove(self.as_ref().to_glib_none().0, index);
        }
    }

    fn set_autofocus(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_autofocus(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_disabled(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    fn set_length(&self, value: libc::c_ulong) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_select_element_set_length(
                self.as_ref().to_glib_none().0,
                value,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_multiple(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_multiple(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_name(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_selected_index(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_selected_index(
                self.as_ref().to_glib_none().0,
                value,
            );
        }
    }

    fn set_size(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_size(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn type_(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
        }
    }

    fn connect_autofocus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autofocus_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::autofocus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_autofocus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_disabled_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::disabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_disabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_form_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_multiple_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::multiple\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_multiple_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_options_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::options\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_options_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selected_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_index_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected-index\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_index_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_will_validate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_will_validate_trampoline<
            P: IsA<DOMHTMLSelectElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLSelectElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLSelectElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::will-validate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_will_validate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLSelectElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLSelectElement")
    }
}
