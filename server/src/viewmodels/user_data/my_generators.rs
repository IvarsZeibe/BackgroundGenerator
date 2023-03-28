use chrono::NaiveDateTime;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneratorDescription {
	pub id: String,
	pub name: String,
	pub description: String,
	#[serde(rename = "dateCreated")]
	pub date_created: NaiveDateTime,
	#[serde(rename = "dateModified")]
	pub date_modified: NaiveDateTime,
	#[serde(rename = "generatorType")]
	pub generator_type: String,
	#[serde(rename = "generatorTypeCode")]
	pub generator_code: String,
	pub image: Vec<u8>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MyGenerators {
	#[serde(rename = "maxGenerators")]
	pub max_generators: i32,
	#[serde(rename = "generatorDescriptions")]
	pub generator_descriptions: Vec<GeneratorDescription>,
}
