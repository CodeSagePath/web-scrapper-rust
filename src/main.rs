use reqwest::blocking::get;
use scraper::{Html, Selector};
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://canopy-how-it-works.notion.site/HOW-IT-WORKS-f6314627a8fa46f1852e9973af60876b";

    // Fetch the page content
    let response = get(url)?.text()?;

    // Save the raw HTML
    let mut html_file = File::create("notion_page.html")?;
    html_file.write_all(response.as_bytes())?;

    // Parse the HTML for text content
    let document = Html::parse_document(&response);
    let selector = Selector::parse("div.notion-page-content")?;
    let mut text_content = String::new();

    for element in document.select(&selector) {
        text_content.push_str(&element.text().collect::<Vec<_>>().join("\n"));
    }

    // Save the text content
    let mut text_file = File::create("notion_page.txt")?;
    text_file.write_all(text_content.as_bytes())?;

    // Regex to find asset URLs (images, PDFs, etc.)
    let re = Regex::new(r#"https?://[^\s"']+\.(?:png|jpg|jpeg|gif|pdf|svg)"#)?;

    // Create a directory to store the downloaded files
    let output_dir = "downloads";
    fs::create_dir_all(output_dir)?;

    // Download each matched URL
    for cap in re.captures_iter(&response) {
        let asset_url = &cap[0];
        let filename = asset_url.split('/').last().unwrap_or("file");

        println!("Downloading: {}", asset_url);

        let mut resp = get(asset_url)?;
        let filepath = Path::new(output_dir).join(filename);
        let mut file = File::create(filepath)?;

        // Save the asset to the file
        std::io::copy(&mut resp, &mut file)?;
    }

    println!("All content extracted and saved.");
    Ok(())
}
