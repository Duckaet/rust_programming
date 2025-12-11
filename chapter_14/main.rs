struct City {
    name: String,
    population: i64,
    country: String,
    stats: Stastics

    impl get_statistics(stats: Stastics){
        rand(1*city.population)
    }
}


fn sort_cities(cities: &mut Vec<City>){
    cities.sort_by_key(|city| -city.population);
}

fn sort_cities_with_stats(cities: &mut Vec<City>, stats: Stastics){
    cities.sort_by_key(|city| -city.get_statistics(stats))
}

// fn startSortingAlgorithm(cities,stat){
//     fn keyfn(city){
//         city.get_statistics(stat)
//     }

//     if pendingSort {

//     }
// }

use std::thread;


fn start_sorting(cities: &mut Vec<City>, stats: Stastics)-> thread::JoinHandle<Vec<City>>{
    fn key_fn = |city: &mut City| -> i64 {
        -city.get_statistics(stats)
    };

    thread::spawn(move||{
        cities.sort_by_key(key_fn);
        cities
    })
}

fn count_selected_cities(cities: &Vec<City>,
test_fn: fn(&City) -> bool) -> usize
{let mut count = 0;
for city in cities {
if test_fn(city) {
count += 1;
}
}
count
}
/// An example of a test function. Note that the type of
/// this function is `fn(&City) -> bool`, the same as
/// the `test_fn` argument to `count_selected_cities`.
fn has_monster_attacks(city: &City) -> bool {
city.monster_attack_risk > 0.0
}
// How many cities are at risk for monster attack?
let n = count_selected_cities(&my_cities, has_monster_attacks);


