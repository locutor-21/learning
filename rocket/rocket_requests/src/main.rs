#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
//        .mount("/", routes![files])
        .mount("/public", StaticFiles::from("/static"))
}

fn main() {
    rocket().launch();
}
