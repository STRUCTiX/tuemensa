use clap::Parser;

///tuemensa is a simple command-line tool designed to retrieve the current meal plans for the canteens at Eberhard Karls Universität Tübingen.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Display the meal plan for Mensa Morgenstelle
    #[arg(short, long, default_value_t = false)]
    pub morgenstelle: bool,

    /// Display the meal plan for Mensa Wilhelmstraße
    #[arg(short, long, default_value_t = false)]
    pub wilhelmstrasse: bool,

    /// Display the meal plan for Mensa Prinz Karl
    #[arg(short, long, default_value_t = false)]
    pub prinzkarl: bool,

    /// Output the meal plan in plain text format
    #[arg(long, default_value_t = false)]
    pub plaintext: bool,

    /// Use very short format (oneline)
    #[arg(short, long, default_value_t = false)]
    pub oneline: bool,

    /// Specify the number of days ahead to display (valid values: 0-7)
    #[arg(short, long, default_value_t = 0)]
    pub days: u8,

    /// Display only the vegetarian meal options
    #[arg(short, long, default_value_t = false)]
    pub vegetarian: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
