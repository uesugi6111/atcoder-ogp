use anyhow::Result;
use scraper::{Html, Selector};
use serde::Serialize;

const USER_SELECTER: &str = "table > tbody > tr:nth-child(3) > td > a:nth-child(1)";
const PROBLEM_SELECTER: &str = "table > tbody > tr:nth-child(2) > td > a:nth-child(1)";
const TITLE_SELECTER: &str = "#main-container > div.row > div:nth-child(2) > p:nth-child(1) > span";

#[derive(Debug, Serialize)]
pub struct TargetPage {
    pub url: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
}

pub async fn crawl(url: &str) -> Result<TargetPage> {
    let html = fetch_page(url).await?;
    Ok(extract_target(url, &html))
}
async fn fetch_page(url: &str) -> Result<String> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

fn extract_target(url: &str, html: &str) -> TargetPage {
    let html_element = scraper::Html::parse_document(html);

    let user_name = get_inner_html(&html_element, USER_SELECTER);
    let problem_name = get_inner_html(&html_element, PROBLEM_SELECTER);
    let submit_id = get_inner_html(&html_element, TITLE_SELECTER);

    TargetPage {
        url: url.to_string(),
        title: format!("{} {} by {}", problem_name, submit_id, user_name),
        description: String::new(),
        image_url: String::new(),
    }
}
fn get_inner_html(html: &Html, selecter: &str) -> String {
    let selecter = Selector::parse(selecter).unwrap();
    match html.select(&selecter).next() {
        Some(e) => e.inner_html(),
        None => "取得失敗".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TARGET_URL: &str = "https://atcoder.jp/contests/abc204/submissions/23259725";
    #[tokio::test]
    async fn test() -> Result<()> {
        let target_page = crawl(TARGET_URL).await?;

        dbg!(target_page);
        Ok(())
    }
    #[tokio::test]
    async fn fetch_test() -> Result<()> {
        let target_page = fetch_page(TARGET_URL).await?;

        dbg!(target_page);
        Ok(())
    }
}
