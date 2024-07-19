use std::fs::File;
use std::io::Read;


/// Returns the system timezone from the `/etc/timezone` file contents as a `String`.
///
pub(super) fn get_system_timezone_from_etc_timezone() -> Option<String> {
    let mut file: File = match File::open("/etc/timezone") {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_ok() {
        Some(contents.trim().to_string())
    } else {
        None
    }
}
