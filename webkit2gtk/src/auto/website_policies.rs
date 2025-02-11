// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use crate::AutoplayPolicy;
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitWebsitePolicies")]
    pub struct WebsitePolicies(Object<ffi::WebKitWebsitePolicies, ffi::WebKitWebsitePoliciesClass>);

    match fn {
        type_ => || ffi::webkit_website_policies_get_type(),
    }
}

impl WebsitePolicies {
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_policies_new")]
    pub fn new() -> WebsitePolicies {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_website_policies_new()) }
    }

    //#[cfg(any(feature = "v2_30", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    //#[doc(alias = "webkit_website_policies_new_with_policies")]
    //#[doc(alias = "new_with_policies")]
    //pub fn with_policies(first_policy_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> WebsitePolicies {
    //    unsafe { TODO: call ffi:webkit_website_policies_new_with_policies() }
    //}

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`WebsitePolicies`] objects.
    ///
    /// This method returns an instance of [`WebsitePoliciesBuilder`] which can be used to create [`WebsitePolicies`] objects.
    pub fn builder() -> WebsitePoliciesBuilder {
        WebsitePoliciesBuilder::default()
    }
}

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
impl Default for WebsitePolicies {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`WebsitePolicies`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct WebsitePoliciesBuilder {
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    autoplay: Option<AutoplayPolicy>,
}

impl WebsitePoliciesBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`WebsitePoliciesBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`WebsitePolicies`].
    pub fn build(self) -> WebsitePolicies {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_30", feature = "dox"))]
        if let Some(ref autoplay) = self.autoplay {
            properties.push(("autoplay", autoplay));
        }
        glib::Object::new::<WebsitePolicies>(&properties)
            .expect("Failed to create an instance of WebsitePolicies")
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    pub fn autoplay(mut self, autoplay: AutoplayPolicy) -> Self {
        self.autoplay = Some(autoplay);
        self
    }
}

pub const NONE_WEBSITE_POLICIES: Option<&WebsitePolicies> = None;

pub trait WebsitePoliciesExt: 'static {
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_policies_get_autoplay_policy")]
    #[doc(alias = "get_autoplay_policy")]
    fn autoplay_policy(&self) -> AutoplayPolicy;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn autoplay(&self) -> AutoplayPolicy;
}

impl<O: IsA<WebsitePolicies>> WebsitePoliciesExt for O {
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn autoplay_policy(&self) -> AutoplayPolicy {
        unsafe {
            from_glib(ffi::webkit_website_policies_get_autoplay_policy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn autoplay(&self) -> AutoplayPolicy {
        unsafe {
            let mut value = glib::Value::from_type(<AutoplayPolicy as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"autoplay\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `autoplay` getter")
        }
    }
}

impl fmt::Display for WebsitePolicies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebsitePolicies")
    }
}
