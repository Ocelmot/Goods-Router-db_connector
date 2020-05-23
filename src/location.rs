
use std::collections::HashMap;

use bson::doc;

use serde::{Serialize, Deserialize};

use crate::geo_types::{Point, Polygon};


fn false_val()->bool{false}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location{
	#[serde(rename="_id", skip_serializing_if="Option::is_none")]
	pub location_id: Option<bson::oid::ObjectId>,
	#[serde(skip_serializing_if="Option::is_none")]
	pub account_id: Option<bson::oid::ObjectId>,
	// pub hub_id: Option<bson::Bson::ObjectId, bson::Null>,
	#[serde(default = "false_val")]
	pub is_hub: bool,
	pub location: Point,
	pub domain: Polygon,
	pub has: HashMap<String, String>,
	pub needs: HashMap<String, String>,
}
impl Location{

	pub fn new(location: Point, domain: Polygon) -> Location{
		Location{
			location_id: None,
			account_id: None,
			// hub_id: None,
			is_hub: false,
			location: location,
			domain: domain,
			has: HashMap::new(),
			needs: HashMap::new(),
		}
	}
}
