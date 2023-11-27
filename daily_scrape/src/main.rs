// use reqwest::Client;
// use scraper::{Html, Selector};

// async fn scrape() -> Result<(), reqwest::Error> {
//     println!("Hello, world!");

//     let http_client = Client::new();
//     let http_result = http_client.get("https://builder.blender.org/download/daily/").send().await?;

//     if let Ok(body) = http_result.text().await {
//         let document = Html::parse_document(&body);
//         let selector = Selector::parse(".hero-content").unwrap();

//         // for div in document.select(&selector) {
//         //     println!("{:?}", div.text().collect::<String>());
//         // }
//         for div in document.select(&selector) {
//             println!("{:?}", div.html());
//         }
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     if let Err(err) = scrape().await {
//         eprintln!("Error: {:?}", err);
//     }
// }


// use reqwest::Client;
// use scraper::{Html, Selector};

// async fn scrape() -> Result<(), reqwest::Error> {
//     println!("Hello, world!");

//     let http_client = Client::new();
//     let http_result = http_client.get("https://builder.blender.org/download/daily/").send().await?;

//     if let Ok(body) = http_result.text().await {
//         let document = Html::parse_document(&body);
//         let selector = Selector::parse(".hero-content").unwrap();

//         for div in document.select(&selector) {
//             println!("{:?}", div.html());
//         }
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     if let Err(err) = scrape().await {
//         eprintln!("Error: {:?}", err);
//     }
// }


// use reqwest::Client;
// use scraper::{Html, Selector};

// async fn scrape() -> Result<(), reqwest::Error> {
//     println!("Hello, world!");

//     let http_client = Client::new();
//     let http_result = http_client.get("https://builder.blender.org/download/daily/").send().await?;

//     if let Ok(body) = http_result.text().await {
//         let document = Html::parse_document(&body);
//         let selector = Selector::parse(".builds-list-container.platform-windows, .builds-list-container.platform-linux, .builds-list-container.platform-darwin").unwrap();

//         for div in document.select(&selector) {
//             println!("{:?}", div.html());
//         }
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     if let Err(err) = scrape().await {
//         eprintln!("Error: {:?}", err);
//     }
// }


// use reqwest::Client;
// use scraper::{Html, Selector};

// async fn scrape() -> Result<(), reqwest::Error> {
//     println!("Hello, world!");

//     let http_client = Client::new();
//     let http_result = http_client.get("https://builder.blender.org/download/daily/").send().await?;

//     if let Ok(body) = http_result.text().await {
//         let document = Html::parse_document(&body);
//         let selector = Selector::parse(".builds-list-container.platform-windows li, .builds-list-container.platform-linux li, .builds-list-container.platform-darwin li").unwrap();

//         for li in document.select(&selector) {
//             println!("{:?}", li.html());
//         }
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     if let Err(err) = scrape().await {
//         eprintln!("Error: {:?}", err);
//     }
// }


// use reqwest::Client;
// use scraper::{Html, Selector};

// async fn scrape() -> Result<(), reqwest::Error> {
//     println!("Hello, world!");

//     let http_client = Client::new();
//     let http_result = http_client.get("https://builder.blender.org/download/daily/").send().await?;

//     if let Ok(body) = http_result.text().await {
//         let document = Html::parse_document(&body);
//         let selector = Selector::parse(".builds-list-container.platform-windows li, .builds-list-container.platform-linux li, .builds-list-container.platform-darwin li").unwrap();

//         for li in document.select(&selector) {
//             // Filter out list items with style="display:none;"
//             let style = li.value().attr("style");
//             if let Some(style_attr) = style {
//                 if !style_attr.contains("display:none;") {
//                     println!("{:?}", li.html());
//                 }
//             } else {
//                 println!("{:?}", li.html());
//             }
//         }
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     if let Err(err) = scrape().await {
//         eprintln!("Error: {:?}", err);
//     }
// }


// use reqwest::Client;
// use scraper::{Html, Selector};

// async fn scrape() -> Result<(), reqwest::Error> {
//     println!("Hello, world!");

//     let http_client = Client::new();
//     let http_result = http_client.get("https://builder.blender.org/download/daily/").send().await?;

//     if let Ok(body) = http_result.text().await {
//         let document = Html::parse_document(&body);
//         let selector = Selector::parse(".builds-list-container.platform-windows li:not([style*='display:none;']), .builds-list-container.platform-linux li:not([style*='display:none;']), .builds-list-container.platform-darwin li:not([style*='display:none;'])").unwrap();

//         for li in document.select(&selector) {
//             let a_selector = Selector::parse("a.b-version, a.b-variant, a.b-reference").unwrap();
//             let div_selector = Selector::parse("div.b-date, div.b-arch").unwrap();

//             // Print anchor tags matching the specified classes
//             for a in li.select(&a_selector) {
//                 println!("{:?}", a.html());
//             }

//             // Print divs matching the specified classes
//             for div in li.select(&div_selector) {
//                 println!("{:?}", div.html());
//             }
//         }
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     if let Err(err) = scrape().await {
//         eprintln!("Error: {:?}", err);
//     }
// }


use reqwest::Client;
use scraper::{Html, Selector};

async fn scrape() -> Result<(), reqwest::Error> {
    println!("Hello, world!");

    let http_client = Client::new();
    let http_result = http_client.get("https://builder.blender.org/download/daily/").send().await?;

    if let Ok(body) = http_result.text().await {
        let document = Html::parse_document(&body);
        let selector = Selector::parse(".builds-list-container.platform-windows li:not([style*='display:none;']), .builds-list-container.platform-linux li:not([style*='display:none;']), .builds-list-container.platform-darwin li:not([style*='display:none;'])").unwrap();

        let mut blender_version: String = String::new();
        let mut blender_variant: String;
        let mut blender_architecture: String;
        let mut blender_download_link: String;
        let mut blender_commit_link: String;
        let mut blender_commit_hash: String;
        let mut blender_release_date: String;
        let mut blender_file_type: String;

        for li in document.select(&selector) {
            let a_selector = Selector::parse("a.b-version, a.b-variant, a.b-reference").unwrap();
            let div_selector = Selector::parse("div.b-date, div.b-arch").unwrap();

            // Decern the correct data 
            for a in li.select(&a_selector) {
                if a.value().attr("class").unwrap_or("").contains("b-version") {
                    // Get the download link
                    blender_download_link = a.value().attr("href").unwrap_or("").to_string();
                    match &blender_download_link {
                        link if link.ends_with(".zip") => {
                            // Handle .zip case
                            println!("Blender Download Link is a .zip file: {}", link);
                            blender_file_type = "zip".to_string();
                        }
                        link if link.ends_with(".dmg") => {
                            // Handle .dmg case
                            println!("Blender Download Link is a .dmg file: {}", link);
                            blender_file_type = "dmg".to_string();
                        }
                        link if link.ends_with(".tar.xz") => {
                            // Handle .tar.xz case
                            println!("Blender Download Link is a .tar.xz file: {}", link);
                            blender_file_type = "tar.xz".to_string();
                        }
                        _ => {
                            println!("Skipped");
                            break;
                        }
                    }

                    let b_version = a.text().collect::<String>();
                    let parts: Vec<&str> = b_version.split(' ').collect();
                    
                    // Get the version
                    if parts.len() > 1 {
                        blender_version = parts[1].to_string();
                    }
                    // Print or process other extracted data...
                    println!("Blender Version: {}", blender_version);
                    println!("Blender Download Link: {}", blender_download_link);
                    println!("Blender File Type: {}", blender_file_type)
                    
                } else if a.value().attr("class").unwrap_or("").contains("b-variant") {
                    blender_variant = a.text().collect::<String>();
                    println!("Blender Variant: {}", blender_variant);
                } else if a.value().attr("class").unwrap_or("").contains("b-reference") {
                    //Commit scrape
                    blender_commit_hash = a.text().collect::<String>();
                    println!("{}", blender_commit_hash);
                    blender_commit_link = a.value().attr("href").unwrap_or("").to_string(); 
                    println!("{}", blender_commit_link);

                }
            }
            for div in li.select(&div_selector) {
                if div.value().attr("class").unwrap_or("").contains("b-date") {
                    if let Some(title) = div.value().attr("title") {
                        
                        // Assign to your variable if needed
                        blender_release_date = title.to_string();
                        println!("Title: {}", blender_release_date);
                    }
                } else if div.value().attr("class").unwrap_or("").contains("b-arch") {
                    let b_architecture = div.text().collect::<String>();
                    let parts: Vec<&str> = b_architecture.split(' ').collect();
                    if parts.len() >= 2 {
                        let remaining_words: Vec<&str> = parts[1..].to_vec();
                        let joined_string = remaining_words.join(" ");
                        
                        blender_architecture = joined_string;
                        println!("{}", blender_architecture);
                    }




                }
                
            }
        }

        
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = scrape().await {
        eprintln!("Error: {:?}", err);
    }
}


// use reqwest::Client;
// use scraper::{Html, Selector};

// async fn scrape() -> Result<(), reqwest::Error> {
//     println!("Hello, world!");

//     let http_client = Client::new();
//     let http_result = http_client.get("https://builder.blender.org/download/daily/").send().await?;

//     if let Ok(body) = http_result.text().await {
//         let document = Html::parse_document(&body);
//         let selector = Selector::parse(".builds-list-container.platform-windows li:not([style*='display:none;']), .builds-list-container.platform-linux li:not([style*='display:none;']), .builds-list-container.platform-darwin li:not([style*='display:none;'])").unwrap();

//         let mut blender_version: String = String::new();
//         let mut blender_variant: String;
//         let mut blender_architecture: String;
//         let mut blender_download_link: String;
//         let mut blender_commit_link: String;
//         let mut blender_commit_hash: String;
//         let mut blender_release_date: String;
//         let mut blender_file_type: String;

//         for li in document.select(&selector) {
//             let a_selector = Selector::parse("a.b-version, a.b-variant, a.b-reference").unwrap();
//             let div_selector = Selector::parse("div.b-date, div.b-arch").unwrap();

//             // Check for b-version and its download link ending with specific extensions
//             let mut should_process = false;
//             for a in li.select(&a_selector) {
//                 if a.value().attr("class").unwrap_or("").contains("b-version") {
//                     blender_version = a.text().collect::<String>();
//                     let href = a.value().attr("href").unwrap_or("");
                    
//                     if href.ends_with(".zip") || href.ends_with(".dmg") || href.ends_with(".tar.xz") {
//                         blender_download_link = href.to_string();
//                         should_process = true;
//                     }
//                 } else if a.value().attr("class").unwrap_or("").contains("b-variant") {
//                     blender_variant = a.text().collect::<String>();
//                 } else {
//                     // Handle other cases
//                 }
//             }
//         }

//         // Other logic...
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     if let Err(err) = scrape().await {
//         eprintln!("Error: {:?}", err);
//     }
// }