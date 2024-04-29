extern crate reqwest;
extern crate select;
extern crate scraper;


use scraper::Node;
use select::document::Document;
use select::predicate::{Name};
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
mod constants;

// fn confirm_string_exists(row: &select::node::Node<'_>) -> bool {
//     false
// }

async fn start_scrape(url: &str, specified_date: &str) -> Result<bool, reqwest::Error> {
    // println!("url: {}", url);
    println!("-------------------");
    let client = Client::new();
    let resp = client.post(url)
        .form(&[
            ("action", "my_open_play_contentbb"),
            ("buttonid", "3"),
            ("gametypeid", "3"),
            ("filterid", "6")
        ])
        .send()
        .await?;
    println!("status: {}", resp.status());
    println!("date: {}", specified_date);
    let body = Document::from_read(resp.bytes().await?.as_ref()).unwrap();
    let table_rows = body.find(Name("tr"));
    for row in table_rows {

        let game_date_td2 = row.find(Name("td")).nth(3);

        // this a check to make sure that the court value is not null
        // apply this comment to all of the if let Some() blocks
        if let Some(game_date_td2) = game_date_td2 {
            let court = game_date_td2.text();
            if court.contains(constants::BEGINNER_NORMAL) {
                let game_date_td = row.find(Name("td")).nth(1);

                if let Some(game_date_td) = game_date_td {
                    let game_date_text = game_date_td.text();
                    if game_date_text.contains(specified_date) {
                        let available_td = row.find(Name("td")).nth(6);

                        if let Some(available_td) = available_td {
                            let available_text = available_td.text().replace(" ", "").replace("\n", "");
                            if available_text.contains(constants::SOLD_OUT) {
                                print!("available: {}|", available_text);
                            } else {
                                print!("available: {}|", available_text);
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
    }
    return Ok(false); // Return Ok(()) to match the expected return type
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let specified_date: &str = "05/03";
    let mut found: Result<bool, reqwest::Error> = Ok(false);

    while !found.unwrap() {
        found = start_scrape(constants::URL, specified_date).await;
        sleep(Duration::from_secs(10)).await;
    }
}