use crate::{Error, Result};

use super::Context;

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub context: Context,
    pub error_code: Option<i64>,
    pub error_message: Option<String>,
    #[serde(rename = "type")]
    pub result: ResponseType,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseType {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "table")]
    Table
}

impl Response {
    pub fn to_result(self) -> Result<()> {
        match self.error_message {
            None => Ok(()),
            Some(error) => Err(Error::InternalError(error)),
        }
    }
}
