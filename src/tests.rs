// ignoring unused import and dead code warnings for tests
#![allow(unused_imports)]
#![allow(dead_code)]
use std::fmt::Debug;

use crate::{
    bubble_sort, insertion_sort, merge_sort_top_down, selection_sort,
    sorts::{
        heapsort::heap_sort, mergesort::merge_sort_top_down_multithread, quicksort::quick_sort,
    },
};
use rand::Rng;

static BASIC_SORT_RAND_VEC_LEN: u32 = 4000;
static ADVANCED_SORT_RAND_VEC_LEN: u32 = 40000;

#[test]
fn insertion_sort_test() {
    let ascending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 <= num2 };
    let descending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 >= num2 };
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

    let mut list = generate_rand_vec(BASIC_SORT_RAND_VEC_LEN);
    insertion_sort(&mut list, ascending_sort_closure);
    assert_eq!(true, is_sorted(&list, ascending_sort_closure));
}

#[test]
fn bubble_sort_test() {
    let ascending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 <= num2 };
    let descending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 >= num2 };
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

    let mut list = generate_rand_vec(BASIC_SORT_RAND_VEC_LEN);
    bubble_sort(&mut list, ascending_sort_closure);
    assert_eq!(true, is_sorted(&list, ascending_sort_closure));
}

#[test]
fn selection_sort_test() {
    let ascending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 <= num2 };
    let descending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 >= num2 };
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

    let mut list = generate_rand_vec(BASIC_SORT_RAND_VEC_LEN);
    selection_sort(&mut list, ascending_sort_closure);
    assert_eq!(true, is_sorted(&list, ascending_sort_closure));
}

#[test]
fn merge_sort_test() {
    // NOTE: to prevent the checks from failing upon some tests, doing `<=` or `>=` should be used
    // in cases where there are more than one of the same number
    let ascending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 <= num2 };
    let descending_sort_closure = |num1: &i32, num2: &i32| -> bool { num1 >= num2 };

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

    // since this algorithm is faster than the others, let's push it a little
    let mut list = generate_rand_vec(ADVANCED_SORT_RAND_VEC_LEN);
    list = merge_sort_top_down(&list, &ascending_sort_closure);
    assert_eq!(true, is_sorted(&list, ascending_sort_closure));
}

#[test]
fn merge_sort_multithreaded_test() {
    let num_threads = 8; // This is the sweet spot on my machine that typically gets the best results
    let list = generate_rand_vec(400000);
    let closure = |num1: &i32, num2: &i32| -> bool { num1 <= num2 };

    let merge_sort_result = merge_sort_top_down_multithread(
        list,
        // evil static closure
        &|num1: &i32, num2: &i32| -> bool { num1 <= num2 },
        num_threads,
    );

    match merge_sort_result {
        Ok(merged_list) => {
            assert_eq!(true, is_sorted(&merged_list, closure))
        }
        Err(err_val) => {
            panic!("Error when doing multithreaded merge sort ({})", err_val);
        }
    }
}

/// Generates a random vector of `i32`s.
pub fn generate_rand_vec(num_elements: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut output = vec![];
    for _ in 0..num_elements {
        output.push(rng.gen_range(i32::MIN..=i32::MAX));
    }

    output
}

#[test]
fn quick_sort_test() {
    let ascending_sort_closure = Box::new(|num1: &i32, num2: &i32| -> bool { num1 < num2 });
    let ascending_is_eq_closure = Box::new(|num1: &i32, num2: &i32| -> bool { num1 <= num2 });
    let descending_sort_closure = Box::new(|num1: &i32, num2: &i32| -> bool { num1 > num2 });
    let descending_is_eq_closure = Box::new(|num1: &i32, num2: &i32| -> bool { num1 >= num2 });

    // create a list of elements
    let mut list = vec![4, 5, 2, 1, 3];
    // sort using a closure to sort elements in ascending order
    list = quick_sort(list, &ascending_sort_closure);
    assert_eq!(vec![1, 2, 3, 4, 5], list);

    let mut list: Vec<i32> = vec![];
    list = quick_sort(list, &ascending_sort_closure);
    let test: Vec<i32> = vec![];
    assert_eq!(test, list);

    let mut list: Vec<i32> = vec![1];
    list = quick_sort(list, &ascending_sort_closure);
    assert_eq!(vec![1], list);

    // descending order sort
    let mut list: Vec<i32> = vec![4, 5, 2, 1, 3];
    list = quick_sort(list, &descending_sort_closure);
    assert_eq!(vec![5, 4, 3, 2, 1], list);

    let mut list = generate_rand_vec(ADVANCED_SORT_RAND_VEC_LEN);
    list = quick_sort(list, &ascending_sort_closure);
    assert_eq!(true, is_sorted(&list, &ascending_is_eq_closure));

    let mut list = generate_rand_vec(ADVANCED_SORT_RAND_VEC_LEN);
    list = quick_sort(list, &descending_sort_closure);
    assert_eq!(true, is_sorted(&list, &descending_is_eq_closure));
}

#[test]
fn heap_sort_test() {
    let ascending_sort_closure = Box::new(|num1: &i32, num2: &i32| -> bool { num1 < num2 });
    let ascending_is_eq_closure = Box::new(|num1: &i32, num2: &i32| -> bool { num1 <= num2 });
    let descending_sort_closure = Box::new(|num1: &i32, num2: &i32| -> bool { num1 > num2 });
    let descending_is_eq_closure = Box::new(|num1: &i32, num2: &i32| -> bool { num1 >= num2 });

    // create a list of elements
    let mut list = vec![4, 5, 2, 1, 3];
    // sort using a closure to sort elements in ascending order
    list = heap_sort(list, &ascending_sort_closure);
    assert_eq!(vec![1, 2, 3, 4, 5], list);

    let mut list: Vec<i32> = vec![];
    list = heap_sort(list, &ascending_sort_closure);
    let test: Vec<i32> = vec![];
    assert_eq!(test, list);

    let mut list: Vec<i32> = vec![1];
    list = heap_sort(list, &ascending_sort_closure);
    assert_eq!(vec![1], list);

    // descending order sort
    let mut list: Vec<i32> = vec![4, 5, 2, 1, 3];
    list = heap_sort(list, &descending_sort_closure);
    assert_eq!(vec![5, 4, 3, 2, 1], list);

    let mut list = generate_rand_vec(ADVANCED_SORT_RAND_VEC_LEN);
    list = heap_sort(list, &ascending_sort_closure);
    assert_eq!(true, is_sorted(&list, &ascending_is_eq_closure));

    let mut list = generate_rand_vec(ADVANCED_SORT_RAND_VEC_LEN);
    list = heap_sort(list, &descending_sort_closure);
    assert_eq!(true, is_sorted(&list, &descending_is_eq_closure));
}

/// Checks to see if a vector is sorted.
pub fn is_sorted<U, T>(list: &Vec<T>, in_order: U) -> bool
where
    T: Debug,
    U: Fn(&T, &T) -> bool,
{
    let mut output = true;
    for i in 1..list.len() {
        if !in_order(&list[i - 1], &list[i]) {
            output = false;
            println!(
                "ERROR: Element {:?} should not come before element {:?}",
                list[i - 1],
                list[i]
            );
            break;
        }
    }

    output
}
