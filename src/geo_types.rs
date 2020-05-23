use serde::{Serialize,  Deserialize};

type Coordinate = (f64, f64);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct Point{
	#[serde(rename = "coordinates")]
	pub coords: Coordinate
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct Polygon{
	#[serde(rename = "coordinates")]
	pub coords: Vec<Vec<Coordinate>>
}