#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;
mod catchers;

fn main() {
    rocket::ignite()
    .mount("/", routes![
        routes::index_routes::echo,
        routes::index_routes::custom_params,
        routes::index_routes::index, 
        routes::index_routes::query_params,
        routes::index_routes::secret_path
    ])
    .mount("/forms", routes![
        routes::forms::form_type,
        routes::forms::json_type,
    ])
    .register(catchers![catchers::not_found])
    .launch();
}