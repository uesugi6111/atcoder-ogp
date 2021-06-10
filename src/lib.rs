mod crawl;
mod template;

use crawl::crawl;
use serde::Serialize;

use serde_json::Map;
use template::{card, TEMPLETES};

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
    event: serde_json::Map<String, serde_json::Value>,
) -> Result<Output, Box<dyn std::error::Error>> {
    let param = get_url(&event);
    let target_page = crawl(&param).await?;

    let body = card(&TEMPLETES, &target_page);
    Ok(Output::new(&body))
}

fn get_url(event: &Map<String, serde_json::Value>) -> String {
    event
        .get("queryStringParameters")
        .unwrap()
        .as_object()
        .unwrap()
        .get("url")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_JSON: &str = r#"
{
    "version": "2.0",
    "routeKey": "ANY /myapp",
    "rawPath": "/default/myapp",
    "rawQueryString": "url=https://atcoder.jp/contests/abc204/submissions/23259725",
    "headers": {
        "accept-encoding": "gzip, deflate, br",
        "accept-language": "ja,en-US;q=0.9,en;q=0.8",
        "content-length": "0"
    },
    "queryStringParameters": {
        "url": "https://atcoder.jp/contests/abc204/submissions/23259725"
    },
    "requestContext": {
        "http": {
            "method": "GET",
            "path": "/default/myapp",
            "protocol": "HTTP/1.1"
        },
        "routeKey": "ANY /myapp",
        "stage": "default"
    },
    "isBase64Encoded": false
}
"#;
    #[test]
    fn test() {
        let a: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(SAMPLE_JSON).unwrap();
        let c = get_url(&a);
        assert_eq!(c, "https://atcoder.jp/contests/abc204/submissions/23259725");
    }
    #[tokio::test]
    async fn test_all() -> Result<(), Box<dyn std::error::Error>> {
        let output = run(serde_json::from_str(SAMPLE_JSON).unwrap()).await?;
        dbg!(&output);
        println!("{}", output.body);
        Ok(())
    }
}
