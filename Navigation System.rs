use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    id: String,
    name: String,
    coordinates: (f64, f64),
}

#[derive(Debug)]
struct Connection {
    from: String,
    to: String,
    distance: f64,
}

#[derive(Debug)]
struct NavigationGraph {
    locations: HashMap<String, Location>,
    connections: HashMap<String, Vec<Connection>>,
}

impl NavigationGraph {
    fn new() -> Self {
        NavigationGraph {
            locations: HashMap::new(),
            connections: HashMap::new(),
        }
    }

    fn add_location(&mut self, location: Location) {
        self.locations.insert(location.id.clone(), location);
    }

    fn add_connection(&mut self, from: String, to: String, distance: f64) {
        let connection = Connection {
            from: from.clone(),
            to: to.clone(),
            distance,
        };
        self.connections
            .entry(from)
            .or_insert_with(Vec::new)
            .push(connection);
    }

    fn get_location(&self, id: &str) -> Option<&Location> {
        self.locations.get(id)
    }

    fn shortest_path(&self, start: &str, end: &str) -> Option<(Vec<String>, f64)> {
        let mut distances = HashMap::new();
        let mut previous = HashMap::new();
        let mut to_visit = VecDeque::new();
        let mut visited = Vec::new();

        distances.insert(start.to_string(), 0.0);
        to_visit.push_back(start.to_string());

        while let Some(current) = to_visit.pop_front() {
            if current == end {
                let mut path = Vec::new();
                let mut current_node = end.to_string();
                path.push(current_node.clone());

                while let Some(prev) = previous.get(&current_node) {
                    path.push(prev.clone());
                    current_node = prev.clone();
                }

                path.reverse();
                return Some((path, *distances.get(end).unwrap()));
            }

            if let Some(connections) = self.connections.get(&current) {
                for connection in connections {
                    let new_distance = distances.get(&current).unwrap() + connection.distance;
                    let is_shorter = distances
                        .get(&connection.to)
                        .map_or(true, |&d| new_distance < d);

                    if is_shorter {
                        distances.insert(connection.to.clone(), new_distance);
                        previous.insert(connection.to.clone(), current.clone());
                        to_visit.push_back(connection.to.clone());
                    }
                }
            }

            visited.push(current);
        }

        None
    }

    fn nearby_locations(&self, center: &str, radius: f64) -> Vec<&Location> {
        let mut result = Vec::new();
        if let Some(center_loc) = self.locations.get(center) {
            for (_, loc) in &self.locations {
                if loc.id != center.id {
                    let dx = loc.coordinates.0 - center_loc.coordinates.0;
                    let dy = loc.coordinates.1 - center_loc.coordinates.1;
                    let distance = (dx * dx + dy * dy).sqrt();
                    if distance <= radius {
                        result.push(loc);
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut graph = NavigationGraph::new();

    let locations = vec![
        Location {
            id: "A".to_string(),
            name: "Central Park".to_string(),
            coordinates: (40.7829, -73.9654),
        },
        Location {
            id: "B".to_string(),
            name: "Times Square".to_string(),
            coordinates: (40.7580, -73.9855),
        },
        Location {
            id: "C".to_string(),
            name: "Empire State".to_string(),
            coordinates: (40.7484, -73.9857),
        },
        Location {
            id: "D".to_string(),
            name: "Statue of Liberty".to_string(),
            coordinates: (40.6892, -74.0445),
        },
    ];

    for loc in locations {
        graph.add_location(loc);
    }

    graph.add_connection("A".to_string(), "B".to_string(), 1.5);
    graph.add_connection("B".to_string(), "A".to_string(), 1.5);
    graph.add_connection("B".to_string(), "C".to_string(), 0.8);
    graph.add_connection("C".to_string(), "B".to_string(), 0.8);
    graph.add_connection("C".to_string(), "D".to_string(), 3.2);
    graph.add_connection("D".to_string(), "C".to_string(), 3.2);

    if let Some((path, distance)) = graph.shortest_path("A", "D") {
        println!("Path from A to D:");
        for node in path {
            if let Some(loc) = graph.get_location(&node) {
                println!("- {} ({})", loc.name, loc.id);
            }
        }
        println!("Total distance: {} units", distance);
    }

    let nearby = graph.nearby_locations("B", 2.0);
    println!("Locations near B:");
    for loc in nearby {
        println!("- {} ({})", loc.name, loc.id);
    }
}
