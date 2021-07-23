use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResult<T> {
    pub code: u16,
    pub msg: Option<Cow<'static, str>>,
    pub data: Option<T>,
}

impl<T> ApiResult<T> {
    pub fn new() -> Self {
        Self {
            code: 200,
            msg: None,
            data: None,
        }
    }

    /// Set the status code represented within the response
    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }
    /// Set the error message represented within the response
    pub fn with_msg<S: Into<Cow<'static, str>>>(mut self, msg: S) -> Self {
        self.msg = Some(msg.into());
        self
    }
    /// Set the data represented within the response
    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
}
