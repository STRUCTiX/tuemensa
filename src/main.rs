use mensa::Mealplan;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

mod mensa;

//use crate::mensa::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shedhalle = mensa::Mensa::from(mensa::MensaName::Shedhalle);
    let morgenstelle = mensa::Mensa::from(mensa::MensaName::Morgenstelle);
    if let mensa::Mensa::Shedhalle(resp) = shedhalle.await? {
        let data = resp.today().iter().map(|&x| x.get_short_info()).collect::<Vec<(&str, String, &str)>>();
        table_short(data).await;
    }

    if let mensa::Mensa::Morgenstelle(resp) = morgenstelle.await? {
        let data = resp.today().iter().map(|&x| x.get_short_info()).collect::<Vec<(&str, String, &str)>>();
        table_short(data).await;
    }

    Ok(())
}

async fn table_short(data: Vec<(&str, String, &str)>) {
    let mut table = Table::new();

    //let mut data_cells = data.iter().map(|x| vec![Cell::new(x.0), Cell::new(&x.1), Cell::new(x.2)]).collect::<Vec<Vec<Cell>>>();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        //.set_width(40)
        .set_header(vec!["Art", "Beschreibung", "Preis (Student)"]);
    
    for d in data.iter() {
        table.add_row(vec![Cell::new(d.0), Cell::new(&d.1), Cell::new(d.2)]);
    }
    // Set the default alignment for the third column to right
    let column = table.column_mut(2).expect("Our table has three columns");
    column.set_cell_alignment(CellAlignment::Right);

    println!("{table}");
}


