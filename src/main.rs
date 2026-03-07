mod firefox;
mod search;
mod utils;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
let options = [
    "[b]  Browser Bookmarks",
    "[h]  Browser History",
    "[s]  Web Search",
].join("\n");

    // Show menu
    let choice = match ui::dmenu("Launch: ", &options) {
        Some(c) => c,
        None => return Ok(()), // cancel
    };

    // Parsing
    let input = choice.split('|').next().unwrap_or("").trim();

    let mut parts = input.splitn(2, ' ');
    let plugin_key = parts.next().unwrap_or("").trim();
    let search_text = parts.next().unwrap_or("").trim();

    // Routing
    match plugin_key {
        "b" => firefox::search_and_launch(firefox::Mode::Bookmarks, search_text)?,
        "h" => firefox::search_and_launch(firefox::Mode::History, search_text)?,
        "s" => search::search_web(search_text)?,
       _ => {}
    }

    Ok(())
}
