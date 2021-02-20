use rocket::http::RawStr;
use rocket::request::{Form, FromFormValue};
use serde::Deserialize;
use rocket_contrib::json::Json;

#[derive(Debug)]
struct Kingdom(String);

impl<'v> FromFormValue<'v> for Kingdom {
  type Error = &'v RawStr;

  fn from_form_value(form_value: &'v RawStr) -> Result<Kingdom, &'v RawStr> {
    match form_value.parse::<String>() {
      Ok(kingdom) if kingdom == "Animale" || kingdom == "Vegetabile" => Ok(Kingdom(kingdom)),
      _ => Err(form_value),
    }
  }
}

#[derive(FromForm, Debug)]
pub struct Classification {
  kingdom: Kingdom,
  genus: String,
  species: String,
}

#[post("/life", data="<input>")]
pub fn form_type(input: Form<Classification>) -> String {
  format!("Taki to oto request wysłałes: {:?}", input)
}

#[derive(Deserialize, Debug)]
pub struct Task {
  description: String,
  complete: bool,
}

#[post("/task", data="<task>")]
pub fn json_type(task: Json<Task>) -> String {
  format!("Le task: {:?}", task)
}
