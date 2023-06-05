mod post;

#[macro_use]
extern crate rocket;

use std::io::prelude::*;
use std::io::BufReader;

use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use serde::{Deserialize, Serialize};

static mut ACCESS_KEY_REQUIRED: bool = true;
const INDEX_REDIRECT: &'static str = "https://ivabus.dev";
const INDEX_WITH_AD: bool = true;

#[derive(Deserialize, Serialize, Clone)]
struct Alias {
	url: String,
	alias: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	redirect_with_ad: Option<bool>,
}

fn read_aliases() -> Vec<Alias> {
	if !std::path::Path::new("./alias.json").exists() {
		let mut file = std::fs::File::create("./alias.json").unwrap();
		file.write_all(b"[]").unwrap();
		return vec![];
	}
	if std::fs::File::open("./alias.json").unwrap().metadata().unwrap().len() == 0 {
		let mut file = std::fs::File::options().write(true).open("./alias.json").unwrap();
		file.write_all(b"[]").unwrap();
		return vec![];
	}
	let file = std::fs::File::open("./alias.json").unwrap();
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents).unwrap();
	let aliases_list: Vec<Alias> = serde_json::from_str(&contents).unwrap();
	aliases_list
}

#[get("/404")]
fn not_found() -> Status {
	Status::NotFound
}

#[get("/<page>")]
async fn get_page(page: String) -> Result<Redirect, RawHtml<String>> {
	let mut decoded_page = String::new();
	url_escape::decode_to_string(page, &mut decoded_page);
	let alias_list = read_aliases();
	for i in alias_list {
		if i.alias == decoded_page {
			if let Some(red) = i.redirect_with_ad {
				if red {
					let mut redirect = String::new();
					let mut file = std::fs::File::open("./redirect.html").unwrap();
					file.read_to_string(&mut redirect).unwrap();
					return Err(RawHtml(redirect.replace("#REDIRECT#", i.url.as_str())));
				}
			}
			return Ok(Redirect::to(i.url));
		}
	}
	Ok(Redirect::to("/404"))
}

#[get("/")]
async fn get_index() -> Result<Redirect, RawHtml<String>> {
	if INDEX_WITH_AD {
		let mut redirect = String::new();
		let mut file = std::fs::File::open("./redirect.html").unwrap();
		file.read_to_string(&mut redirect).unwrap();
		Err(RawHtml(redirect.replace("#REDIRECT#", INDEX_REDIRECT)))
	} else {
		Ok(Redirect::to(INDEX_REDIRECT))
	}
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	if !std::path::Path::new("./access_keys").exists() {
		eprintln!("No ./access_keys found. Falling back to no authorization");
		eprintln!("Continue? (press enter or ctrl-c to exit)");
		let mut s = String::new();
		std::io::stdin().read_line(&mut s).unwrap();
		unsafe {
			ACCESS_KEY_REQUIRED = false;
		}
	} else if std::fs::File::open("./access_keys").unwrap().metadata().unwrap().len() == 0 {
		eprintln!("No keys in ./access_keys found. Falling back to no authorization");
		eprintln!("Continue? (press enter or ctrl-c to exit)");
		let mut s = String::new();
		std::io::stdin().read_line(&mut s).unwrap();
		unsafe {
			ACCESS_KEY_REQUIRED = false;
		}
	}

	let _rocket = rocket::build()
		.mount(
			"/",
			routes![
				not_found,
				post::create_alias,
				post::get_aliases,
				post::remove_alias,
				get_page,
				get_index
			],
		)
		.launch()
		.await?;

	Ok(())
}
