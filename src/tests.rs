// ignoring unused import and dead code warnings for tests
#![allow(unused_imports)]
#![allow(dead_code)]
use crate::{bubble_sort, insertion_sort, merge_sort_top_down, selection_sort};
use rand::Rng;

#[test]
fn insertion_sort_test() {
    let ascending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 < num2 };
    let descending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 > num2 };
    // create a list of elements
    let mut list = vec![4, 5, 2, 1, 3];
    // sort using a closure to sort elements in ascending order
    insertion_sort(&mut list, ascending_sort_closure);
    assert_eq!(vec![1, 2, 3, 4, 5], list);

    let mut list: Vec<i32> = vec![];
    insertion_sort(&mut list, ascending_sort_closure);
    let test: Vec<i32> = vec![];
    assert_eq!(test, list);

    let mut list: Vec<i32> = vec![1];
    insertion_sort(&mut list, ascending_sort_closure);
    assert_eq!(vec![1], list);

    // descending order sort
    let mut list: Vec<i32> = vec![4, 5, 2, 1, 3];
    insertion_sort(&mut list, descending_sort_closure);
    assert_eq!(vec![5, 4, 3, 2, 1], list);

    let mut list = generate_rand_vec(4000);
    insertion_sort(&mut list, ascending_sort_closure);
    is_sorted(&list, ascending_sort_closure);
}

#[test]
fn bubble_sort_test() {
    let ascending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 < num2 };
    let descending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 > num2 };
    // create a list of elements
    let mut list = vec![4, 5, 2, 1, 3];
    // sort using a closure to sort elements in ascending order
    bubble_sort(&mut list, ascending_sort_closure);
    assert_eq!(vec![1, 2, 3, 4, 5], list);

    let mut list: Vec<i32> = vec![];
    bubble_sort(&mut list, ascending_sort_closure);
    let test: Vec<i32> = vec![];
    assert_eq!(test, list);

    let mut list: Vec<i32> = vec![1];
    bubble_sort(&mut list, ascending_sort_closure);
    assert_eq!(vec![1], list);

    // descending order sort
    let mut list: Vec<i32> = vec![4, 5, 2, 1, 3];
    bubble_sort(&mut list, descending_sort_closure);
    assert_eq!(vec![5, 4, 3, 2, 1], list);

    let mut list = generate_rand_vec(4000);
    bubble_sort(&mut list, ascending_sort_closure);
    is_sorted(&list, ascending_sort_closure);
}

#[test]
fn selection_sort_test() {
    let ascending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 < num2 };
    let descending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 > num2 };
    // create a list of elements
    let mut list = vec![4, 5, 2, 1, 3];
    // sort using a closure to sort elements in ascending order
    selection_sort(&mut list, ascending_sort_closure);
    assert_eq!(vec![1, 2, 3, 4, 5], list);

    let mut list: Vec<i32> = vec![];
    selection_sort(&mut list, ascending_sort_closure);
    let test: Vec<i32> = vec![];
    assert_eq!(test, list);

    let mut list: Vec<i32> = vec![1];
    selection_sort(&mut list, ascending_sort_closure);
    assert_eq!(vec![1], list);

    // descending order sort
    let mut list: Vec<i32> = vec![4, 5, 2, 1, 3];
    selection_sort(&mut list, descending_sort_closure);
    assert_eq!(vec![5, 4, 3, 2, 1], list);

    let mut list = generate_rand_vec(4000);
    selection_sort(&mut list, ascending_sort_closure);
    is_sorted(&list, ascending_sort_closure);
}

#[test]
fn merge_sort_test() {
    let ascending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 < num2 };
    let descending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 > num2 };

    // create a list of elements
    let mut list = vec![4, 5, 2, 1, 3];
    // sort using a closure to sort elements in ascending order
    list = merge_sort_top_down(&list, &ascending_sort_closure);
    assert_eq!(vec![1, 2, 3, 4, 5], list);

    let mut list: Vec<i32> = vec![];
    list = merge_sort_top_down(&list, &ascending_sort_closure);
    let test: Vec<i32> = vec![];
    assert_eq!(test, list);

    let mut list: Vec<i32> = vec![1];
    list = merge_sort_top_down(&list, &ascending_sort_closure);
    assert_eq!(vec![1], list);

    // descending order sort
    let mut list: Vec<i32> = vec![4, 5, 2, 1, 3];
    list = merge_sort_top_down(&list, &descending_sort_closure);
    assert_eq!(vec![5, 4, 3, 2, 1], list);

    let mut list = generate_rand_vec(400000);
    list = merge_sort_top_down(&list, &ascending_sort_closure);
    is_sorted(&list, ascending_sort_closure);
}

/// Generates a random vector of `i32`s.
fn generate_rand_vec(num_elements: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut output = vec![];
    for _ in 0..num_elements {
        output.push(rng.gen_range(i32::MIN..=i32::MAX));
    }

    output
}

/// Checks to see if a vector is sorted.
fn is_sorted<U, T>(list: &Vec<T>, in_order: U) -> bool
where
    U: Fn(&T, &T) -> bool,
{
    let mut output = true;
    for i in 1..list.len() {
        if !in_order(&list[i - 1], &list[i]) {
            output = false;
            break;
        }
    }

    output
}
