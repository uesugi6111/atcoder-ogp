use lambda_runtime::{error::HandlerError, lambda, Context};

use atcoder_ogp::{run, Output};

async fn handler(
    event: std::collections::HashMap<String, String>,
    context: Context,
) -> Result<Output, HandlerError> {
    Ok(run(event, context).await.unwrap_or_else(|e| {
        println!("{}",e);
        Output::new("エラーが発生しました<a href=\"https://twitter.com/takeda_SE\">uesugi</a>までお問い合わせください。")
    }))
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let rt = tokio::runtime::Runtime::new().unwrap();
    lambda!(move |event, context| rt.block_on(handler(event, context)));
}
