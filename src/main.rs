mod mensa;

//use crate::mensa::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.my-stuwe.de//wp-json/mealplans/v1/canteens/611?lang=de")
        .await?
        .json::<mensa::MensaShedhalle>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
