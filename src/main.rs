use std::{collections::HashMap, fs, ops::Sub, time::Instant};

/*
    Day 5 - Part Two: One of the most optimal solution for this problem
    using a little bit of Math for our input value, we can determine the following:

    f(seed) = soil
        -> f(soil) = fertilizer
            -> f(fertilizer) = water
                -> f(water) = light
                    -> f(light) = temperature
                        -> f(temperature) = humidity
                            -> f(humidity) = location
    
    From this we can build a system to calculate directly the location based on a seed. By using
    function composition we can create such system.
*/

/*
    A node represents the following:
    x + shift     lower_bound <= x <= upper_bound
    Where x is the input seed
 */
#[derive(Debug, Clone)]
struct Node {
    lower_bound: i64,
    upper_bound: i64,
    shift: i64,
}

impl Node {
    fn new(lower_bound: Option<i64>, upper_bound: Option<i64>, shift: Option<i64>) -> Self {
        Node {
            lower_bound: match lower_bound {
                Some(lower) => lower,
                None => i64::MIN,
            },
            upper_bound: match upper_bound {
                Some(upper) => upper,
                None => i64::MAX,
            },
            shift: match shift {
                Some(shift) => shift,
                None => 0,
            },
        }
    }

    fn merge(node_a: &Self, node_b: &Self) -> Option<Self> {
        let node_b_lower = node_b.lower_bound.saturating_sub(node_a.shift);
        let node_b_upper = node_b.upper_bound.saturating_sub(node_a.shift);

        if node_a.upper_bound < node_b_lower || node_b_upper < node_a.lower_bound {
            return None;
        }

        let &lower = [node_a.lower_bound, node_b_lower].iter().max().unwrap();
        let &upper = [node_a.upper_bound, node_b_upper].iter().min().unwrap();
        let shift = node_a.shift.saturating_add(node_b.shift);

        Some(Node {
            lower_bound: lower,
            upper_bound: upper,
            shift,
        })
    }
}

/*
    A node wise represents the system that will hold all the nodes
    for a given range:
    
    From: Seed
    To: Soil
    Nodes = [
        x + shift     lower_bound <= x <= upper_bound
        ...
        ...
        ...
    ]
    Where x is the input seed
 */
#[derive(Debug, Clone)]
struct NodeWise {
    from: String,
    to: String,
    nodes: Vec<Node>,
}

impl NodeWise {
    fn new(from: String, to: String, nodes: Vec<Node>) -> Self {
        NodeWise {
            from,
            to,
            nodes: NodeWise::fill_gaps(nodes),
        }
    }

    fn fill_gaps(nodes: Vec<Node>) -> Vec<Node> {
        let mut result = Vec::new();
        let mut cloned_nodes = nodes.clone();

        cloned_nodes.sort_by(|node_a, node_b| node_a.lower_bound.cmp(&node_b.lower_bound));

        if cloned_nodes[0].lower_bound != i64::MIN {
            result.push(Node::new(
                None,
                Some(cloned_nodes[0].lower_bound.sub(1)),
                None,
            ));
        } 

        result.push(cloned_nodes[0].clone());

        for index in 1..cloned_nodes.len() {
            let previous_node = cloned_nodes[index - 1].clone();
            let current_node = cloned_nodes[index].clone();

            if (current_node
                .lower_bound
                .saturating_sub(previous_node.upper_bound))
                > 1
            {
                result.push(Node::new(
                    Some(previous_node.upper_bound.saturating_add(1)),
                    Some(current_node.lower_bound.saturating_sub(1)),
                    None,
                ));
            }

            result.push(current_node);
        }

        let last = cloned_nodes.last().unwrap();

        if last.upper_bound != i64::MAX {
            result.push(Node::new(
                Some(result.last().unwrap().upper_bound.saturating_add(1)),
                None,
                None,
            ))
        }

        result
    }

    fn merge(self, node_b: &NodeWise) -> Self {
        let mut nodes = Vec::new();

        self.nodes.iter().for_each(|a| {
            node_b.nodes.iter().for_each(|b| match Node::merge(a, b) {
                Some(merged_node) => nodes.push(merged_node),
                None => (),
            })
        });

        NodeWise::new(self.from.to_string(), node_b.to.to_string(), nodes.clone())
    }
}

const KEYS: [&'static str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location"
];

fn main() {
    let content = fs::read_to_string("./input.txt").expect("File should exist.");
    let mut lines = content.lines();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|seed_id| match seed_id.trim().parse() {
            Ok(id) => id,
            Err(_) => panic!("Seed should be a number"),
        })
        .collect();
    let seed_ranges = seeds.chunks(2);
    let content: Vec<String> = lines
        .filter(|x| !x.is_empty())
        .map(|line| line.trim().replace(" map:", ""))
        .collect();
    let mut almanac: HashMap<String, Vec<Node>> = HashMap::from([
        (String::from("seed-to-soil"), Vec::new()),
        (String::from("soil-to-fertilizer"), Vec::new()),
        (String::from("fertilizer-to-water"), Vec::new()),
        (String::from("water-to-light"), Vec::new()),
        (String::from("light-to-temperature"), Vec::new()),
        (String::from("temperature-to-humidity"), Vec::new()),
        (String::from("humidity-to-location"), Vec::new()),
    ]);

    let mut key = String::from("seed-to-soil");

    for line in content {
        if almanac.keys().collect::<Vec<&String>>().contains(&&line) {
            key = line.to_string();
        } else {
            let values: Vec<i64> = line
                .split(' ')
                .map(|value| match value.trim().parse() {
                    Ok(number) => number,
                    Err(_) => panic!("Value should be a number"),
                })
                .collect();
            let (destination, source, range) = (values[0], values[1], values[2]);

            almanac.entry(key.to_string()).and_modify(|nodes| {
                nodes.push(Node::new(
                    Some(source),
                    Some(source + range),
                    Some(destination.saturating_sub(source)),
                ));
            });
        }
    }

    let systems: [NodeWise; 7] = KEYS.map(|key| {
        let nodes = almanac.get(key).unwrap();
        let split_key: Vec<&str> = key.split("-to-").collect();
        
        NodeWise::new(
            split_key[0].to_string(),
            split_key[1].to_string(),
            nodes.clone(),
        )
    });

    let mut system = systems.first().unwrap().clone();

    for current_system in systems.iter().skip(1) {
        system = system.merge(current_system);
    }

    let min_location = seed_ranges
        .map(|seed| {
            let lower = seed[0] as i64;
            let upper = seed.iter().sum::<u64>().saturating_sub(1) as i64;
            let mut location: Vec<i64> = Vec::new();

            system.nodes.iter().for_each(|node| {
                if node.upper_bound < lower || upper < node.lower_bound {
                    return;
                }

                let node_lower = [node.lower_bound, lower].into_iter().max().unwrap();
                location.push(node_lower + node.shift);
            });
            *location.iter().min().unwrap()
        })
        .min()
        .unwrap();

    let start_time = Instant::now();

    println!("Time to get soil: {:?}", start_time.elapsed());
    println!("Location: {min_location}");
}
