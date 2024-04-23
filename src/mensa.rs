extern crate serde_derive;
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ureq::{Agent, Error};

pub enum MensaName {
    Shedhalle,
    Morgenstelle,
}

pub trait Mealplan {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn today(&self) -> Vec<&Menu>;
    fn nth(&self, days: u8, vegetarian: bool) -> Option<Vec<&Menu>>;
}

fn get_nth_date(days: u8) -> Option<chrono::DateTime<Local>> {
    if days > 7 {
        return None;
    }

    if let Some(dt) = Local::now().checked_add_days(chrono::Days::new(days as u64)) {
        return match dt.weekday() {
            chrono::Weekday::Sat => dt.checked_add_days(chrono::Days::new(2)),
            chrono::Weekday::Sun => dt.checked_add_days(chrono::Days::new(1)),
            _ => Some(dt),
        };
    }
    None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mensa {
    #[serde(alias = "621")]
    #[serde(rename = "611")]
    canteen: Canteen,
}

impl Mensa {
    pub fn from(name: MensaName) -> Result<Mensa, Error> {
        let agent: Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();

        let canteen_id = match name {
            MensaName::Shedhalle => 611,
            MensaName::Morgenstelle => 621,
        };

        let url =
            format!("https://www.my-stuwe.de//wp-json/mealplans/v1/canteens/{canteen_id}?lang=de");

        Ok(agent.get(&url).call()?.into_json::<Mensa>()?)
    }

    fn _print(&self) {
        println!("{:#?}", self);
    }
}

impl Mealplan for Mensa {
    fn id(&self) -> &str {
        &self.canteen.canteen_id
    }

    fn name(&self) -> &str {
        &&self.canteen.canteen
    }

    fn today(&self) -> Vec<&Menu> {
        let local = format!("{}", Local::now().format("%Y-%m-%d"));
        self.canteen
            .menus
            .iter()
            .filter(|&x| x.menu_date == local)
            .collect()
    }

    fn nth(&self, days: u8, vegetarian: bool) -> Option<Vec<&Menu>> {
        match get_nth_date(days) {
            Some(dt) => {
                let local = format!("{}", dt.format("%Y-%m-%d"));
                if vegetarian {
                    Some(
                        self.canteen
                            .menus
                            .iter()
                            .filter(|&x| x.menu_date == local && x.menu_line.contains("veg"))
                            .collect(),
                    )
                } else {
                    Some(
                        self.canteen
                            .menus
                            .iter()
                            .filter(|&x| x.menu_date == local)
                            .collect(),
                    )
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Canteen {
    #[serde(rename = "canteenId")]
    canteen_id: String,
    canteen: String,
    menus: Vec<Menu>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    id: String,
    #[serde(rename = "menuLine")]
    menu_line: String,
    photo: Photo,
    #[serde(rename = "studentPrice")]
    student_price: String,
    #[serde(rename = "guestPrice")]
    guest_price: String,
    #[serde(rename = "pupilPrice")]
    pupil_price: String,
    #[serde(rename = "menuDate")]
    menu_date: String,
    menu: Vec<String>,
    meats: Vec<String>,
    icons: Vec<String>,
    #[serde(rename = "filtersInclude")]
    filters_include: Vec<FiltersInclude>,
    allergens: Vec<String>,
    additives: Vec<String>,
}

impl Menu {
    pub fn _print(&self) {
        println!("{:#?}", self);
    }

    pub fn print_short_info(&self) {
        println!(
            "{}: {}, {}€",
            self.menu_line,
            self.menu.join(", "),
            self.student_price
        );
    }

    pub fn print_very_short_info(&self) {
        if let Some(name) = self.menu.first() {
            println!("{}", name);
        }
    }

    pub fn get_short_info(&self) -> (&str, String, &str) {
        (&self.menu_line, self.menu.join(", "), &self.student_price)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    thumbnail: String,
    medium: String,
    large: String,
    full: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FiltersInclude {
    F,
    #[serde(rename = "Vegan")]
    FiltersIncludeVegan,
    G,
    #[serde(rename = "mensaVital")]
    MensaVital,
    R,
    S,
    V,
    #[serde(rename = "vegan")]
    Vegan,
}
