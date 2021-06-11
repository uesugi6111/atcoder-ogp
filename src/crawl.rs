use anyhow::Result;
use scraper::{Html, Selector};
use serde::Serialize;

const USER_SELECTER: &str = "table > tbody > tr:nth-child(3) > td > a:nth-child(1)";
const PROBLEM_SELECTER: &str = "table > tbody > tr:nth-child(2) > td > a:nth-child(1)";
const TITLE_SELECTER: &str = "#main-container > div.row > div:nth-child(2) > p:nth-child(1) > span";
const IMAGE_URL: &str =
    "https://jobs.img.atcoder.jp/company/atcoder/22e0e825aef7d8df994bb9f2b4e22996.png";

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
    let _submit_id = get_inner_html(&html_element, TITLE_SELECTER);
    let contest_name = get_inner_html(&html_element, "#navbar-collapse > ul:nth-child(1) > li > a");
    let description = generate_description(&html_element);

    TargetPage {
        url: url.to_string(),
        title: format!(
            "提出 - {} - {} by {}",
            contest_name, problem_name, user_name
        ),
        description,
        image_url: IMAGE_URL.to_string(),
    }
}
fn get_inner_html(html: &Html, selecter: &str) -> String {
    let selecter = Selector::parse(selecter).unwrap();
    match html.select(&selecter).next() {
        Some(e) => e.inner_html(),
        None => "取得失敗".to_string(),
    }
}
fn generate_description(html: &Html) -> String {
    let ss = "table > tbody > tr > th:nth-child(1) ";
    let sss = "table > tbody > tr > td:nth-child(2) ";

    let th_selecter = Selector::parse(ss).unwrap();
    let td_selecter = Selector::parse(sss).unwrap();

    let th = html.select(&th_selecter).collect::<Vec<_>>();
    let td = html.select(&td_selecter).collect::<Vec<_>>();

    (1..9)
        .map(|i| format!("{} : {}, ", get_inner_text(&th[i]), get_inner_text(&td[i])))
        .collect::<String>()
}
fn get_inner_text(e: &scraper::ElementRef) -> String {
    let mut a = e.children().next().unwrap();
    while a.has_children() {
        a = a.first_child().unwrap();
    }
    a.value().as_text().unwrap().text.to_string()
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
