mod point;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs;

use point::Point;

fn from_str(map: String) -> Vec<Point> {
    return map
        .lines()
        .enumerate()
        .map(|(y, r)| {
            r.chars().enumerate().filter_map(move |(x, r1)| {
                return if r1 == '#' {
                    Some(Point {
                        x: x as i16,
                        y: y as i16,
                    })
                } else {
                    None
                };
            })
        })
        .flatten()
        .collect();
}

fn find_optimal_station(asteroids: Vec<Point>) -> Option<(Point, usize)> {
    asteroids
        .iter()
        .map(|&a| {
            let visible: HashSet<Point> = asteroids
                .iter()
                .filter_map(move |&b| if a == b { None } else { Some(a - b) })
                .map(|v| v.normalize())
                .collect();

            (a, visible.len())
        })
        .max_by_key(|(_, len)| *len)
}

fn get_nth_vaporized_asteroid(station: Point, asteroids: Vec<Point>, n: usize) -> Option<Point> {
    // Normalize and store in hashmap
    let mut vaporized = BTreeMap::new();

    for asteroid in asteroids {
        if asteroid != station {
            vaporized
                .entry((asteroid - station).normalize())
                .or_insert(Vec::new())
                .push(asteroid);
        }
    }

    // Sort asteroids by length from the station
    vaporized.iter_mut().for_each(|(_, value)| {
        value.sort_by(|a, b| {
            (*b - station)
                .mag()
                .partial_cmp(&(*a - station).mag())
                .unwrap()
        });
    });

    return match vaporized.iter_mut().enumerate().find(|(i, _)| *i == n) {
        Some((_, (_, val))) => val.pop(),
        None => None,
    };
}

fn main() {
    let input = fs::read_to_string("data/10.dat").expect("Something went wrong reading the file");

    let asteroids = from_str(input);
    let part_one = find_optimal_station(asteroids.clone()).unwrap();
    println!("Part one pos: {:?}, answer: {}", part_one.0, part_one.1);
    let part_two = get_nth_vaporized_asteroid(part_one.0, asteroids, 199).unwrap();
    println!(
        "Part one pos: {:?}, answer: {}",
        part_two,
        part_two.x * 100 + part_two.y
    );
}

#[test]
fn test_part_two() {
    let asteroids = from_str(
        ".#..##.###...#######
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
            .to_string(),
    );
    let station = find_optimal_station(asteroids.clone()).unwrap().0;
    assert!(station == Point { x: 11, y: 13 });
    let res = get_nth_vaporized_asteroid(station, asteroids.clone(), 0).unwrap();
    assert!(res == Point { x: 11, y: 12 });
    let res = get_nth_vaporized_asteroid(station, asteroids.clone(), 2).unwrap();
    assert!(res == Point { x: 12, y: 2 });
    let res = get_nth_vaporized_asteroid(station, asteroids.clone(), 9).unwrap();
    assert!(res == Point { x: 12, y: 8 });
    let res = get_nth_vaporized_asteroid(station, asteroids.clone(), 19).unwrap();
    assert!(res == Point { x: 16, y: 0 });
    let res = get_nth_vaporized_asteroid(station, asteroids.clone(), 49).unwrap();
    assert!(res == Point { x: 16, y: 9 });
    let res = get_nth_vaporized_asteroid(station, asteroids.clone(), 99).unwrap();
    assert!(res == Point { x: 10, y: 16 });
    let res = get_nth_vaporized_asteroid(station, asteroids.clone(), 199).unwrap();
    assert!(res == Point { x: 8, y: 2 });
    let res = get_nth_vaporized_asteroid(station, asteroids.clone(), 200).unwrap();
    assert!(res == Point { x: 10, y: 9 });
}
