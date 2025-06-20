use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // Start WebDriver (e.g., chromedriver --port=4444)
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to URL
    driver.get("https://www.rust-lang.org").await?;

    // Find element
    let elem = driver.find_element(By::Id("some-id")).await?;
    
    // Type text
    elem.send_keys("Rust").await?;
    
    // Click element
    elem.click().await?;

    // Get page title
    let title = driver.title().await?;
    println!("Page title is: {}", title);

    // Always explicitly close the browser
    driver.quit().await?;

    Ok(())
}