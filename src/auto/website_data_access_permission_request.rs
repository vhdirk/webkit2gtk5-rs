// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::PermissionRequest;
use glib::object::IsA;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitWebsiteDataAccessPermissionRequest")]
    pub struct WebsiteDataAccessPermissionRequest(Object<ffi::WebKitWebsiteDataAccessPermissionRequest, ffi::WebKitWebsiteDataAccessPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_website_data_access_permission_request_get_type(),
    }
}

pub const NONE_WEBSITE_DATA_ACCESS_PERMISSION_REQUEST: Option<&WebsiteDataAccessPermissionRequest> =
    None;

pub trait WebsiteDataAccessPermissionRequestExt: 'static {
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_access_permission_request_get_current_domain")]
    #[doc(alias = "get_current_domain")]
    fn current_domain(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_access_permission_request_get_requesting_domain")]
    #[doc(alias = "get_requesting_domain")]
    fn requesting_domain(&self) -> Option<glib::GString>;
}

impl<O: IsA<WebsiteDataAccessPermissionRequest>> WebsiteDataAccessPermissionRequestExt for O {
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn current_domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(
                ffi::webkit_website_data_access_permission_request_get_current_domain(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn requesting_domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(
                ffi::webkit_website_data_access_permission_request_get_requesting_domain(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }
}

impl fmt::Display for WebsiteDataAccessPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebsiteDataAccessPermissionRequest")
    }
}
