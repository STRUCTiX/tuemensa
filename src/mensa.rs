extern crate serde_derive;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MensaShedhalle {
    #[serde(rename = "611")]
    the_611: The611,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct The611 {
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
    student_price: StudentPrice,
    #[serde(rename = "guestPrice")]
    guest_price: String,
    #[serde(rename = "pupilPrice")]
    pupil_price: String,
    #[serde(rename = "menuDate")]
    menu_date: String,
    menu: Vec<String>,
    meats: Vec<FiltersInclude>,
    icons: Vec<FiltersInclude>,
    #[serde(rename = "filtersInclude")]
    filters_include: Vec<FiltersInclude>,
    allergens: Vec<String>,
    additives: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum StudentPrice {
    #[serde(rename = "-")]
    Empty,
    #[serde(rename = "0,95")]
    The095,
    #[serde(rename = "1,00")]
    The100,
    #[serde(rename = "2,65")]
    The265,
    #[serde(rename = "3,10")]
    The310,
    #[serde(rename = "3,20")]
    The320,
    #[serde(rename = "3,40")]
    The340,
    #[serde(rename = "3,50")]
    The350,
}

