
use std::collections::HashMap;

use bson::{Bson, doc, bson};

use serde::{Serialize, Deserialize};

use crate::geo_types::{Point, Polygon};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location{
	#[serde(rename="_id", skip_serializing_if="Option::is_none")]
	pub location_id: Option<bson::oid::ObjectId>,
	#[serde(skip_serializing_if="Option::is_none")]
	pub account_id: Option<bson::oid::ObjectId>,
	// pub hub_id: Option<bson::Bson::ObjectId, bson::Null>,
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
			location: location,
			domain: domain,
			has: HashMap::new(),
			needs: HashMap::new(),
		}
	}
}

// impl From<Location> for bson::Document {
// 	fn from(location: Location) -> Self {
// 		let mut d = doc!{
// 			"location": {"type": "Point", "coordinates": [location.location.0, location.location.1]}
// 		};

// 		match location.domain{
// 			Domain::Radius(r) => {
// 				d.insert_bson(String::from("domain"), bson!({"type": "Radius", "radius": r}));
// 			},
// 			Domain::Polygon(c) => {
// 				let mut coords: Vec<Bson> = vec![];
// 				for (lon, lat) in c {
// 					let pair: Vec<Bson> = vec![Bson::FloatingPoint(lon), Bson::FloatingPoint(lat)];
// 					coords.push(Bson::Array(pair));
// 				}
// 				d.insert_bson(String::from("domain"), bson!({"type": "Polygon", "coordinates": Bson::Array(coords)}));
// 			},
// 		}
// 		d
// 	}
// }



