use std::path::Path;

pub fn is_mac_app_in_path(app_name: &str) -> bool {
    Path::new("/Applications/")
        .join(format!("{}.app", app_name))
        .exists()
}
