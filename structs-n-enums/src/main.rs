#![allow(unused)]

use locations::Locations;

fn main() {


    let mut my_locations = Locations::new();

    my_locations.add_all("Miami", "Florida", "FL", 89, 8_234_343);
    my_locations.add_all("New York City", "New York", "NY", 75, 28_234_111);
    my_locations.add_all("Los Angeles", "California", "CA", 70, 40_234_111);

    my_locations.add("Birmingham", "Alabama", "AL");


    print_location_info("Atlanta", &my_locations);
    print_location_info( "Miami", &my_locations);
    print_location_info("Birmingham", &my_locations);


}

fn print_location_info(city:&str, my_locations:&Locations) {
    let city_info = my_locations.get_city_info(city);
    match city_info {
        Option::Some((name, code, temp, popu)) => {
                        println!("\n\nCity:{}\nstate_name:{}\nstate_code:{}\ntemp:{:?}\npopu:{:?}",
                    city, name, code, temp, popu);
        }
        _ => { println!("\n\nCity {} not found.\n\n", city) }
    }
}
