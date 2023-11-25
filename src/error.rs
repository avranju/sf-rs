use std::string::FromUtf16Error;

use thiserror::Error as ThisError;
use windows::core::Error as WindowsError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Windows Error: {0}")]
    Windows(#[from] WindowsError),

    #[error("Utf16 Error: {0}")]
    Utf16StringDecode(#[from] FromUtf16Error),

    #[error("Call abandoned: {0}")]
    Abandoned(&'static str),
}
