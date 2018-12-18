use core::result;
use failure_derive::Fail;

pub type Result<T> = result::Result<T, ZeebeClientError>;

#[derive(Fail, Debug)]
pub enum ZeebeClientError {
    #[fail(display = "Invalid value provided form argument {}: {}", name, message)]
    InvalidArgument { name: String, message: String },
    #[fail(display = "{}", _0)]
    Grpc(#[fail(cause)] grpcio::Error),
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] std::io::Error),
}

impl From<grpcio::Error> for ZeebeClientError {
    fn from(error: grpcio::Error) -> Self {
        ZeebeClientError::Grpc(error)
    }
}

impl From<std::io::Error> for ZeebeClientError {
    fn from(error: std::io::Error) -> Self {
        ZeebeClientError::Io(error)
    }
}
