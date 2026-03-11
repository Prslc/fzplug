use anyhow::{Context, Result};
use reqwest;
use std::process::Command;

pub fn search_web(query: &str) -> Result<()> {
    if query.is_empty() {
        println!("Search query cannot be empty.");
        return Ok(());
    }

    // Fetch autocomplete suggestions from DuckDuckGo API
    let suggestions = fetch_suggestions(query)?;

    // Display suggestions in dmenu
    let options = suggestions.join("\n");
    let selected = crate::ui::dmenu("Search Suggestions: ", &options).unwrap_or(query.to_string());

    // Perform the search with the selected or original query
    let search_url = format!("https://duckduckgo.com/?q={}", selected);
    Command::new("xdg-open")
        .arg(search_url)
        .spawn()
        .context("Failed to open web browser")?;

    Ok(())
}

fn fetch_suggestions(query: &str) -> Result<Vec<String>> {
    let url = format!("https://duckduckgo.com/ac/?q={}", query);
    let response = reqwest::blocking::get(&url).context("Failed to fetch suggestions")?;
    let suggestions: Vec<serde_json::Value> =
        response.json().context("Failed to parse suggestions")?;

    Ok(suggestions
        .into_iter()
        .filter_map(|item: serde_json::Value| item["phrase"].as_str().map(|s| s.to_string()))
        .collect::<Vec<String>>())
}

pub fn search_github(query: &str) -> Result<()> {
    let search_url = format!("https://github.com/search?q={}", query);
    Command::new("xdg-open")
        .arg(search_url)
        .spawn()
        .context("Failed to open web browser")?;

    Ok(())
}

pub fn search_bilibili(query: &str) -> Result<()> {
    let search_url = format!("https://search.bilibili.com/all?keyword={}", query);
    Command::new("xdg-open")
        .arg(search_url)
        .spawn()
        .context("Failed to open web browser")?;

    Ok(())
}