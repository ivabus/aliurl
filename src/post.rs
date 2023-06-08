use crate::*;
use rand::{distributions::Alphanumeric, Rng};
use rocket::http::{RawStr, Status};
use rocket::response::content::RawJson;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct CreateAliasRequest {
	url: String,
	redirect_with_ad: Option<String>,
	access_key: Option<String>,
	alias: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GetAliasesRequest {
	access_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RemoveAliasRequest {
	alias: String,
	access_key: Option<String>,
}

fn check_access_key(key: Option<String>) -> Result<String, (Status, RawJson<String>)> {
	let mut file = std::fs::File::open("./access_keys").unwrap();
	let mut buffer: String = String::new();
	file.read_to_string(&mut buffer).unwrap();
	let access_keys: Vec<&str> = buffer.split("\n").collect();
	return if let Some(key) = key {
		if !access_keys.contains(&key.as_str()) {
			Err((Status::Forbidden, RawJson(json!({"Error": "Invalid access key"}).to_string())))
		} else {
			Ok(key)
		}
	} else {
		unsafe {
			if ACCESS_KEY_REQUIRED {
				Err((Status::Forbidden, RawJson(json!({"Error": "No access key"}).to_string())))
			} else {
				Ok("".to_string())
			}
		}
	};
}

#[post("/api/create_alias", data = "<data>")]
pub fn create_alias(data: &RawStr) -> (Status, RawJson<String>) {
	let data: CreateAliasRequest = match serde_json::from_str(&data.to_string()) {
		Ok(req) => req,
		Err(e) => {
			return (Status::BadRequest, RawJson(json!({"Error": e.to_string()}).to_string()))
		}
	};

	if let Err(e) = check_access_key(data.access_key) {
		return e;
	}

	let mut aliases_list = read_aliases();
	let mut file = std::fs::File::options().write(true).open("./alias.json").unwrap();
	let alias = match data.alias {
		None => {
			let mut gen: String;
			'gen: loop {
				gen = rand::thread_rng()
					.sample_iter(&Alphanumeric)
					.take(LEN_OF_GENERATIVE_ALIASES)
					.map(char::from)
					.collect();
				for i in &aliases_list {
					if i.alias == gen {
						continue 'gen;
					}
				}
				break 'gen;
			}
			gen
		}
		Some(alias) => alias,
	};
	if alias.contains("?") {
		return (
			Status::BadRequest,
			RawJson(json!({"Error": "Alias should not contain \"?\""}).to_string()),
		);
	}
	let alia = if let Some(s) = data.redirect_with_ad {
		if s.to_lowercase() == "true" {
			Alias {
				url: data.url.clone(),
				alias: alias.clone(),
				redirect_with_ad: Some(true),
			}
		} else {
			Alias {
				url: data.url.clone(),
				alias: alias.clone(),
				redirect_with_ad: None,
			}
		}
	} else {
		Alias {
			url: data.url.clone(),
			alias: alias.clone(),
			redirect_with_ad: None,
		}
	};
	aliases_list.push(alia.clone());
	aliases_list.dedup_by(|a, b| a.alias == b.alias);

	file.write_all(serde_json::to_string(&aliases_list).unwrap().as_bytes()).unwrap();

	file.sync_all().unwrap();
	return (Status::Ok, RawJson(serde_json::to_string(&alia).unwrap()));
}

#[post("/api/get_aliases", data = "<data>")]
pub fn get_aliases(data: &RawStr) -> (Status, RawJson<String>) {
	let data: GetAliasesRequest = match serde_json::from_str(&data.to_string()) {
		Ok(req) => req,
		Err(e) => {
			return (Status::BadRequest, RawJson(json!({"Error": format!("{e}")}).to_string()))
		}
	};
	if let Err(e) = check_access_key(data.access_key) {
		return e;
	}

	return (Status::Ok, RawJson(serde_json::to_string(&read_aliases()).unwrap()));
}

#[post("/api/remove_alias", data = "<data>")]
pub fn remove_alias(data: &RawStr) -> (Status, RawJson<String>) {
	let data: RemoveAliasRequest = match serde_json::from_str(&data.to_string()) {
		Ok(req) => req,
		Err(e) => {
			return (Status::BadRequest, RawJson(json!({"Error": format!("{e}")}).to_string()))
		}
	};
	if let Err(e) = check_access_key(data.access_key) {
		return e;
	}
	let mut aliases_list = read_aliases();
	let mut removed_aliases: Vec<Alias> = vec![];
	let mut file = std::fs::File::options().write(true).open("./alias.json").unwrap();

	for i in (0..aliases_list.len()).rev() {
		if aliases_list[i].alias == data.alias {
			removed_aliases.push(aliases_list.remove(i));
		}
	}
	let aliases_list = serde_json::to_string(&aliases_list).unwrap();
	file.write_all(&aliases_list.as_bytes()).unwrap();
	file.set_len(aliases_list.as_bytes().len() as u64).unwrap();
	file.sync_all().unwrap();

	return (Status::Ok, RawJson(serde_json::to_string(&removed_aliases).unwrap()));
}
