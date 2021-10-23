// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::WebView;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitFindController")]
    pub struct FindController(Object<ffi::WebKitFindController, ffi::WebKitFindControllerClass>);

    match fn {
        type_ => || ffi::webkit_find_controller_get_type(),
    }
}

impl FindController {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FindController`] objects.
    ///
    /// This method returns an instance of [`FindControllerBuilder`] which can be used to create [`FindController`] objects.
    pub fn builder() -> FindControllerBuilder {
        FindControllerBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FindController`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct FindControllerBuilder {
    web_view: Option<WebView>,
}

impl FindControllerBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FindControllerBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FindController`].
    pub fn build(self) -> FindController {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref web_view) = self.web_view {
            properties.push(("web-view", web_view));
        }
        glib::Object::new::<FindController>(&properties)
            .expect("Failed to create an instance of FindController")
    }

    pub fn web_view(mut self, web_view: &impl IsA<WebView>) -> Self {
        self.web_view = Some(web_view.clone().upcast());
        self
    }
}

pub const NONE_FIND_CONTROLLER: Option<&FindController> = None;

pub trait FindControllerExt: 'static {
    #[doc(alias = "webkit_find_controller_count_matches")]
    fn count_matches(&self, search_text: &str, find_options: u32, max_match_count: u32);

    #[doc(alias = "webkit_find_controller_get_max_match_count")]
    #[doc(alias = "get_max_match_count")]
    fn max_match_count(&self) -> u32;

    #[doc(alias = "webkit_find_controller_get_options")]
    #[doc(alias = "get_options")]
    fn options(&self) -> u32;

    #[doc(alias = "webkit_find_controller_get_search_text")]
    #[doc(alias = "get_search_text")]
    fn search_text(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_find_controller_get_web_view")]
    #[doc(alias = "get_web_view")]
    fn web_view(&self) -> Option<WebView>;

    #[doc(alias = "webkit_find_controller_search")]
    fn search(&self, search_text: &str, find_options: u32, max_match_count: u32);

    #[doc(alias = "webkit_find_controller_search_finish")]
    fn search_finish(&self);

    #[doc(alias = "webkit_find_controller_search_next")]
    fn search_next(&self);

    #[doc(alias = "webkit_find_controller_search_previous")]
    fn search_previous(&self);

    fn text(&self) -> Option<glib::GString>;

    #[doc(alias = "counted-matches")]
    fn connect_counted_matches<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "failed-to-find-text")]
    fn connect_failed_to_find_text<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "found-text")]
    fn connect_found_text<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "max-match-count")]
    fn connect_max_match_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "options")]
    fn connect_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "text")]
    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FindController>> FindControllerExt for O {
    fn count_matches(&self, search_text: &str, find_options: u32, max_match_count: u32) {
        unsafe {
            ffi::webkit_find_controller_count_matches(
                self.as_ref().to_glib_none().0,
                search_text.to_glib_none().0,
                find_options,
                max_match_count,
            );
        }
    }

    fn max_match_count(&self) -> u32 {
        unsafe { ffi::webkit_find_controller_get_max_match_count(self.as_ref().to_glib_none().0) }
    }

    fn options(&self) -> u32 {
        unsafe { ffi::webkit_find_controller_get_options(self.as_ref().to_glib_none().0) }
    }

    fn search_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_find_controller_get_search_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn web_view(&self) -> Option<WebView> {
        unsafe {
            from_glib_none(ffi::webkit_find_controller_get_web_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn search(&self, search_text: &str, find_options: u32, max_match_count: u32) {
        unsafe {
            ffi::webkit_find_controller_search(
                self.as_ref().to_glib_none().0,
                search_text.to_glib_none().0,
                find_options,
                max_match_count,
            );
        }
    }

    fn search_finish(&self) {
        unsafe {
            ffi::webkit_find_controller_search_finish(self.as_ref().to_glib_none().0);
        }
    }

    fn search_next(&self) {
        unsafe {
            ffi::webkit_find_controller_search_next(self.as_ref().to_glib_none().0);
        }
    }

    fn search_previous(&self) {
        unsafe {
            ffi::webkit_find_controller_search_previous(self.as_ref().to_glib_none().0);
        }
    }

    fn text(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text` getter")
        }
    }

    fn connect_counted_matches<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn counted_matches_trampoline<
            P: IsA<FindController>,
            F: Fn(&P, u32) + 'static,
        >(
            this: *mut ffi::WebKitFindController,
            match_count: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                FindController::from_glib_borrow(this).unsafe_cast_ref(),
                match_count,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"counted-matches\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    counted_matches_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_failed_to_find_text<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn failed_to_find_text_trampoline<
            P: IsA<FindController>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitFindController,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FindController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"failed-to-find-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    failed_to_find_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_found_text<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn found_text_trampoline<
            P: IsA<FindController>,
            F: Fn(&P, u32) + 'static,
        >(
            this: *mut ffi::WebKitFindController,
            match_count: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                FindController::from_glib_borrow(this).unsafe_cast_ref(),
                match_count,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"found-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    found_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_max_match_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_match_count_trampoline<
            P: IsA<FindController>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitFindController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FindController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-match-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_match_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_options_trampoline<
            P: IsA<FindController>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitFindController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FindController::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P: IsA<FindController>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitFindController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FindController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FindController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FindController")
    }
}
