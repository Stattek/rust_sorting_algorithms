mod sorts;
mod tests;

use sorts::{bubble::bubble_sort, insertion::insertion_sort, merge::merge_sort_top_down, selection::selection_sort};

fn main() {
    println!("Hello, world!");

    // sort an i32 vec
    let mut list = vec![
        5, 4, 2, 1, 3, 57, 65856, 345, 34534, 321321, 235457, 4, 32, 343, 54334634, 435345353,
    ];
    insertion_sort(&mut list, |first, second| -> bool { first < second });
    println!("Insertion sort: {:?}", list);

    // now we can try sorting with u32!
    let mut list: Vec<u32> = vec![0, 5, 6, 32, 78, 43];
    insertion_sort(&mut list, |first, second| -> bool { first < second });
    println!("Insertion sort: {:?}", list);

    // Bubble sort
    let mut list: Vec<u32> = vec![0, 5, 6, 32, 78, 43];
    bubble_sort(&mut list, |first, second| -> bool { first < second });
    println!("Bubble sort: {:?}", list);

    // Selection sort
    let mut list: Vec<u32> = vec![0, 5, 6, 32, 78, 43];
    selection_sort(&mut list, |first, second| -> bool { first < second });
    println!("Selection sort: {:?}", list);

    // Merge sort
    let mut list: Vec<u32> = vec![0, 5, 6, 32, 78, 43];
    list = merge_sort_top_down(&list, &|first, second| -> bool { first < second });
    println!("Merge sort: {:?}", list);
}
