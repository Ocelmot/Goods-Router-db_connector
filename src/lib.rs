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



    pub fn get_locations(&self) -> Result<mongodb::Cursor, &'static str>{
        let collection = self.db_client.database("goods_router").collection("locations");
        // let err_result = collection.find_one(doc!{"_id": id}, None);
        let err_result = collection.find(None, None);
        match err_result{
            Err(_) => {return Err("Could not fetch locations")},
            Ok(result) =>{Ok(result)}
        }
    }



    // pub fn get_location(self, id: ObjectId) -> Result<location::Location, &'static str>{
    //     let collection = self.db_client.database("goods_router").collection("locations");
    //     let err_result = collection.find_one(doc!{"_id": id}, None);
    //     match err_result{
    //         Err(_) => {return Err("Could not get location with that id")},
    //         Ok(result) => {
    //             if let Some(result) = result {
    //                 let loc_result = bson::from_bson(bson::Bson::Document(result));
                    
    //                 match loc_result{
    //                     Ok(x) => {
    //                         return Ok(x);
    //                     },
    //                     Err(x) => {
    //                         print!("{}", x);
    //                         return Err("Could not deserialize document");
    //                     }
    //                 }
    //             }else{
    //                 return Err("No such document");
    //             }
    //         }
    //     }
    // }






    pub fn save_location(&self, location: &location::Location) -> Result<(), &'static str>{
        let collection = self.db_client.database("goods_router").collection("locations");
        let ser = bson::to_bson(&location);


        match ser {
            Ok(x) => {
                let doc = x.as_document();
                match doc {
                    Some(doc_some) => {
                        let err_result = match location.location_id{
                            Some(id) => {
                                let options = UpdateOptions::builder();
                                options.upsert(true);
                                collection.update_one(doc!{"_id": bson::Bson::ObjectId(id)}, doc_some.to_owned(), options.build())
                            },
                            None =>{
                                collection.insert_one(doc_some.to_owned(), None)
                            }
                        };
                        match err_result{
                            Err(_) => {return Err("Could not insert test")},
                            Ok(_) => {return Ok(());}
                        }
                    },
                    None => {
                        return Err("Could not convert bson to document")
                    }
                }
            },
            Err(x) => {
                return Err("Could not serialize location");
            }
        }
    }
}