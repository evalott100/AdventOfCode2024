use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

fn load_input(path: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let file_string = read_to_string(path).expect("Failed to read file");

    let mut number_rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    let (ordering_rules_raw, update_raw) = file_string.split_once("\n\n").expect("Bad file!");
    for line in ordering_rules_raw.lines() {
        let (page_before_raw, page_after_raw) = line.split_once("|").expect("Bad line!");
        let page_before: i32 = page_before_raw.parse().unwrap();
        let page_after: i32 = page_after_raw.parse().unwrap();

        number_rules
            .entry(page_before)
            .or_insert(HashSet::new())
            .insert(page_after);
    }

    // I investigated using a Vec<HashSet<usize, i32>> for this but it wasn't any faster,
    // I think becaues the limitations to getting by index is offset by the better iteration
    // performance.
    let mut update_orders: Vec<Vec<i32>> = Vec::new();

    for update_order_raw in update_raw.split('\n').filter(|x| !x.is_empty()) {
        let update_order: Vec<i32> = update_order_raw
            .split(",")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        update_orders.push(update_order);
    }

    (number_rules, update_orders)
}

fn check_correctly_ordered(
    number_rules: &HashMap<i32, HashSet<i32>>,
    update_order: &[i32],
) -> bool {
    for (order_index, page_number) in update_order.iter().enumerate() {
        if !(number_rules.contains_key(page_number)) {
            continue;
        }

        let after = number_rules.get(page_number).unwrap();

        if update_order[..order_index]
            .iter()
            .any(|x| after.contains(x))
        {
            return false;
        }
    }
    true
}

fn update_reordered(
    number_rules: &HashMap<i32, HashSet<i32>>,
    update_order: &Vec<i32>,
) -> Option<Vec<i32>> {
    // Very sloppy while loop here
    // ... But apparently it's faster than getting all the indices which need to go after and then
    // swapping, despite this taking more iterations on average... :-/
    // The below implementation takes more time:
    /*

                 let pages_that_need_to_go_after = number_rules.get(&reordered_update[order_index]);

                 if pages_that_need_to_go_after.is_none() {
                     continue;
                 }

                 let problem_indices: Vec<usize> = reordered_update[..order_index]
                     .iter()
                     .enumerate()
                     .filter_map(|(index, x)| {
                         if pages_that_need_to_go_after.unwrap().contains(x) {
                             Some(index)
                         } else {
                             None
                         }
                     })
                     .collect();

                 if problem_indices.is_empty() {
                     continue;
                 }
                 reordered_update.swap(order_index, problem_indices[0]);
    * */

    let mut loop_ran_once = false;

    let mut reordered_update: Vec<i32> = update_order.clone();

    while !check_correctly_ordered(number_rules, &reordered_update) {
        loop_ran_once = true;
        for order_index in 0..reordered_update.len() {
            let pages_that_need_to_go_after = match number_rules.get(&reordered_update[order_index])
            {
                Some(to_go_after) => to_go_after,
                None => {
                    continue;
                }
            };

            for page_to_swap_after in pages_that_need_to_go_after.iter() {
                if reordered_update[..order_index].contains(page_to_swap_after) {
                    let page_after_index = reordered_update
                        .iter()
                        .position(|x| x == page_to_swap_after)
                        .unwrap();
                    reordered_update.swap(order_index, page_after_index);
                }
            }
        }
    }

    if loop_ran_once {
        Some(reordered_update)
    } else {
        None
    }
}

fn solution_1(number_rules: &HashMap<i32, HashSet<i32>>, update_orders: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for update_order in update_orders.iter() {
        if !check_correctly_ordered(number_rules, update_order) {
            continue;
        }
        sum += update_order[update_order.len() / 2];
    }
    sum
}

fn solution_2(number_rules: &HashMap<i32, HashSet<i32>>, update_orders: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for update_order in update_orders.iter() {
        if let Some(reordered_update) = update_reordered(number_rules, update_order) {
            sum += reordered_update[reordered_update.len() / 2];
        }
    }
    sum
}

fn main() {
    let input_start = Instant::now();
    let (number_rules, update_order) = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(&number_rules, &update_order);
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    let output_2 = solution_2(&number_rules, &update_order);
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
