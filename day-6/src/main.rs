use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

fn check_orbit_key<'a>(orbit_pair: Vec<&'a str>, orbit_map: &mut HashMap<&'a str, Vec<&'a str>>) {
    match orbit_map.entry(orbit_pair[1]) {
        Entry::Vacant(e) => {
            e.insert(vec![orbit_pair[0]]);
        }
        Entry::Occupied(mut e) => {
            e.get_mut().push(orbit_pair[0]);
        }
    }
}

fn count_suborbits<'a>(orbit: &'a str, orbit_map: &HashMap<&'a str, Vec<&'a str>>) -> isize {
    match orbit_map.get(orbit) {
        Some(value) => {
            (value.len() as isize)
                + value
                    .iter()
                    .map(|o| count_suborbits(o, &orbit_map))
                    .sum::<isize>()
        }
        None => 0,
    }
}

fn get_parents<'a>(orbit: &'a str, orbit_map: &HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    match orbit_map.get(orbit) {
        Some(orbits) => {
            let orbits_vec = orbits.clone();
            orbits_vec
                .into_iter()
                .chain(
                    orbits
                        .clone()
                        .into_iter()
                        .flat_map(|o| get_parents(o, orbit_map))
                        .collect::<Vec<&str>>(),
                )
                .collect::<Vec<&str>>()
        }
        None => vec![],
    }
}

fn parent_distance<'a>(
    start_orbit: &'a str,
    end_orbit: &'a str,
    orbit_map: &HashMap<&'a str, Vec<&'a str>>,
) -> Option<isize> {
    match orbit_map.get(start_orbit) {
        Some(orbits) => {
            if orbits.contains(&end_orbit) {
                return Some(1);
            } else {
                let orbit_val = orbits
                    .into_iter()
                    .filter_map(|o| parent_distance(o, end_orbit, orbit_map))
                    .min();
                if orbit_val.is_some() {
                    return Some(orbit_val.unwrap() + 1);
                } else {
                    return None;
                }
            }
        }
        None => None,
    }
}

fn main() {
    let mut orbit_map = HashMap::new();
    let contents = fs::read_to_string("input.txt").unwrap();
    let orbit_splits: Vec<Vec<&str>> = contents
        .lines()
        .map(|l| l.split(')').collect::<Vec<&str>>())
        .collect();

    orbit_splits
        .into_iter()
        .map(|item| check_orbit_key(item, &mut orbit_map))
        .collect::<Vec<_>>();

    let orbit_count = &orbit_map
        .keys()
        .map(|o| count_suborbits(o, &orbit_map))
        .sum::<isize>();

    println!("Part 1 answer is: {}", orbit_count);

    let you_parents = get_parents("YOU", &orbit_map);
    let san_parents = get_parents("SAN", &orbit_map);

    let orbit_overlap: Vec<&str> = san_parents
        .into_iter()
        .filter(|o| you_parents.contains(o))
        .collect();

    let min_distance = orbit_overlap
        .into_iter()
        .map(|o| {
            parent_distance("YOU", o, &orbit_map).unwrap()
                + parent_distance("SAN", o, &orbit_map).unwrap()
                - 2
        })
        .min();
    println!("Part 2 answer is: {}", min_distance.unwrap());
}
