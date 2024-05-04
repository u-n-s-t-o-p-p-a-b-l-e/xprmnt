use reqwest::Error;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "http://api-open-notify.org/astros.json";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;

        match serde_json::from_str::<Value>(&body) {
            Ok(json) => {
                match serde_json::to_string_pretty(&json) {
                    Ok(formatted_json) => println!("Response:\n{}", formatted_json),
                    Err(err) => println!("Failed to format JSON: {}", err),
                }
            }
            Err(err) => {
                println!("Failed to parse JSON: {}", err);
            }
        }
    } else {
        println!("Failed to get data");
    }
    Ok(())
}
