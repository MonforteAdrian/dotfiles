use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let lines = std::fs::read_to_string("data/day5.txt").unwrap();
    let res1 = part1(&lines);
    let res2 = part2(&lines);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn part1(s: &str) -> i32 {
    let mut res = 0;
    let mut sections: Vec<&str> = s.split("\n\n").collect();
    let updates = sections.pop().expect("No updates section");
    let rules = sections.pop().expect("No rules section");

    let graph = parse_rules(rules);

    for update in updates.lines() {
        let pages: Vec<i32> = update
            .split(',')
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if is_valid_order(&graph, &pages) {
            res += pages[pages.len() / 2];
        }
    }
    res
}

fn part2(s: &str) -> i32 {
    let mut sections: Vec<&str> = s.split("\n\n").collect();
    let updates = sections.pop().expect("No updates section");
    let rules = sections.pop().expect("No rules section");

    let graph = parse_rules(rules);
    let mut sum = 0;

    for update in updates.lines() {
        let pages: Vec<i32> = update
            .split(',')
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if !is_valid_order(&graph, &pages) {
            let reordered = reorder_pages(&graph, &pages);
            sum += reordered[reordered.len() / 2];
        }
    }

    sum
}

fn parse_rules(rules: &str) -> HashMap<i32, HashSet<i32>> {
    let mut graph = HashMap::new();

    for line in rules.lines() {
        if let Some((left, right)) = line.split_once('|') {
            let left: i32 = left.parse().unwrap();
            let right: i32 = right.parse().unwrap();

            graph.entry(left).or_insert_with(HashSet::new).insert(right);
        }
    }

    graph
}

fn is_valid_order(graph: &HashMap<i32, HashSet<i32>>, pages: &[i32]) -> bool {
    let mut position = HashMap::new();
    for (index, &page) in pages.iter().enumerate() {
        position.insert(page, index);
    }

    for (&page, dependents) in graph {
        if let Some(&page_pos) = position.get(&page) {
            for &dependent in dependents {
                if let Some(&dep_pos) = position.get(&dependent) {
                    if page_pos >= dep_pos {
                        return false; // Rule violated
                    }
                }
            }
        }
    }

    true
}

fn reorder_pages(graph: &HashMap<i32, HashSet<i32>>, pages: &[i32]) -> Vec<i32> {
    let mut indegree = HashMap::new();
    let mut adj_list = HashMap::new();

    // Build adjacency list and indegree map
    for &page in pages {
        indegree.entry(page).or_insert(0);
        adj_list.entry(page).or_insert_with(HashSet::new);
    }

    for (&page, dependents) in graph {
        if pages.contains(&page) {
            for &dependent in dependents {
                if pages.contains(&dependent) {
                    adj_list.get_mut(&page).unwrap().insert(dependent);
                    *indegree.entry(dependent).or_insert(0) += 1;
                }
            }
        }
    }

    // Perform topological sort
    let mut sorted = Vec::new();
    let mut queue = VecDeque::new();

    for (&page, &deg) in &indegree {
        if deg == 0 {
            queue.push_back(page);
        }
    }

    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        if let Some(dependents) = adj_list.get(&page) {
            for &dependent in dependents {
                if let Some(deg) = indegree.get_mut(&dependent) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(dependent);
                    }
                }
            }
        }
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!(part1(report), 143);
    }
}
