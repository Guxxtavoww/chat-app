#[macro_use]
extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hi"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world])
}
