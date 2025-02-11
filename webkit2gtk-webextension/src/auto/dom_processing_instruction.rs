// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMCharacterData;
use crate::DOMEventTarget;
use crate::DOMNode;
use crate::DOMObject;
use crate::DOMStyleSheet;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitDOMProcessingInstruction")]
    pub struct DOMProcessingInstruction(Object<ffi::WebKitDOMProcessingInstruction, ffi::WebKitDOMProcessingInstructionClass>) @extends DOMCharacterData, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_processing_instruction_get_type(),
    }
}

pub const NONE_DOM_PROCESSING_INSTRUCTION: Option<&DOMProcessingInstruction> = None;

pub trait DOMProcessingInstructionExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_processing_instruction_get_sheet")]
    #[doc(alias = "get_sheet")]
    fn sheet(&self) -> Option<DOMStyleSheet>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_processing_instruction_get_target")]
    #[doc(alias = "get_target")]
    fn target(&self) -> Option<glib::GString>;

    #[doc(alias = "sheet")]
    fn connect_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "target")]
    fn connect_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMProcessingInstruction>> DOMProcessingInstructionExt for O {
    fn sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_processing_instruction_get_sheet(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn target(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_processing_instruction_get_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sheet_trampoline<
            P: IsA<DOMProcessingInstruction>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMProcessingInstruction,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMProcessingInstruction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sheet\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sheet_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_target_trampoline<
            P: IsA<DOMProcessingInstruction>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMProcessingInstruction,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMProcessingInstruction::from_glib_borrow(this).unsafe_cast_ref())
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

impl fmt::Display for DOMProcessingInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMProcessingInstruction")
    }
}
