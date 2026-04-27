use crate::domain::errors::FileError;

pub(crate) fn transport(err: reqwest::Error) -> FileError {
    FileError::Transport(err.to_string())
}

pub(crate) fn load_credential(env_var: &'static str) -> Result<String, FileError> {
    std::env::var(env_var).map_err(|_| FileError::MissingCredential(env_var.into()))
}
