use std::collections::HashMap;

#[derive(Debug)]
struct Node(u64, u64);

impl Node {
    fn new(destination: u64, range: u64) -> Self {
        Node(destination, range)
    }
}

pub fn part_one(seeds: Vec<u64>, content: &Vec<String>) -> u64 {
    let mut almanac: HashMap<String, HashMap<u64, Node>> = HashMap::from([
        (String::from("seed-to-soil"), HashMap::new()),
        (String::from("soil-to-fertilizer"), HashMap::new()),
        (String::from("fertilizer-to-water"), HashMap::new()),
        (String::from("water-to-light"), HashMap::new()),
        (String::from("light-to-temperature"), HashMap::new()),
        (String::from("temperature-to-humidity"), HashMap::new()),
        (String::from("humidity-to-location"), HashMap::new()),
    ]);

    let mut key = String::from("seed-to-soil");

    for line in content {
        if almanac.keys().collect::<Vec<&String>>().contains(&&line) {
            key = line.to_string();
        } else {
            let values: Vec<u64> = line
                .split(' ')
                .map(|value| match value.trim().parse() {
                    Ok(number) => number,
                    Err(_) => panic!("Value should be a number"),
                })
                .collect();
            let (destination, source, range) = (values[0], values[1], values[2]);

            almanac.entry(key.to_string()).and_modify(|map| {
                map.entry(source).or_insert(Node::new(destination, range));
            });
        }
    }

    seeds
        .iter()
        .map(|seed| get_value(*seed, almanac.get("seed-to-soil").unwrap()))
        .map(|soil| get_value(soil, almanac.get("soil-to-fertilizer").unwrap()))
        .map(|fertilizer| get_value(fertilizer, almanac.get("fertilizer-to-water").unwrap()))
        .map(|water| get_value(water, almanac.get("water-to-light").unwrap()))
        .map(|light| get_value(light, almanac.get("light-to-temperature").unwrap()))
        .map(|temperature| get_value(temperature, almanac.get("temperature-to-humidity").unwrap()))
        .map(|humidity| get_value(humidity, almanac.get("humidity-to-location").unwrap()))
        .min()
        .unwrap()
}

fn get_value(source: u64, map: &HashMap<u64, Node>) -> u64 {
    let mapped_source = map
        .iter()
        .filter(|(map_source, node)| {
            source >= **map_source && source < map_source.saturating_add(node.1)
        })
        .next();

    match mapped_source {
        Some((map_source, node)) => (node.0..=node.0.saturating_add(map_source.abs_diff(source)))
            .last()
            .unwrap(),
        None => source,
    }
}
