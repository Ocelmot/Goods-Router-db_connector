


use serde::{Serialize,  Deserialize};



type Coordinate = (f64, f64);

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Point{
	#[serde(rename = "coordinates")]
	pub coords: Coordinate
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Polygon{
	#[serde(rename = "coordinates")]
	pub coords: Vec<Vec<Coordinate>>
}





// trait GeoData{
// 	// fn from(name: &str, coordinates: Vec<(f64, f64)>) -> Self;
// 	fn get_type<'a>(self) -> &'a str;
// 	fn get_coordinates<'a>(self) -> &'a Vec<(f64, f64)>;
// }


// impl Serialize for dyn GeoData {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S:Serializer,{
// 		let mut m = serializer.serialize_map(Some(2))?;
// 		m.serialize_entry("type", self.get_type());
// 		m.serialize_entry("coordinates", self.get_coordinates());
// 		m.end()
//     }
// }

// struct GeoDataVisitor;
// impl<'de> Visitor<'de> for GeoDataVisitor{
//     type Value = GeoDataVisitor;
//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result{
//         formatter.write_str("A testEnum to be turned into a boolean")
//     }

//     fn visit_map<E>(self, value: DeserializeMap) -> Result<Self::Value, E> where E: de::Error,{
// 		let t = value.get("type")?;
// 		let c = value.get("coordinates")?;
// 		// GeoData::
//     }
// }


// impl<'de> Deserialize<'de> for dyn GeoData{
// 	fn deserialize<D>(deserializer: D) -> Result<dyn GeoData, D::Error> where D:Deserializer<'de>,{
// 		deserializer.deserialize_bool(GeoVisitor)
// 	}
// }



















pub enum GeoArea{
	Radius(f64),
	Polygon(Vec<(f64, f64)>),
}



