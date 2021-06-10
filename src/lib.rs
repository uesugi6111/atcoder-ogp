mod crawl;
mod template;

use crawl::crawl;
use serde::Serialize;
use template::{card, TEMPLETES};

use lambda_runtime::Context;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Output {
    #[serde(rename = "statusCode")]
    status_code: u64,
    #[serde(rename = "Content-Type")]
    content_type: String,
    body: String,
}

impl Output {
    pub fn new(body: &str) -> Self {
        Output {
            status_code: 200,
            content_type: "text/html".to_string(),
            body: body.to_string(),
        }
    }
}

pub async fn run(
    _event: HashMap<String, String>,
    _context: Context,
) -> Result<Output, Box<dyn std::error::Error>> {
    let param = "url";
    let target_page = crawl(param).await?;

    let body = card(&TEMPLETES, target_page);
    Ok(Output::new(&body))
}
