use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("SDL2 error: {0}")]
    SdlError(String),
    #[error("Font loading error")]
    FontError,
    #[error("Rendering error: {0}")]
    RenderError(String),
}

impl From<String> for UiError {
    fn from(s: String) -> Self {
        UiError::SdlError(s)
    }
}