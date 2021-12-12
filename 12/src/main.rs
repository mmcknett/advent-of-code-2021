use std::collections::HashMap;
use std::io::{self, BufRead};

type AdjacencyList = HashMap<String, Vec<String>>;

fn main() {
    let mut adjacency: AdjacencyList = HashMap::new();

    for line in io::stdin().lock().lines() {
        let linestr = line.unwrap();
        let mut tofrom = linestr.split("-");
        let fr = tofrom.next().unwrap();
        let to = tofrom.next().unwrap();

        println!("from: {}, to: {}", fr, to);

        // Filter out node -> start and end -> node
        if fr != "end" && to != "start" {
            if !adjacency.contains_key(fr) {
                adjacency.insert(String::from(fr), vec![String::from(to)]);
            } else {
                adjacency.get_mut(fr).unwrap().push(String::from(to));
            }
        }

        // Filter out node -> start and end -> node
        if to != "end" && fr != "start" {
            if !adjacency.contains_key(to) {
                adjacency.insert(String::from(to), vec![String::from(fr)]);
            } else {
                adjacency.get_mut(to).unwrap().push(String::from(fr));
            }
        }
    }

    println!("Adjacency list:\n{:?}", adjacency);

    let st = String::from("start");
    let mut visited = vec![&st];
    let num_paths = count_paths("start", &adjacency, &mut visited, None);
    match num_paths {
        Some(n) => println!("There are {} paths through the caves.", n),
        None => println!("No paths through the caves.")
    }
}

fn count_paths<'a>(
    from: &str,
    adjacency: &'a AdjacencyList,
    visited: &mut Vec<&'a String>,
    doubled_str: Option<&str>
) -> Option<u32> {
    // Look at each node in the adjacency list from where we are.
    // Only count all-lowercase caves as "visited".
    return match adjacency.get(from) {
        Some(next_list) => {
            let mut count = 0;
            for next in next_list {
                if next == "end" {
                    // Enable to see all paths printed out
                    // println!("{:?}, \"end\"", visited);
                    count += 1;
                } else {
                    let next_is_small: bool = next.chars().all(|c| c.is_lowercase());
                    let next_was_visited = visited.contains(&next);
                    // let next_was_double_visited = match doubled_str {
                    //     Some(s) => s == next,
                    //     None => false
                    // };
                    if !next_was_visited || !next_is_small {
                        visited.push(&next);
                        // println!("branch 1 Visitng {}", next);
                        match count_paths(next, adjacency, visited, doubled_str) {
                            Some(next_count) => count += next_count,
                            None => ()
                        }
                        visited.pop();
                    }

                    // Part 2: if there's no doubled_str and next was visited,
                    // try using it as the doubled cave.
                    // TODO: could combine this above as an elseif?
                    if next_was_visited && next_is_small && doubled_str.is_none() {
                        visited.push(&next);
                        // println!("branch 2 Visitng {}", next);
                        match count_paths(next, adjacency, visited, Some(next)) {
                            Some(next_count) => count += next_count,
                            None => ()
                        }
                        visited.pop();
                    }
                }
            }
            Some(count)
        },
        None => None
    };
}
