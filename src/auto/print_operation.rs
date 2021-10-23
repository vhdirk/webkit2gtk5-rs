// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use crate::PrintCustomWidget;
use crate::PrintOperationResponse;
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
    #[doc(alias = "WebKitPrintOperation")]
    pub struct PrintOperation(Object<ffi::WebKitPrintOperation, ffi::WebKitPrintOperationClass>);

    match fn {
        type_ => || ffi::webkit_print_operation_get_type(),
    }
}

impl PrintOperation {
    #[doc(alias = "webkit_print_operation_new")]
    pub fn new(web_view: &impl IsA<WebView>) -> PrintOperation {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_print_operation_new(
                web_view.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PrintOperation`] objects.
    ///
    /// This method returns an instance of [`PrintOperationBuilder`] which can be used to create [`PrintOperation`] objects.
    pub fn builder() -> PrintOperationBuilder {
        PrintOperationBuilder::default()
    }
}

impl Default for PrintOperation {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct PrintOperation object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PrintOperation`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct PrintOperationBuilder {
    page_setup: Option<gtk::PageSetup>,
    print_settings: Option<gtk::PrintSettings>,
    web_view: Option<WebView>,
}

impl PrintOperationBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PrintOperationBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`PrintOperation`].
    pub fn build(self) -> PrintOperation {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref page_setup) = self.page_setup {
            properties.push(("page-setup", page_setup));
        }
        if let Some(ref print_settings) = self.print_settings {
            properties.push(("print-settings", print_settings));
        }
        if let Some(ref web_view) = self.web_view {
            properties.push(("web-view", web_view));
        }
        glib::Object::new::<PrintOperation>(&properties)
            .expect("Failed to create an instance of PrintOperation")
    }

    pub fn page_setup(mut self, page_setup: &gtk::PageSetup) -> Self {
        self.page_setup = Some(page_setup.clone());
        self
    }

    pub fn print_settings(mut self, print_settings: &gtk::PrintSettings) -> Self {
        self.print_settings = Some(print_settings.clone());
        self
    }

    pub fn web_view(mut self, web_view: &impl IsA<WebView>) -> Self {
        self.web_view = Some(web_view.clone().upcast());
        self
    }
}

pub const NONE_PRINT_OPERATION: Option<&PrintOperation> = None;

pub trait PrintOperationExt: 'static {
    #[doc(alias = "webkit_print_operation_get_page_setup")]
    #[doc(alias = "get_page_setup")]
    fn page_setup(&self) -> Option<gtk::PageSetup>;

    #[doc(alias = "webkit_print_operation_get_print_settings")]
    #[doc(alias = "get_print_settings")]
    fn print_settings(&self) -> Option<gtk::PrintSettings>;

    #[doc(alias = "webkit_print_operation_print")]
    fn print(&self);

    #[doc(alias = "webkit_print_operation_run_dialog")]
    fn run_dialog(&self, parent: Option<&impl IsA<gtk::Window>>) -> PrintOperationResponse;

    #[doc(alias = "webkit_print_operation_set_page_setup")]
    fn set_page_setup(&self, page_setup: &gtk::PageSetup);

    #[doc(alias = "webkit_print_operation_set_print_settings")]
    fn set_print_settings(&self, print_settings: &gtk::PrintSettings);

    #[doc(alias = "web-view")]
    fn web_view(&self) -> Option<WebView>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "create-custom-widget")]
    fn connect_create_custom_widget<F: Fn(&Self) -> PrintCustomWidget + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "failed")]
    fn connect_failed<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "finished")]
    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "page-setup")]
    fn connect_page_setup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "print-settings")]
    fn connect_print_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PrintOperation>> PrintOperationExt for O {
    fn page_setup(&self) -> Option<gtk::PageSetup> {
        unsafe {
            from_glib_none(ffi::webkit_print_operation_get_page_setup(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn print_settings(&self) -> Option<gtk::PrintSettings> {
        unsafe {
            from_glib_none(ffi::webkit_print_operation_get_print_settings(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn print(&self) {
        unsafe {
            ffi::webkit_print_operation_print(self.as_ref().to_glib_none().0);
        }
    }

    fn run_dialog(&self, parent: Option<&impl IsA<gtk::Window>>) -> PrintOperationResponse {
        unsafe {
            from_glib(ffi::webkit_print_operation_run_dialog(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    fn set_page_setup(&self, page_setup: &gtk::PageSetup) {
        unsafe {
            ffi::webkit_print_operation_set_page_setup(
                self.as_ref().to_glib_none().0,
                page_setup.to_glib_none().0,
            );
        }
    }

    fn set_print_settings(&self, print_settings: &gtk::PrintSettings) {
        unsafe {
            ffi::webkit_print_operation_set_print_settings(
                self.as_ref().to_glib_none().0,
                print_settings.to_glib_none().0,
            );
        }
    }

    fn web_view(&self) -> Option<WebView> {
        unsafe {
            let mut value = glib::Value::from_type(<WebView as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"web-view\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `web-view` getter")
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn connect_create_custom_widget<F: Fn(&Self) -> PrintCustomWidget + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_custom_widget_trampoline<
            P: IsA<PrintOperation>,
            F: Fn(&P) -> PrintCustomWidget + 'static,
        >(
            this: *mut ffi::WebKitPrintOperation,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::WebKitPrintCustomWidget {
            let f: &F = &*(f as *const F);
            f(PrintOperation::from_glib_borrow(this).unsafe_cast_ref()).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-custom-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    create_custom_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_failed<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn failed_trampoline<
            P: IsA<PrintOperation>,
            F: Fn(&P, &glib::Error) + 'static,
        >(
            this: *mut ffi::WebKitPrintOperation,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                PrintOperation::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(error),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"failed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    failed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn finished_trampoline<P: IsA<PrintOperation>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitPrintOperation,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PrintOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"finished\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    finished_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_page_setup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_setup_trampoline<
            P: IsA<PrintOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitPrintOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PrintOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::page-setup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_page_setup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_print_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_print_settings_trampoline<
            P: IsA<PrintOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitPrintOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PrintOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::print-settings\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_print_settings_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PrintOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PrintOperation")
    }
}
