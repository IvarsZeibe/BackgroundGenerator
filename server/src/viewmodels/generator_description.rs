use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneratorDescription {
    pub name: String,
    pub description: String,
}
