use chrono::{Datelike, Local};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use mensa::Mealplan;

mod cli;
mod mensa;

//use crate::mensa::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::get_args();
    exec_arguments(&args).await?;
    Ok(())
}

async fn exec_arguments(args: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    let shedhalle = mensa::Mensa::from(mensa::MensaName::Shedhalle);
    let morgenstelle = mensa::Mensa::from(mensa::MensaName::Morgenstelle);

    if args.morgenstelle {
        if let mensa::Mensa::Morgenstelle(resp) = morgenstelle.await? {
            exec_arg_helper(args, &resp);
        }
    }

    if args.shedhalle {
        if let mensa::Mensa::Shedhalle(resp) = shedhalle.await? {
            exec_arg_helper(args, &resp);
        }
    }
    Ok(())
}

fn exec_arg_helper(args: &cli::Args, m: &dyn mensa::Mealplan) {
    if let Some(menus) = m.nth(args.days, args.vegetarian) {
        if args.plaintext {
            for i in menus.iter() {
                i.print_short_info();
            }
            return;
        }

        if args.oneline {
            menus.first().unwrap().print_very_short_info();
            return;
        }

        // Default case --> print fancy
        if let Some(dt) = Local::now().checked_add_days(chrono::Days::new(args.days as u64)) {
            println!("Datum: {}", dt.date_naive());
        }
        println!("{}", m.name());
        table_short(
            menus
                .iter()
                .map(|&x| x.get_short_info())
                .collect::<Vec<(&str, String, &str)>>(),
        );
    }
}

fn table_short(data: Vec<(&str, String, &str)>) {
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
