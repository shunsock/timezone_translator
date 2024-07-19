use std::fs;

pub(crate) fn get_system_timezone_from_etc_localtime() -> Option<String> {
    return match fs::read_link("/etc/localtime") {
        Ok(path) => {
            let path_str = path.to_string_lossy();
            if let Some(pos) = path_str.find("/zoneinfo/") {
                Some(path_str[pos + "/zoneinfo/".len()..].to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    };
}
