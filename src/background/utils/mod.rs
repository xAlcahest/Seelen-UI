pub mod ahk;
pub mod constants;
pub mod pwsh;
pub mod rect;
pub mod virtual_desktop;

use std::{path::PathBuf, time::Duration};

use tauri::{AppHandle, Manager};
use windows::{
    core::GUID,
    Win32::{
        Foundation::{HANDLE, RECT},
        UI::Shell::{SHGetKnownFolderPath, KF_FLAG_DEFAULT},
    },
};

use crate::error_handler::Result;

pub fn sleep_millis(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}

pub fn filename_from_path(path: &str) -> String {
    path.split('\\').last().unwrap_or_default().to_string()
}

pub fn are_overlaped(a: &RECT, b: &RECT) -> bool {
    if a.right < b.left || a.left > b.right || a.bottom < b.top || a.top > b.bottom {
        return false;
    }
    true
}

pub fn pascal_to_kebab(input: &str) -> String {
    let mut kebab_case = String::new();
    let mut prev_char_lowercase = false;
    for c in input.chars() {
        if c.is_uppercase() {
            if prev_char_lowercase {
                kebab_case.push('-');
            }
            kebab_case.push(c.to_ascii_lowercase());
            prev_char_lowercase = false;
        } else {
            kebab_case.push(c);
            prev_char_lowercase = true;
        }
    }
    kebab_case
}

pub fn kebab_to_pascal(input: &str) -> String {
    let mut pascal_case = String::new();
    let mut prev_char_dash = false;
    for c in input.chars() {
        if c == '-' {
            prev_char_dash = true;
        } else {
            if prev_char_dash || pascal_case.is_empty() {
                pascal_case.push(c.to_ascii_uppercase());
                prev_char_dash = false;
            } else {
                pascal_case.push(c);
            }
        }
    }
    pascal_case
}

pub fn is_windows_10() -> bool {
    matches!(os_info::get().version(), os_info::Version::Semantic(_, _, x) if x >= &10240 && x < &22000)
}

pub fn is_windows_11() -> bool {
    matches!(os_info::get().version(), os_info::Version::Semantic(_, _, x) if x >= &22000)
}

/// Is Windows 11 22H2 or newer
pub fn is_windows_11_22h2() -> bool {
    matches!(os_info::get().version(), os_info::Version::Semantic(_, _, x) if x >= &22621)
}

/// Resolve paths with folder ids in the form of "{GUID}\path\to\file"
///
/// https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
pub fn resolve_guid_path<S: AsRef<str>>(path: S) -> Result<PathBuf> {
    let parts = path.as_ref().split("\\");
    let mut path_buf = PathBuf::new();

    for (idx, part) in parts.into_iter().enumerate() {
        if part.starts_with("{") && part.ends_with("}") {
            let guid = part.trim_start_matches('{').trim_end_matches('}');
            let rfid = GUID::from(guid);
            let string_path = unsafe {
                SHGetKnownFolderPath(&rfid as _, KF_FLAG_DEFAULT, HANDLE(0))?.to_string()?
            };

            path_buf.push(string_path);
        } else if idx == 0 {
            return Ok(PathBuf::from(path.as_ref()));
        } else {
            path_buf.push(part);
        }
    }

    Ok(path_buf)
}

pub fn app_data_path(handle: &AppHandle) -> PathBuf {
    handle
        .path()
        .app_data_dir()
        .expect("Failed to resolve App Data path")
}
