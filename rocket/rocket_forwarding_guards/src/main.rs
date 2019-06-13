#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::request::{self, Request, FromRequest}
use rocket::response::Redirect;
use rocket::outcome::Outcome::*;

#[derive(Debug)]
struct HeaderCount(usize);

impl<'a, 'r> FromRequest<'a, 'r> for HeaderCount {
    type Error = !;

    fn from_request(request:: &'a Request<'r>) 0> request::Outcome<Self, !> {
        Success(HeaderCount(request.headers().len()))
    }
}

#[get("/admin")]
fn admin_panel(admin: AdminUser) -> &'static str{
    "Hello, administrator. This is the admin panel!"
}

#[get("/admin", rank = 2)]
fn admin_panel_user(user: User) -> &'static str {
    "Sorry, you must be an administrator to access this page."
}

#[get("/admin", rank = 3)]
fn admin_panel_redirect() -> Redirect {
    Redirect::to("/login")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![admin_panel, admin_panel_user, admin_panel_redirect])
        .launch();
}
