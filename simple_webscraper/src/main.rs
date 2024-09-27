

// How to webscrape in Rust:
// Download the target page with reqwest.
// Parse its HTML document and get data from it with scraper.
// Export the scraped data to a CSV file.

struct Country{ // country
    name: String, // country-name
    capital: String, // country-capital
    population: String, // country-population
    area: String, // country-area
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Reqwest crate is used to get an HTML page
    // Blocking feature makes reqwest not asynchronous

    let reqwest_response = reqwest::blocking::get("https://www.scrapethissite.com/pages/simple/")?;
    let html_content_string = reqwest_response.text()?;

    // println!("{html_content_string}");  Uncomment to print the HTML page

    // Scraper crate is used to extract the HTML data

    let html_content = scraper::Html::parse_document(&html_content_string);

    // Now you should look into the HTML file and find a way to get the information you need (example: selecting classes and IDs)

    let html_country_selector = scraper::Selector::parse(".country")?;   // Create a selector
    let html_all_countries = html_content.select(&html_country_selector);   // Select HTML content that match the selector

    let mut countries: Vec<Country> = Vec::new();   // Vector of structs of Country

    for html_country in html_all_countries {    // Iterate over all the elements that have the class country

        let name_selector = scraper::Selector::parse(".country-name")?; // Selector class country-name

        let name =  html_country    // HTML country
            .select(&name_selector) // Select element with class country-name
            .next()     // .next because the element is a Struct of type Select
            .map(|x| x.text().collect::<String>()   // Make the element a String
            .trim()
            .to_owned())
            .ok_or("Country name not found")?;

        let capital_selector = scraper::Selector::parse(".country-capital")?;   // Selector class country-capital

        let capital = html_country
            .select(&capital_selector)  // Select element with class country-capital
            .next()     // .next because the element is a Struct of type Select
            .map(|x| x.text().collect::<String>()   // Make the element a String
            .trim()
            .to_owned())
            .ok_or("Country capital not found")?;

        let population_selector = scraper::Selector::parse(".country-population")?;   // Selector class country-capital

        let population = html_country
            .select(&population_selector)  // Select element with class country-capital
            .next()     // .next because the element is a Struct of type Select
            .map(|x| x.text().collect::<String>()   // Make the element a String
            .trim()
            .to_owned())
            .ok_or("Country population not found")?;

        let area_selector = scraper::Selector::parse(".country-area")?;   // Selector class country-capital

        let area = html_country
            .select(&area_selector)  // Select element with class country-capital
            .next()     // .next because the element is a Struct of type Select
            .map(|x| x.text().collect::<String>()   // Make the element a String
            .trim()
            .to_owned())
            .ok_or("Country area not found")?;  

        let country = Country {
            name,
            capital,
            population,
            area,
        };

        countries.push(country);
    }

    // Initialize the output CSV file

    let mut writer = csv::Writer::from_path("countries.csv")?;

    // Write the CSV header

    writer.write_record(&["Name", "Capital", "Population", "Area"])?;

    // Populate the file with each country

    for country in countries {

        writer.write_record(&[

            country.name,

            country.capital,

            country.population,

            country.area,

        ])?;

    }

    Ok(())
}
