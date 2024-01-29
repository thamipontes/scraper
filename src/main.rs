use scraper::{Html, Selector};
use reqwest;
use encoding_rs::WINDOWS_1252;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    let url = "https://lucas-schiavini.com";

    let res_bytes = reqwest::get(url).await?.bytes().await?;

    let (res, _, _) = WINDOWS_1252.decode(&res_bytes);

    let document = Html::parse_document(&res);

    let selector = Selector::parse("a.u-permalink").map_err(|e| {
        eprintln!("Erro ao fazer o parsing do seletor: {:?}", e);
        "Erro ao fazer o parsing do seletor" 
    })?;

    for element in document.select(&selector) {
        if let Some(text) = element.value().attr("aria-label") {
            if text.starts_with("M") {
                if let Some(href) = element.value().attr("href") {
                    let new_url = format!("{}{}", url, href);
                    let new_res_m = reqwest::get(&new_url).await?.bytes().await?;

                    let (res_m, _, _) = WINDOWS_1252.decode(&new_res_m);

                    let document_m = Html::parse_document(&res_m);

                    let selector_m = Selector::parse("p").map_err(|e| {
                        eprintln!("Erro ao fazer o parsing do seletor: {:?}", e);
                        "Erro ao fazer o parsing do seletor"
                    })?;
                    for element_m in document_m.select(&selector_m) {
                    
                        if let Some(text_m) = element_m.text().next() {
                            if text_m.starts_with("I") {
                                    println!("{}", text_m);    
                            }
                        }
                    }
                    
                    break;
                }
            }
        }
    }

    Ok(())
}
