use chrono::Local;
use mensa::{Mealplan, Mensa, MensaName};
use prettytable::{row, Cell, Row, Table};

mod cli;
mod mensa;

fn main() {
    let args = cli::get_args();
    exec_arguments(&args);
}

fn exec_arguments(args: &cli::Args) {
    let shedhalle = Mensa::from(MensaName::Shedhalle);
    let morgenstelle = Mensa::from(MensaName::Morgenstelle);

    if args.morgenstelle {
        if let Ok(resp) = morgenstelle {
            exec_arg_helper(args, &resp);
        }
    }

    if args.shedhalle {
        if let Ok(resp) = shedhalle {
            exec_arg_helper(args, &resp);
        }
    }
}

fn exec_arg_helper(args: &cli::Args, m: &impl Mealplan) {
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

    table.add_row(row!["Art", "Beschreibung", "Preis (Student)"]);

    for d in data.iter() {
        table.add_row(Row::new(vec![
            Cell::new(d.0),
            Cell::new(&d.1),
            Cell::new(d.2).style_spec("r"),
        ]));
    }

    table.printstd();
}
