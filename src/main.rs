use std::time::Duration;

use thirtyfour::{By, DesiredCapabilities, WebDriver};
use tokio::time::sleep;

struct Content {
    value: String,
    pointer: String,
    endpoint: String,
}

struct Data {
    content: Vec<Content>,
}

impl Data {
    fn new() -> Self {
        Self { content: vec![] }
    }
}

const DRIVER_ADDRESS: &str = "http://localhost:9515";
// const URL: &str = "http://localhost:5173/";
const URL: &str = "https://www.spacex.com/vehicles/falcon-9/";
// const DRIVER_ADDRESS: &str = "127.0.6533.72";

#[tokio::main]
async fn main() -> Result<(), String> {
    let driver = setup_driver(DRIVER_ADDRESS).await.unwrap();

    let root = driver.find_all(By::XPath("//*")).await.unwrap();

    let data: Data = Data::new();

    let res = driver
        .execute(
            r#"
        window.scrollTo(0, 100000)
        return true
    "#,
            Vec::new(),
        )
        .await
        .unwrap();

    let resp = res.convert::<bool>().unwrap();

    println!("{resp}");

    for r in root {
        let tag_name = r.tag_name().await.unwrap();
        let text = r.text().await;
        // match text {
        //     Ok(text) => {
        //         let content = Content {
        //             value: text,
        //             pointer:
        //         }
        //         data.content.push(content)
        //     },
        //     Err(err) => ()
        // }
        println!(
            "{} -> {}: {} - {}",
            r.tag_name().await.unwrap(),
            r.text().await.unwrap_or(String::from("No text")),
            r.class_name()
                .await
                .unwrap()
                .unwrap_or(String::from("No classname")),
            r.attr("style")
                .await
                .unwrap()
                .unwrap_or(String::from("No style"))
        )
    }

    driver.quit().await.unwrap();

    Ok(())
}

async fn setup_driver(address: &str) -> Result<WebDriver, String> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(address, caps).await.unwrap();

    driver.goto(URL).await.unwrap();

    sleep(Duration::from_millis(2500)).await;

    Ok(driver)
}
