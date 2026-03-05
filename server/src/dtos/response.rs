use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl<T> ResponseData<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            error: None,
            includes: None,
            meta: None,
        }
    }

    pub fn with_meta(mut self, meta: Meta) -> Self {
        self.meta = Some(meta);
        self
    }
}

#[async_trait]
impl<T> Scribe for ResponseData<T>
where
    T: Serialize + Send,
{
    fn render(self, res: &mut Response) {
        res.render(Json(self));
    }
}
