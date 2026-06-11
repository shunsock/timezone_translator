use std::env;

pub(crate) struct EnvironmentVariableTzProvider {
    env_name: String,
}

impl EnvironmentVariableTzProvider {
    pub(crate) fn new(env_var_name: Option<String>) -> Self {
        match env_var_name {
            Some(env_name) => EnvironmentVariableTzProvider { env_name },
            None => EnvironmentVariableTzProvider {
                env_name: "TZ".to_string(),
            },
        }
    }
}

impl EnvironmentVariableTzProvider {
    pub(crate) fn get_env_var_tz(&self) -> Option<String> {
        env::var(&self.env_name).ok()
    }
}
