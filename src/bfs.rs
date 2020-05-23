
use crate::GRConnection;
use crate::location::Location;

use queues::*;
use std::error::Error;
use std::rc::Rc;
use std::collections::BTreeMap;

pub struct BFS{
	connector: GRConnection
}

impl BFS{

	pub fn new(connector: GRConnection) -> BFS{
		BFS{
			connector
		}
	}

	// origin, resource -> array of locations that is a path
	pub fn bfs(&self, start: Location, resource: String) -> Result<Vec<Location>, Box<dyn Error>>{
		let mut queue: Queue<Location> = Queue::new();
		// queue.
		
		//get set of next hops (locations within start location's domain + locations where the start location is in its domain)
		let start_locations = self.connector.get_mutual_locations(&start)?;
		for l in start_locations{
			queue.add(l)?;
		}
		let mut parents: BTreeMap<bson::oid::ObjectId, Rc<Location>> = BTreeMap::new();
		//loop through locations
		while queue.size() != 0{
			let current_location = queue.remove()?;
			//If a location needs what we have (or vice versa if we are consuming) this is the end of the path. (Add to the array and finish)
			if current_location.needs.contains_key(&resource){
				let mut path: Vec<Location> = vec![];
				// trace back through parents to build path!
				let mut key = current_location.location_id.ok_or("Location has no id!")?;
				loop{
					let parent = parents.remove(&key);
					match parent{
						Some(l) =>{ // add location to path
							let l = &*l;
							let l = l.clone();
							key = l.location_id.clone().ok_or("Location has no id!")?;
							path.push(l);
						},
						None => { // no parent, must be start element, we are done
							break;
						}
					}
				}
				//maybe reverse path before returning?
				return Ok(path);
			}

			//If location is a hub, get (locations within start location's domain + locations where the start location is in its domain), add those locations to the end of the locations array
			if current_location.is_hub{
				let new_locations = self.connector.get_mutual_locations(&current_location)?;
				let current_location_rc = Rc::new(current_location);
				for location in new_locations{
					let key = location.location_id.clone().ok_or("Location has no id!")?;
					
					if parents.contains_key(&key){ // if location has a parent, it has been already been discovered
						continue;
					}
					//Otherwise, set parent to the hub that introduced it
					parents.insert(key, current_location_rc.clone());
					//and enqueue
					queue.add(location)?;
				}
			}
		}
		
		//If we exhaust the array, there is no path, return None 
		return Err("No path exists".into());
	}

	//Vec<T> //type (vector of some type)
	//vec![]; //macro shorthand

	//(locations within start location's domain + locations where the start location is in its domain)
    fn get_mutuals(self, location: Location) -> Vec<Location>{ //return array of locations
        
        
		let domains = vec![];
		// location.domain;

		domains
	}
}

