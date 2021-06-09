use serde::Serialize;
use serde_json::Value;

use lambda_runtime::Context;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Output(pub HashMap<String, Value>);

pub async fn run(
    _event: HashMap<String, String>,
    _context: Context,
) -> Result<Output, Box<dyn std::error::Error>> {
    todo!()
}
