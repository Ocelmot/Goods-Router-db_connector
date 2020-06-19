
use crate::GRConnection;
use crate::location::Location;

use queues::*;
use std::error::Error;
use std::rc::Rc;
use std::collections::BTreeMap;

pub struct BFS{
	connector: GRConnection
}


fn print_locations(locations: &Vec<Location>){
	for location in locations{
		println!("Location {:?}", location);
	}
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
		let mut parents: BTreeMap<bson::oid::ObjectId, Rc<Location>> = BTreeMap::new();
		let start_rc = Rc::new(start.clone());

		//get set of next hops (locations within start location's domain + locations where the start location is in its domain)
		let start_locations = self.connector.get_mutual_locations(&start)?;
		println!("Start locations:");
		print_locations(&start_locations);
		for l in start_locations{
			let id = l.location_id.clone().ok_or("Location has no id")?;
			parents.insert(id, start_rc.clone());
			queue.add(l)?;
		}
		
		
		//loop through locations
		while queue.size() != 0{
			let current_location = queue.remove()?;
			println!("=====Current Location {:?}", current_location);
			//If a location needs what we have (or vice versa if we are consuming) this is the end of the path. (Add to the array and finish)
			if current_location.needs.contains_key(&resource){
				println!("===============Backtracing==============");
				let mut path: Vec<Location> = vec![];
				path.push(current_location.clone());
				println!("Parents tree {:?}", parents);
				// trace back through parents to build path!
				let mut key = current_location.location_id.ok_or("Location has no id!")?;
				loop{
					println!("backlocation id {:?}", key);
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
				path.reverse();
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
}
