use std::collections::HashMap;

use bson::doc;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User{
	#[serde(rename="_id", skip_serializing_if="Option::is_none")]
	pub user_id: Option<bson::oid::ObjectId>,
	#[serde(rename="_id", skip_serializing_if="Option::is_none")]
	pub location_id: Option<bson::oid::ObjectId>,
	pub username: String,
	pub password: String,
}
impl User{
	pub fn new(username: String, password: String) -> User{
		User{
			user_id: None,
			location_id: None,
			username: username,
			password: password
		}
	}
}
