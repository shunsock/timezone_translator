use std::env;
use std::env::VarError;

pub(super) struct EnvironmentVariableTzProvider {
    env_name: String,
}

impl EnvironmentVariableTzProvider {
    pub(super) fn new(env_var_name: Option<String>) -> Self {
        return match env_var_name {
            Some(env_name) => EnvironmentVariableTzProvider { env_name },
            None => EnvironmentVariableTzProvider {
                env_name: "TZ".to_string(),
            },
        };
    }
}

impl EnvironmentVariableTzProvider {
    pub(super) fn get_env_var_tz(&self) -> Option<String> {
        let timezone: Result<String, VarError> = env::var(&self.env_name);

        match timezone {
            Ok(tz) => Some(tz),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn set_tmp_env_var() {
        env::set_var("TEST_TZ", "America/New_York");
    }

    fn remove_tmp_env_var() {
        env::remove_var("TEST_TZ");
    }

    #[test]
    fn test_get_env_var_tz() {
        set_tmp_env_var();
        let tz_provider = EnvironmentVariableTzProvider::new(Some("TEST_TZ".to_string()));
        assert_eq!(
            tz_provider.get_env_var_tz(),
            Some("America/New_York".to_string())
        );
        remove_tmp_env_var();
    }
}
