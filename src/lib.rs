#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

use mongodb::{Client, options::ClientOptions, options::UpdateOptions};

use bson::doc;

pub use bson::oid::ObjectId;

pub mod location;
pub mod user;
pub mod bfs;
pub mod geo_types;
pub mod pool;


#[derive(Debug)]
pub struct Error {msg:String, source: Option<Box<dyn std::error::Error>>}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match &self.source{
			Some(err) => {
				write!(f, "Error {}\nFrom {}", self.msg, &*err)
			},
			None => {
				write!(f, "Error {}", self.msg)
			}
		}
    }
}

impl std::error::Error for Error{
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
    }
}

impl From<String> for Error{
	fn from(error: String) -> Self{
		Error{msg: error.into(), source: None}
	}
}

impl From<&str> for Error{
	fn from(error: &str) -> Self{
		Error{msg: error.into(), source: None}
	}
}

impl From<mongodb::error::Error> for Error{
	fn from(error: mongodb::error::Error) -> Self{
		Error{msg: "Mongodb Error".into(), source: Some(Box::new(error))}
	}
}

impl From<bson::DecoderError> for Error{
	fn from(error: bson::DecoderError) -> Self{
		Error{msg: "Bson Error".into(), source: Some(Box::new(error))}
	}
}

impl From<bson::EncoderError> for Error{
	fn from(error: bson::EncoderError) -> Self{
		Error{msg: "Bson Error".into(), source: Some(Box::new(error))}
	}
}







pub struct GRConnection{
	db_client:  Client,
	errored: bool
}

impl GRConnection{
	pub fn new<S: Into<String>>(url: S)->Result<GRConnection, Error>{
		let option_result = ClientOptions::parse(&url.into());
		if let Err(_) = option_result {
			return Err("Failed to parse options".into());
		}
		let client_result = Client::with_options(option_result.unwrap());
		if let Err(_) = client_result {
			return Err("Failed to connect to url".into());
		}
		return Ok(GRConnection{db_client:client_result.unwrap(), errored:false});
	}

	pub fn err(&self)->bool{
		self.errored
	}
	pub fn is_connected(&self)->bool{
		let collection = self.db_client.database("goods_router").collection("locations");
		let doc = collection.find_one(doc!{}, None);
		true
	}

	pub fn get_location(&self, id: ObjectId) -> Result<location::Location, Error>{
		let collection = self.db_client.database("goods_router").collection("locations");
		let doc = collection.find_one(doc!{"_id": id}, None)?;
		let doc = doc.ok_or("No such location")?;
		let location = bson::from_bson(bson::Bson::Document(doc))?;
		Ok(location)
	}

	pub fn save_location(&self, location: &mut location::Location) -> Result<(), Error>{
		let collection = self.db_client.database("goods_router").collection("locations");
		let ser = bson::to_bson(&location)?;

		let doc = ser.as_document().ok_or("Could not convert bson to document")?;
 
		match &location.location_id{
			Some(id) => {
				let options = UpdateOptions::builder().upsert(true).build();
				collection.update_one(doc!{"_id": id}, doc.to_owned(), options)?;
				return Ok(());
			},
			None =>{
				let res = collection.insert_one(doc.to_owned(), None)?;
				let id = res.inserted_id.as_object_id().ok_or("")?.to_owned();
				location.location_id = Some(id);
				return Ok(());
			}
		}
	}

	pub fn get_locations(&self) -> Result<mongodb::Cursor, Error>{
		let collection = self.db_client.database("goods_router").collection("locations");
		let result = collection.find(None, None)?;
		Ok(result)
	}

	pub fn get_locations_within(&self, bounds: geo_types::Polygon) -> Result<mongodb::Cursor, Error>{
		let bounds = bson::to_bson(&bounds)?;
		let collection = self.db_client.database("goods_router").collection("locations");
		let result = collection.find(doc!{"location": {"$geoWithin":{"$geometry": bounds}} }, None)?;
		Ok(result)
	}

	pub fn get_mutual_locations(&self, location: &location::Location) -> Result<Vec<location::Location>, Error>{
		let domain = bson::to_bson(&location.domain)?;
		let location = bson::to_bson(&location.location)?;
		let collection = self.db_client.database("goods_router").collection("locations");
		let data = collection.find(doc!{
			"$or":[
				{"location": {"$geoWithin":{"$geometry": domain}}}, //their location is in our domain
				{"domain": {"$geoIntersects": {"$geometry": location}}} // our location is in their domain
			]
		}, None)?;
		let mut result = vec!{};
		for l in data{
			let l = l.unwrap();
			let l: location::Location = bson::from_bson(bson::Bson::Document(l)).unwrap();
			result.push(l);
		}

		Ok(result)
	}

	pub fn save_user(&self, user: &mut user::User) -> Result<(), Error>{
		let collection = self.db_client.database("goods_router").collection("users");
		let ser = bson::to_bson(&user)?;
		let doc = ser.as_document().ok_or("Could not convert bson to document")?;
 
		match &user.user_id{
			Some(id) => {
				let options = UpdateOptions::builder().upsert(true).build();
				collection.update_one(doc!{"_id": id}, doc.to_owned(), options)?;
				return Ok(());
			},
			None =>{
				let res = collection.insert_one(doc.to_owned(), None)?;
				let id = res.inserted_id.as_object_id().ok_or("")?.to_owned();
				user.user_id = Some(id);
				return Ok(());
			}
		}
	}

	pub fn get_user(&self, id: ObjectId) -> Result<user::User, Error>{
		let collection = self.db_client.database("goods_router").collection("users");
		let doc = collection.find_one(doc!{"_id": id}, None)?;
		let doc = doc.ok_or("No such User")?;
		let user = bson::from_bson(bson::Bson::Document(doc))?;
		Ok(user)
	}

	pub fn get_user_by_name(&self, username: String) -> Result<user::User, Error>{
		let collection = self.db_client.database("goods_router").collection("users");
		let doc = collection.find_one(doc!{"username": username }, None)?;
		let doc = doc.ok_or("No such User")?;
		let user = bson::from_bson(bson::Bson::Document(doc))?;
		Ok(user)
	}
}