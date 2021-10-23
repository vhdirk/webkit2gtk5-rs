// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::PermissionRequest;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitNotificationPermissionRequest")]
    pub struct NotificationPermissionRequest(Object<ffi::WebKitNotificationPermissionRequest, ffi::WebKitNotificationPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_notification_permission_request_get_type(),
    }
}

impl NotificationPermissionRequest {}

pub const NONE_NOTIFICATION_PERMISSION_REQUEST: Option<&NotificationPermissionRequest> = None;

impl fmt::Display for NotificationPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NotificationPermissionRequest")
    }
}
