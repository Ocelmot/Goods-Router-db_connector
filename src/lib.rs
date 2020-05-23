#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::error::Error;

use mongodb::{Client, options::ClientOptions, options::UpdateOptions};

use bson::doc;

pub use bson::oid::ObjectId;

pub mod location;
pub mod bfs;
pub mod geo_types;


pub struct GRConnection{
    db_client:  Client
}

impl GRConnection{
    pub fn new<S: Into<String>>(url: S)->Result<GRConnection, &'static str>{
        let option_result = ClientOptions::parse(&url.into());
        if let Err(_) = option_result {
            return Err("Failed to parse options");
        }
        let client_result = Client::with_options(option_result.unwrap());
        if let Err(_) = client_result {
            return Err("Failed to connect to url");
        }
        return Ok(GRConnection{db_client:client_result.unwrap()});
    }

    pub fn get_location(self, id: ObjectId) -> Result<location::Location, Box<dyn Error>>{
        let collection = self.db_client.database("goods_router").collection("locations");
        let doc = collection.find_one(doc!{"_id": id}, None)?;
        let doc = doc.ok_or("No such location")?;
        let location = bson::from_bson(bson::Bson::Document(doc))?;
        Ok(location)
    }

    pub fn save_location(&self, location: &mut location::Location) -> Result<(), Box<dyn Error>>{
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

    pub fn get_locations(&self) -> Result<mongodb::Cursor, Box<dyn Error>>{
        let collection = self.db_client.database("goods_router").collection("locations");
        let result = collection.find(None, None)?;
        Ok(result)
    }

    pub fn get_locations_within(&self, bounds: geo_types::Polygon) -> Result<mongodb::Cursor, Box<dyn Error>>{
        let bounds = bson::to_bson(&bounds)?;
        let collection = self.db_client.database("goods_router").collection("locations");
        let result = collection.find(doc!{"location": {"$geoWithin":{"$geometry": bounds}} }, None)?;
        Ok(result)
    }

    pub fn get_mutual_locations(&self, location: &location::Location) -> Result<Vec<location::Location>, Box<dyn Error>>{
        let domain = bson::to_bson(&location.domain)?;
        let location = bson::to_bson(&location.location)?;
        let collection = self.db_client.database("goods_router").collection("locations");
        let data = collection.find(doc!{
            "$or":[
                {"location": {"$geoWithin":{"$geometry": domain}}}, //thier location is in our domain
                {"domain": {"$geoIntersects": {"$geometry": location}}} // our location is in thier domain
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
}