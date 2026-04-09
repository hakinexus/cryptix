use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptixError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Image Processing Error: {0}")]
    Image(#[from] image::ImageError),
    
    #[error("Invalid Key: {0}")]
    InvalidKey(String),
    
    #[error("Output MUST be saved as a lossless format (.png).")]
    LossyFormat,

    #[error("INTEGRITY FAILURE: Data has been tampered with or corrupted!")]
    TamperedData,
}
