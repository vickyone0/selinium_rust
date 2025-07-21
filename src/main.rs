use fantoccini::{ClientBuilder, Locator};
use serde_json::{Map, Value};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    // Build the capabilities as a Map<String, Value>
    let mut caps = Map::new();
    let mut chrome_opts = Map::new();
    chrome_opts.insert("binary".to_string(), Value::String("/usr/bin/brave".to_string())); // Update path if needed
    caps.insert(
        "goog:chromeOptions".to_string(),
        Value::Object(chrome_opts),
    );

    let c = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:9515")
        .await
        .expect("failed to connect to WebDriver");

    c.goto("https://noscura.co.in/homePage").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://noscura.co.in/homePage");

    // Wait for the semantics tree to load (increase sleep if needed)
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Click the About button (try aria-label, update if needed)
    let about_btn = c
        .find(Locator::Css(r#"flt-semantics-host [aria-label="About"]"#))
        .await?;

    about_btn.click().await?;

    c.close().await
}