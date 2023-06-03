#[macro_use]
extern crate rocket;

use std::io::prelude::*;
use std::io::BufReader;

use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::Redirect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct CreateAliasRequest {
	url: String,
	access_key: Option<String>,
	alias: Option<String>,
}

static mut ACCESS_KEY_REQUIRED: bool = true;
const INDEX_REDIRECT: &'static str = "https://ivabus.dev";

#[derive(Deserialize, Serialize, Clone)]
struct Alias {
	url: String,
	alias: String,
}

fn read_alias() -> Vec<Alias> {
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
	let alias_list: Vec<Alias> = serde_json::from_str(&contents).unwrap();
	alias_list
}

#[post("/post", data = "<data>")]
fn create_alias(data: &RawStr) -> (Status, String) {
	let data: CreateAliasRequest = match serde_json::from_str(&data.to_string()) {
		Ok(req) => req,
		Err(e) => return (Status::BadRequest, format!("Error: {e}")),
	};
	let mut file = std::fs::File::open("./access_keys").unwrap();
	let mut buffer: String = String::new();
	file.read_to_string(&mut buffer).unwrap();
	let access_keys: Vec<&str> = buffer.split("\n").collect();
	if let Some(key) = data.access_key {
		if !access_keys.contains(&key.as_str()) {
			return (Status::Forbidden, "Access key is invalid".to_string());
		}
	} else {
		unsafe {
			if ACCESS_KEY_REQUIRED {
				return (Status::Forbidden, "Access key needs to be provided".to_string());
			}
		}
	};

	let mut alias_list = read_alias();
	let mut file = std::fs::File::options().write(true).open("./alias.json").unwrap();
	let alias = match data.alias {
		None => uuid::Uuid::new_v4().to_string(),
		Some(alias) => alias,
	};
	if alias.contains("?") {
		return (Status::BadRequest, format!("Error: alias should not contain '?'"));
	}
	alias_list.push(Alias {
		url: data.url.clone(),
		alias: alias.clone(),
	});
	alias_list.dedup_by(|a, b| a.alias == b.alias);

	file.write_all(serde_json::to_string(&alias_list).unwrap().as_bytes()).unwrap();

	file.sync_all().unwrap();
	return (Status::Ok, format!("Created {} at {}", data.url, alias));
}

#[get("/404")]
fn not_found() -> Status {
	Status::NotFound
}

#[get("/<page>")]
async fn get_page(page: String) -> Redirect {
	let mut decoded_page = String::new();
	url_escape::decode_to_string(page, &mut decoded_page);
	let alias_list = read_alias();
	for i in alias_list {
		if i.alias == decoded_page {
			return Redirect::to(i.url);
		}
	}
	Redirect::to("/404")
}

#[get("/")]
async fn get_index() -> Redirect {
	Redirect::to(INDEX_REDIRECT)
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
		.mount("/", routes![not_found, create_alias, get_page, get_index])
		.launch()
		.await?;

	Ok(())
}
