use mensa::Mealplan;

mod mensa;

//use crate::mensa::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shedhalle = mensa::Mensa::from(mensa::MensaName::Shedhalle).await?;
    if let mensa::Mensa::Shedhalle(resp) = shedhalle {
        //println!("{:#?}", resp);
        //println!("{}", resp.today());
        for a in resp.today().iter() {
            a.print_very_short_info();
        }

    }
    Ok(())
}


