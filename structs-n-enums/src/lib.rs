#![allow(unused)]

use std::{collections::hash_map::HashMap, io::SeekFrom};

#[derive(Debug)]
enum Temperature {
    Some(isize),
    None
}

#[derive(Debug)]
enum Population {
    Some(usize),
    None
}

#[derive(Debug)]
struct LocationInfo {
    state_name: String,
    state_code: String,
    temperature: Temperature,
    population: Population,
}

pub struct Locations {
    locations:HashMap<String, LocationInfo>
}

impl Locations {
    pub fn new() -> Locations {
        Locations{ locations:HashMap::new()}
    }

    pub fn add(&mut self, city:&str, state_name:&str, state_code:&str) {
        let mut location_info = LocationInfo {
            state_name: state_name.to_string(),
            state_code:  state_code.to_string(),
            temperature: Temperature::None,
            population: Population::None
        };
        self.locations.insert(city.to_string(), location_info);
    }

    pub fn add_all(&mut self, city:&str, state_name:&str, state_code:&str, temp:isize, popu:usize) {
   
        let mut location_info = LocationInfo {
            state_name: state_name.to_string(),
            state_code:  state_code.to_string(),
            temperature: Temperature::Some(temp),
            population: Population::Some(popu)
        };
        self.locations.insert(city.to_string(), location_info);
    }

    pub fn get_city_info(&self, city:&str) -> Option<(String, String, Option<isize>, Option<usize>)> {
        let mut name = "".to_string();
        let mut code = "".to_string();
        let mut temp = Option::None;
        let mut popu = Option::None;

        match self.locations.get(&city.to_string()) {

            Option::Some(loc_info) => {
                name = loc_info.state_name.clone();
                code = loc_info.state_code.clone();
                temp = match loc_info.temperature {
                            Temperature::Some(t) => { Option::Some(t) }
                            _ => { Option::None}
                };
                popu = match loc_info.population {
                            Population::Some(p) => { Option::Some(p) }
                            _ => { Option::None}
                };

                Option::Some((name, code, temp, popu))
        }

            _ => { Option::None }

        }
    }
}