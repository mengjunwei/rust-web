use crate::error::Error;

#[derive(Debug)]
pub struct ErrorMsg {
    inner: Error,
    code: u16,
    msg: String,
}

impl ErrorMsg {
    /// 返回原始错误
    pub fn into_inner(self) -> Error {
        self.inner
    }

    /// 返回错误码
    pub fn code(&self) -> u16 {
        self.code
    }

    /// 返回错误信息
    pub fn msg(&self) -> &str {
        &self.msg
    }

    /// 重置错误信息
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    /// 追加错误信息, 在错误码信息的基础上添加新的信息
    pub fn append_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg.to_owned());
        self
    }
}
