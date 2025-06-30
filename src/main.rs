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

    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

    c.find(Locator::Css(".mw-disambig")).await?.click().await?;
    c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    c.close().await
}