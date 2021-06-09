use lambda_runtime::{error::HandlerError, lambda, Context};

use atcoder_ogp::{run, Output};
use serde_json::Value;

async fn handler(
    event: std::collections::HashMap<String, String>,
    context: Context,
) -> Result<Output, HandlerError> {
    Ok(run(event, context).await.unwrap_or_else(|e| {
        Output({
            let mut a = std::collections::HashMap::new();
            a.insert("message".to_string(), Value::String(e.to_string()));
            a
        })
    }))
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let rt = tokio::runtime::Runtime::new().unwrap();
    lambda!(move |event, context| rt.block_on(handler(event, context)));
}
