use clap::Parser;

/// tuemensa is a simple cli tool to retrieve the current meal plan.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Show Mensa Morgenstelle
   #[arg(short, long, default_value_t = false)]
   pub morgenstelle: bool,

   /// Show Mensa Shedhalle
   #[arg(short, long, default_value_t = false)]
   pub shedhalle: bool,

   /// Format as plain text
   #[arg(short, long, default_value_t = false)]
   pub plaintext: bool,

   /// Use very short format (oneline)
   #[arg(short, long, default_value_t = false)]
   pub oneline: bool,

   /// Offset of days in the future (valid inputs 0-7)
   #[arg(short, long, default_value_t = 0)]
   pub days: u8,
}

pub fn get_args() -> Args {
    Args::parse()
}
