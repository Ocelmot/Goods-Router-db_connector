use std::f64::consts::PI;
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

impl Polygon{
	pub fn from_point_radius(center: Point, radius: f64)->Polygon{
		let gon_size = 32;
		
		let radius = radius/111.32f64;
		let mut coords = vec![];
		for slice in 0..gon_size+1 {
			let angle: f64 = slice as f64 *((PI*2f64)/gon_size as f64);
			let x = center.coords.1 + angle.cos()*radius;
			let width_scale = 1f64/(x*(PI/180f64)).cos();
			let y = center.coords.0 + ((angle.sin()*radius)*width_scale);

			coords.push((y, x));
		}

		Polygon{
			coords: vec![coords]
		}
	}
}