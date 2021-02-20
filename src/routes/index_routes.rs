use rocket::http::RawStr;
use rocket::http::Status;
use rocket::request::{FromParam, FromRequest, self, Outcome, Request};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/echo/<word>")]
pub fn echo(word: &RawStr) -> String {
  format!("Echo: {}!", word.as_str())
}

#[get("/params/<custom>")]
pub fn custom_params(custom: MyOwnParam) -> String {
  format!("Param key: {}, param value: {}", custom.original, custom.transformed)
}

#[get("/query?<name>&<age>")]
pub fn query_params(name: String, age: u32) -> String {
  format!("I'm {} and I'm {} years old.", name, age)
}

#[get("/secret")]
pub fn secret_path(secret: Secret) -> &'static str {
  "Like a boss!"
}

pub struct MyOwnParam<'r> {
  original: &'r str,
  transformed: String
}

impl<'r> FromParam<'r> for MyOwnParam<'r> {
  type Error = &'r RawStr;

  fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
    match param.as_str() {
      "love" => Ok(Self {
        original: param,
        transformed: String::from("<3")
      }),
      _ => Ok(Self {
        original: param,
        transformed: String::from("No love for you!")
      })
    }
  }
}

pub struct Secret(String);

#[derive(Debug)]
pub enum SecretError {
  YouStupid,
  JesusDoesntLoveYou,
  StopDrinking,
}

impl<'a, 'r> FromRequest<'a, 'r> for Secret {
  type Error = SecretError;

  fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    let keys: Vec<_> = req.headers().get("x-secret").collect();
    match keys.len() {
      0 => Outcome::Failure((Status::BadRequest, SecretError::YouStupid)),
      1 if Self::does_jesus_love_you(keys[0]) => Outcome::Success(Secret(keys[0].to_string())),
      1 => Outcome::Failure((Status::BadRequest, SecretError::JesusDoesntLoveYou)),
      _ => Outcome::Failure((Status::BadRequest, SecretError::StopDrinking))
    }
  }
}

impl Secret {
  fn does_jesus_love_you(secret_value: &str) -> bool {
    secret_value == "Jesus"
  }
}