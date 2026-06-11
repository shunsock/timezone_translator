//! The three places a Unix system records its timezone.
//!
//! Each source returns `None` when it cannot answer, so the caller
//! can try them in order of reliability.

use std::env;
use std::fs;

/// Reads the `TZ` environment variable.
pub(crate) fn timezone_from_env_var() -> Option<String> {
    env::var("TZ").ok()
}

/// Follows the `/etc/localtime` symlink and extracts the zone name
/// after the `zoneinfo/` directory (e.g. `Asia/Tokyo`).
pub(crate) fn timezone_from_etc_localtime() -> Option<String> {
    let link_target = fs::read_link("/etc/localtime").ok()?;
    let path = link_target.to_string_lossy();

    path.find("/zoneinfo/")
        .map(|position| path[position + "/zoneinfo/".len()..].to_string())
}

/// Reads the zone name written in `/etc/timezone` (Debian-style).
pub(crate) fn timezone_from_etc_timezone() -> Option<String> {
    fs::read_to_string("/etc/timezone")
        .ok()
        .map(|contents| contents.trim().to_string())
}
