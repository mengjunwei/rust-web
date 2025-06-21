use crate::error::Error;

#[derive(Debug)]
pub struct ErrorMsg {
    inner: Error,
    code: u16,
    msg: String,
}
