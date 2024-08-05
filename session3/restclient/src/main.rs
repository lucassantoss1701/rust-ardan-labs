#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=38.95178&longitude=92.3341&current_weather=true";
    let response = reqwest::get(URL).await?;
    let weather: serde_json::Value = response.json().await?;


    println!("{weather:#?}");

    Ok(())
}
