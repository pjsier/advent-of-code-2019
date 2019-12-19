use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Asteroid {
    pub x: i32,
    pub y: i32,
}

impl Asteroid {
    pub fn new(x: i32, y: i32) -> Asteroid {
        Asteroid { x, y }
    }

    fn diff_range(self, start: i32, end: i32) -> RangeInclusive<i32> {
        if start < end {
            (start..=end)
        } else {
            (end..=start)
        }
    }

    pub fn in_line_of_sight(
        &self,
        asteroid: &Asteroid,
        asteroids: &HashMap<Asteroid, bool>,
    ) -> bool {
        let angle = (asteroid.x as f64 - self.x as f64).atan2(asteroid.y as f64 - self.y as f64);

        let line_of_sight = self
            .diff_range(self.x, asteroid.x)
            .map(|x| {
                self.diff_range(self.y, asteroid.y)
                    .map(|y| (x, y))
                    .collect::<Vec<(i32, i32)>>()
            })
            .flatten()
            .filter(|(x, y)| (*x as f64 - self.x as f64).atan2(*y as f64 - self.y as f64) == angle)
            .map(|(x, y)| Asteroid::new(x, y))
            .filter(|a| *a != *asteroid && *a != *self)
            .collect::<Vec<Asteroid>>();

        line_of_sight
            .iter()
            .skip_while(|a| !asteroids.contains_key(*a))
            .take(1)
            .collect::<Vec<&Asteroid>>()
            .len()
            == 0
    }

    pub fn count_in_line_of_sight(&self, asteroids: &HashMap<Asteroid, bool>) -> usize {
        asteroids
            .keys()
            .filter(|a| **a != *self && self.in_line_of_sight(*a, asteroids))
            .collect::<Vec<&Asteroid>>()
            .len()
    }
}

fn load_asteroid_map(map: String) -> HashMap<Asteroid, bool> {
    map.lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(x, c)| *c == '#')
                .map(move |(x, _c)| (Asteroid::new(x as i32, y as i32), true))
        })
        .flatten()
        .collect::<HashMap<Asteroid, bool>>()
}

fn calculate_ideal_asteroid(asteroids: &HashMap<Asteroid, bool>) -> Asteroid {
    asteroids
        .keys()
        .map(|a| (a, a.count_in_line_of_sight(asteroids)))
        .collect::<Vec<(&Asteroid, usize)>>()
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
        .clone()
}

fn vaporize_asteroids(station: Asteroid, asteroids: HashMap<Asteroid, bool>) -> Vec<Asteroid> {
    let mut asteroid_map = asteroids.clone();
    let mut vaporized: Vec<Asteroid> = vec![];

    // Remove station point
    asteroid_map.remove_entry(&station);

    while asteroid_map.len() > 0 {
        let mut in_line_of_sight = asteroid_map
            .keys()
            .map(|a| *a)
            .filter(|a| *a != station && station.in_line_of_sight(a, &asteroid_map))
            .collect::<Vec<Asteroid>>();
        in_line_of_sight.sort_by(|a, b| {
            (a.x as f64 - station.x as f64)
                .atan2(a.y as f64 - station.y as f64)
                .partial_cmp(&(b.x as f64 - station.x as f64).atan2(b.y as f64 - station.y as f64))
                .unwrap()
        });
        in_line_of_sight.reverse();
        vaporized.extend(
            in_line_of_sight
                .iter()
                .map(|a| asteroid_map.remove_entry(a).unwrap().0),
        );
    }
    vaporized
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let asteroid_map = load_asteroid_map(contents);
    let ideal = calculate_ideal_asteroid(&asteroid_map);
    println!("Part 1: {}", ideal.count_in_line_of_sight(&asteroid_map));

    let vaporized = vaporize_asteroids(ideal, asteroid_map);
    println!("Part 2: {:?}", vaporized[199].x * 100 + vaporized[199].y);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let map_str = ".#..#
.....
#####
....#
...##"
            .to_owned();
        let asteroid_map = load_asteroid_map(map_str);
        let asteroid = Asteroid::new(3, 4);
        assert_eq!(asteroid_map.len(), 10);
        assert_eq!(asteroid.count_in_line_of_sight(&asteroid_map), 8);
        assert_eq!(calculate_ideal_asteroid(&asteroid_map), asteroid);
    }

    #[test]
    fn test_sample_2() {
        let map_str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            .to_owned();
        let asteroid_map = load_asteroid_map(map_str);
        let asteroid = Asteroid::new(5, 8);
        assert_eq!(asteroid.count_in_line_of_sight(&asteroid_map), 33);
        assert_eq!(calculate_ideal_asteroid(&asteroid_map), asteroid);
    }

    #[test]
    fn test_sample_3() {
        let map_str = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
            .to_owned();
        let asteroid_map = load_asteroid_map(map_str);
        let asteroid = Asteroid::new(1, 2);
        assert_eq!(asteroid.count_in_line_of_sight(&asteroid_map), 35);
        assert_eq!(calculate_ideal_asteroid(&asteroid_map), asteroid);
    }

    #[test]
    fn test_sample_4() {
        let map_str = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
            .to_owned();
        let asteroid_map = load_asteroid_map(map_str);
        let asteroid = Asteroid::new(6, 3);
        assert_eq!(asteroid.count_in_line_of_sight(&asteroid_map), 41);
        assert_eq!(calculate_ideal_asteroid(&asteroid_map), asteroid);
    }

    #[test]
    fn test_sample_5() {
        let map_str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            .to_owned();
        let asteroid_map = load_asteroid_map(map_str);
        let asteroid = Asteroid::new(11, 13);
        assert_eq!(asteroid.count_in_line_of_sight(&asteroid_map), 210);
        assert_eq!(calculate_ideal_asteroid(&asteroid_map), asteroid);
    }

    #[test]
    fn test_sample_6() {
        let map_str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            .to_owned();
        let asteroid_map = load_asteroid_map(map_str);
        let station = Asteroid::new(11, 13);
        let vaporized = vaporize_asteroids(station, asteroid_map);
        assert_eq!(vaporized[0], Asteroid::new(11, 12));
        assert_eq!(vaporized[199], Asteroid::new(8, 2));
        assert_eq!(vaporized[298], Asteroid::new(11, 1));
    }
}
