// use std::fs::File;
// use std::io::{self, Read, Write};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct MyData {
//     name: String,
//     age: u32,
// }

// fn main() -> io::Result<()> {
//     // Create some initial data
//     let initial_data = MyData {
//         name: String::from("John"),
//         age: 30,
//     };

//     // Serialize the initial data to JSON and write it to a file
//     let json_data = serde_json::to_string_pretty(&initial_data)?;

//     let mut file = File::create("data.json")?;
//     file.write_all(json_data.as_bytes())?;

//     println!("Initial JSON data:\n{}\n", json_data);

//     // Prompt the user for a new name
//     println!("Enter a new name:");
//     let mut new_name = String::new();
//     io::stdin().read_line(&mut new_name)?;

//     // Read the JSON file and deserialize its content
//     let mut file = File::open("data.json")?;
//     let mut file_content = String::new();
//     file.read_to_string(&mut file_content)?;

//     let mut data: MyData = serde_json::from_str(&file_content)?;

//     // Update the name field with the new input
//     data.name = new_name.trim().to_string();

//     // Serialize the updated data back to JSON
//     let updated_json = serde_json::to_string_pretty(&data)?;

//     // Write the updated JSON back to the file
//     let mut file = File::create("data.json")?;
//     file.write_all(updated_json.as_bytes())?;

//     println!("Updated JSON data:\n{}\n", updated_json);

//     Ok(())
// }


// use std::fs::File;
// use std::io::Write;
// use serde::{Serialize};
// use scraper::{Html, Selector};
// use reqwest;

// #[derive(Serialize)]
// struct Movie {
//     number: u32,
//     name: String,
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Hello, world!");

//     let response = reqwest::blocking::get(
//         "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100"
//     )?
//     .text()?;

//     let document = Html::parse_document(&response);
//     let title_selector = Selector::parse("h3.lister-item-header > a").unwrap();
//     let titles = document.select(&title_selector).map(|x| x.inner_html());

//     let movies: Vec<Movie> = titles.zip(1..101).map(|(item, number)| Movie { number, name: item }).collect();

//     // Serialize the data to JSON
//     let json_data = serde_json::to_string_pretty(&movies)?;

//     // Write the JSON data to a file
//     let mut file = File::create("movies.json")?;
//     file.write_all(json_data.as_bytes())?;

//     println!("Data written to movies.json");

//     Ok(())
// }


// use std::fs::File;
// use std::io::Write;
// use serde::Serialize;
// use scraper::{Html, Selector};
// use reqwest;

// #[derive(Serialize)]
// struct Movie {
//     number: u32,
//     name: String,
//     link: String, // Add a field to store the link
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Hello, world!");

//     let response = reqwest::blocking::get(
//         "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100"
//     )?
//     .text()?;

//     let document = Html::parse_document(&response);
//     let title_selector = Selector::parse("h3.lister-item-header > a").unwrap();
//     let titles = document.select(&title_selector);

//     let movies: Vec<Movie> = titles.enumerate().map(|(number, title_element)| {
//         let name = title_element.inner_html();
//         let link = title_element.value().attr("href").unwrap_or_default();

//         Movie {
//             number: (number + 1) as u32,
//             name,
//             link: link.to_string(), // Store the link in the struct
//         }
//     }).collect();

//     // Serialize the data to JSON
//     let json_data = serde_json::to_string_pretty(&movies)?;

//     // Write the JSON data to a file
//     let mut file = File::create("movies.json")?;
//     file.write_all(json_data.as_bytes())?;

//     println!("Data written to movies.json");

//     // Print the links
//     for movie in &movies {
//         println!("{}. {} - Link: {}", movie.number, movie.name, movie.link);
//     }

//     Ok(())
// }



// This is what works.
// use std::fs::File;
// use std::io::Write;
// use serde::Serialize;
// use scraper::{Html, Selector};
// use reqwest;

// #[derive(Serialize)]
// struct Movie {
//     number: u32,
//     name: String,
//     link: String, // Store the relative link
//     full_link: String, // Store the complete URL
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Hello, world!");

//     let response = reqwest::blocking::get(
//         "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100"
//     )?
//     .text()?;

//     let base_url = "https://www.imdb.com"; // Base URL to prepend

//     let document = Html::parse_document(&response);
//     let title_selector = Selector::parse("h3.lister-item-header > a").unwrap();
//     let titles = document.select(&title_selector);

//     let movies: Vec<Movie> = titles.enumerate().map(|(number, title_element)| {
//         let name = title_element.inner_html();
//         let link = title_element.value().attr("href").unwrap_or_default();
//         let full_link = format!("{}{}", base_url, link); // Create complete URL

//         Movie {
//             number: (number + 1) as u32,
//             name,
//             link: link.to_string(), // Store the relative link
//             full_link, // Store the complete URL
//         }
//     }).collect();

//     // Serialize the data to JSON
//     let json_data = serde_json::to_string_pretty(&movies)?;

//     // Write the JSON data to a file
//     let mut file = File::create("movies.json")?;
//     file.write_all(json_data.as_bytes())?;

//     println!("Data written to movies.json");

//     // Print the links
//     for movie in &movies {
//         println!("{}. {} - Link: {}", movie.number, movie.name, movie.full_link);
//     }

//     Ok(())
// }


use reqwest;
use scraper;

fn main() {
    let response = reqwest::blocking::get(
        "https://download.blender.org/release/",
    )
    .unwrap()
    .text()
    .unwrap();
    println!("{response}");

    // let document = scraper::Html::parse_document(&response);
    // println!("{:?}", document);
}

// fn main() {
//     // download the target HTML document
//     let response = reqwest::blocking::get("https://scrapeme.live/shop/");
//     // get the HTML content from the request response
//     // and print it
//     let html_content = response.unwrap().text().unwrap();
//     println!("{html_content}");
// }












// fn main() {
//     println!("Hello, world!");

//     let response = reqwest::blocking::get(
//         "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100"
//     ).unwrap()
//     .text()
//     .unwrap();
    
//     let document = scraper::Html::parse_document(&response);
//     let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap(); 
//     let titles = document.select(&title_selector).map(|x| x.inner_html());
//     titles
//         .zip(1..101)
//         .for_each(|(item, number)| println!("{}. {}", number, item));
// }
