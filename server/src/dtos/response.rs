use salvo::{http::header::CONTENT_TYPE, prelude::*};
use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::error::AppError;

#[derive(Serialize, Debug, Default)]
pub struct SuccessResponse<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Value>,
}

#[derive(Serialize, Debug, Default)]
pub struct SuccessListResponse<T> {
    pub data: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Value>,
    pub meta: PaginationMeta,
}

#[derive(Serialize, Debug, Default)]
pub struct PaginationMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_id: Option<String>,
    pub result_count: u64,
    pub has_more: bool,
}

/// RFC 9457
#[derive(Serialize, Debug)]
pub struct ProblemDetails {
    // default as "about:blank"
    #[serde(rename = "type")]
    pub kind: String,
    pub title: String,
    // HTTP status code
    #[serde(serialize_with = "serialize_http_status")]
    pub status: StatusCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    // current request URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

fn serialize_http_status<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(status.as_u16())
}

impl Default for ProblemDetails {
    fn default() -> Self {
        Self {
            kind: "about:blank".to_owned(),
            title: "An error occurred".to_owned(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            detail: None,
            instance: None,
        }
    }
}

impl AppError {
    pub fn to_problem_details(&self, uri: String) -> ProblemDetails {
        match self {
            // TODO: Map specific error types to appropriate HTTP status codes and messages
            _ => ProblemDetails {
                instance: Some(uri),
                ..Default::default()
            },
        }
    }
}

#[async_trait]
impl Writer for AppError {
    async fn write(self, req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let problem = self.to_problem_details(req.uri().to_string());

        let body = serde_json::to_string(&problem).unwrap_or("{}".to_owned());

        res.status_code(problem.status);
        res.headers
            .insert(CONTENT_TYPE, "application/problem+json".parse().unwrap());
        let _ = res.write_body(body);
    }
}
