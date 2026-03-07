use std::collections::HashMap;

use salvo::{http::header::CONTENT_TYPE, prelude::*};
use serde::Serialize;
use serde_json::Value;

use crate::error::AppError;

#[derive(Serialize, Debug)]
pub struct OkEnvelope<T> {
    pub data: T,
    pub meta: CommonMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Value>,
}

type OkListEnvelope<T> = OkEnvelope<Vec<T>>;

#[derive(Serialize, Debug)]
pub struct CommonMeta {
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_token: Option<String>,
}

/// RFC 9457
#[derive(Serialize, Debug)]
pub struct ProblemDetails {
    // --- 5 standard fields ---

    // default as "about:blank"
    #[serde(rename = "type")]
    pub kind: String,

    pub title: String,

    // HTTP status code
    #[serde(serialize_with = "http_status_serializer")]
    pub status: StatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    // current request URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    // --- extension fields ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationError>>,
}

fn http_status_serializer<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u16(status.as_u16())
}

#[derive(Serialize, Debug)]
pub struct ValidationError {
    pub message: String,
    pub parameters: Option<HashMap<String, Vec<String>>>,
}

impl Default for ProblemDetails {
    fn default() -> Self {
        Self {
            kind: "about:blank".to_string(),
            title: "An error occurred".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            detail: None,
            instance: None,
            errors: None,
        }
    }
}

impl AppError {
    pub fn to_problem_details(&self, uri: String) -> ProblemDetails {
        match self {
            AppError::General(msg) => ProblemDetails {
                kind: "about:blank".to_string(),
                detail: Some(msg.clone()),
                instance: Some(uri),
                status: StatusCode::INTERNAL_SERVER_ERROR,
                ..Default::default()
            },
        }
    }
}

#[async_trait]
impl Writer for AppError {
    async fn write(self, req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let problem = self.to_problem_details(req.uri().to_string());

        res.status_code(problem.status).render(Json(problem));

        res.headers
            .insert(CONTENT_TYPE, "application/problem+json".parse().unwrap());
    }
}
