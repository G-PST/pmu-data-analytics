#[derive(thiserror::Error,Debug)]
pub enum Error {
    #[error("Generic Error {0}")]
    Generic(String)
}
