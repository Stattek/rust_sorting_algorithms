mod sorts;
mod tests;

use std::time::Instant;

use sorts::{
    bubble::bubble_sort,
    insertion::insertion_sort,
    merge::{merge_sort_top_down, merge_sort_top_down_multithread},
    selection::selection_sort,
};
use tests::{generate_rand_vec, is_sorted};

fn main() {
    println!("Hello, world!");

    // sort an i32 vec
    let mut list = vec![
        5, 4, 2, 1, 3, 57, 65856, 345, 34534, 321321, 235457, 4, 32, 343, 54334634, 435345353,
    ];
    insertion_sort(&mut list, |first, second| -> bool { first <= second });
    println!("Insertion sort: {:?}", list);

    // now we can try sorting with u32!
    let mut list: Vec<u32> = vec![0, 5, 6, 32, 78, 43];
    insertion_sort(&mut list, |first, second| -> bool { first <= second });
    println!("Insertion sort: {:?}", list);

    // Bubble sort
    let mut list: Vec<u32> = vec![0, 5, 6, 32, 78, 43];
    bubble_sort(&mut list, |first, second| -> bool { first <= second });
    println!("Bubble sort: {:?}", list);

    // Selection sort
    let mut list: Vec<u32> = vec![0, 5, 6, 32, 78, 43];
    selection_sort(&mut list, |first, second| -> bool { first <= second });
    println!("Selection sort: {:?}", list);

    // Merge sort
    let mut list: Vec<u32> = vec![0, 5, 6, 32, 78, 43];
    list = merge_sort_top_down(&list, &|first, second| -> bool { first <= second });
    println!("Merge sort: {:?}", list);

    let num_threads = 8; // This is the sweet spot on my machine that typically gets the best results
    let list = generate_rand_vec(400000);
    let closure = |num1: &i32, num2: &i32| -> bool { num1 <= num2 };

    let starting_time = Instant::now();
    let merge_sort_result = merge_sort_top_down_multithread(
        list,
        // evil static closure
        &|num1: &i32, num2: &i32| -> bool { num1 <= num2 },
        num_threads,
    );

    match merge_sort_result {
        Ok(merged_list) => {
            let elapsed_time = starting_time.elapsed();
            assert_eq!(true, is_sorted(&merged_list, closure));
            println!(
                "Merge sort multithreaded took {} ms",
                elapsed_time.as_millis()
            );
        }
        Err(err_val) => {
            panic!("Error when doing multithreaded merge sort ({})", err_val);
        }
    }
}
