// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitHitTestResult")]
    pub struct HitTestResult(Object<ffi::WebKitHitTestResult, ffi::WebKitHitTestResultClass>);

    match fn {
        type_ => || ffi::webkit_hit_test_result_get_type(),
    }
}

impl HitTestResult {
            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`HitTestResult`] objects.
            ///
            /// This method returns an instance of [`HitTestResultBuilder`] which can be used to create [`HitTestResult`] objects.
            pub fn builder() -> HitTestResultBuilder {
                HitTestResultBuilder::default()
            }
        
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`HitTestResult`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct HitTestResultBuilder {
    context: Option<u32>,
    image_uri: Option<String>,
    link_label: Option<String>,
    link_title: Option<String>,
    link_uri: Option<String>,
    media_uri: Option<String>,
}

impl HitTestResultBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`HitTestResultBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`HitTestResult`].
    pub fn build(self) -> HitTestResult {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
if let Some(ref context) = self.context {
                properties.push(("context", context));
            }
if let Some(ref image_uri) = self.image_uri {
                properties.push(("image-uri", image_uri));
            }
if let Some(ref link_label) = self.link_label {
                properties.push(("link-label", link_label));
            }
if let Some(ref link_title) = self.link_title {
                properties.push(("link-title", link_title));
            }
if let Some(ref link_uri) = self.link_uri {
                properties.push(("link-uri", link_uri));
            }
if let Some(ref media_uri) = self.media_uri {
                properties.push(("media-uri", media_uri));
            }
        glib::Object::new::<HitTestResult>(&properties)
                .expect("Failed to create an instance of HitTestResult")

    }

    pub fn context(mut self, context: u32) -> Self {
        self.context = Some(context);
        self
    }

    pub fn image_uri(mut self, image_uri: &str) -> Self {
        self.image_uri = Some(image_uri.to_string());
        self
    }

    pub fn link_label(mut self, link_label: &str) -> Self {
        self.link_label = Some(link_label.to_string());
        self
    }

    pub fn link_title(mut self, link_title: &str) -> Self {
        self.link_title = Some(link_title.to_string());
        self
    }

    pub fn link_uri(mut self, link_uri: &str) -> Self {
        self.link_uri = Some(link_uri.to_string());
        self
    }

    pub fn media_uri(mut self, media_uri: &str) -> Self {
        self.media_uri = Some(media_uri.to_string());
        self
    }
}

pub const NONE_HIT_TEST_RESULT: Option<&HitTestResult> = None;

pub trait HitTestResultExt: 'static {
    #[doc(alias = "webkit_hit_test_result_context_is_editable")]
    fn context_is_editable(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_context_is_image")]
    fn context_is_image(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_context_is_link")]
    fn context_is_link(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_context_is_media")]
    fn context_is_media(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_context_is_scrollbar")]
    fn context_is_scrollbar(&self) -> bool;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_hit_test_result_context_is_selection")]
    fn context_is_selection(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_get_context")]
    #[doc(alias = "get_context")]
    fn context(&self) -> u32;

    #[doc(alias = "webkit_hit_test_result_get_image_uri")]
    #[doc(alias = "get_image_uri")]
    fn image_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_hit_test_result_get_link_label")]
    #[doc(alias = "get_link_label")]
    fn link_label(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_hit_test_result_get_link_title")]
    #[doc(alias = "get_link_title")]
    fn link_title(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_hit_test_result_get_link_uri")]
    #[doc(alias = "get_link_uri")]
    fn link_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_hit_test_result_get_media_uri")]
    #[doc(alias = "get_media_uri")]
    fn media_uri(&self) -> Option<glib::GString>;
}

impl<O: IsA<HitTestResult>> HitTestResultExt for O {
    fn context_is_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_editable(self.as_ref().to_glib_none().0))
        }
    }

    fn context_is_image(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_image(self.as_ref().to_glib_none().0))
        }
    }

    fn context_is_link(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_link(self.as_ref().to_glib_none().0))
        }
    }

    fn context_is_media(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_media(self.as_ref().to_glib_none().0))
        }
    }

    fn context_is_scrollbar(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_scrollbar(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn context_is_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_selection(self.as_ref().to_glib_none().0))
        }
    }

    fn context(&self) -> u32 {
        unsafe {
            ffi::webkit_hit_test_result_get_context(self.as_ref().to_glib_none().0)
        }
    }

    fn image_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_image_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn link_label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_label(self.as_ref().to_glib_none().0))
        }
    }

    fn link_title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_title(self.as_ref().to_glib_none().0))
        }
    }

    fn link_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn media_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_media_uri(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for HitTestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("HitTestResult")
    }
}
