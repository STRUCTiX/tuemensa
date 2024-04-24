use mensa::{Mealplan, Mensa, MensaName};
use prettytable::{row, Cell, Row, Table};

mod cli;
mod mensa;

fn main() {
    let args = cli::get_args();
    exec_arguments(&args);
}

fn exec_arguments(args: &cli::Args) {
    if args.morgenstelle {
        if let Ok(resp) = Mensa::from(MensaName::Morgenstelle) {
            exec_arg_helper(args, &resp);
        }
    }

    if args.shedhalle {
        if let Ok(resp) = Mensa::from(MensaName::Shedhalle) {
            exec_arg_helper(args, &resp);
        }
    }

    if args.prinzkarl {
        if let Ok(resp) = Mensa::from(MensaName::PrinzKarl) {
            exec_arg_helper(args, &resp);
        }
    }
}

fn exec_arg_helper(args: &cli::Args, m: &impl Mealplan) {
    if let Some(menus) = m.nth(args.days, args.vegetarian) {
        if args.plaintext {
            for i in menus.1.iter() {
                i.print_short_info();
            }
            return;
        }

        if args.oneline {
            menus.1.first().unwrap().print_very_short_info();
            return;
        }

        // Default case --> print fancy
        println!("Datum: {}", menus.0);
        println!("{}", m.name());
        table_short(
            menus
                .1
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
