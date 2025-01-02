use std::mem::swap;

use crate::sorts::insertionsort::insertion_sort;

/// Performs a quicksort on a list of elements.
///
/// # Params
/// - `list` - The `Vec<T>` to sort.
/// - `in_order` - The closure to use to sort the array. Determines if its `first`
/// argument comes before its `second` argument.
///
/// # Returns
/// - The sorted `Vec<T>`.
///
/// # Notes
/// - Avoid using `<=` or `>=`, as this sort doesn't work properly with those comparisons.
/// - Stick to using `<` and `>` for comparing elements.
///
/// # Example
///
/// ```rust norun
///
/// // sort using a closure to sort elements in ascending order
/// // NOTE: avoid using <= or >=, as this sort doesn't work properly with those comparisons.
/// let mut list = vec![4, 5, 2, 1, 3];
/// list = quick_sort(list, &Box::new(|num1: &i32, num2: &i32| -> bool { num1 < num2 }));
/// assert_eq!(vec![1, 2, 3, 4, 5], list);
///
/// ```
pub fn quick_sort<T, U>(list: Vec<T>, in_order: &Box<U>) -> Vec<T>
where
    T: Clone,              // we want to be able to clone the datatype held in the vector
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    quick_sort_recursive(list, in_order)
}

/// Performs a recursive quicksort. Helper function for quick_sort.
///
/// # Params
/// - `list` - The `Vec<T>` to sort.
/// - `in_order` - The closure to use to sort the array. Determines if its `first`
/// argument comes before its `second` argument.
///
/// # Returns
/// - The sorted `Vec<T>`.
///
/// # Notes
/// - Avoid using `<=` or `>=`, as this sort doesn't work properly with those comparisons.
/// - Stick to using `<` and `>` for comparing elements.
/// - This implementation could be made better, if I knew how to share multiple mutable references
/// to modify the list rather than copying parts of it to subsequent recursive calls (Is this possible in unsafe Rust?).
/// This means that this algorithm could likely be made better in terms of memory management. It's still fast, though.
fn quick_sort_recursive<T, U>(mut list: Vec<T>, in_order: &Box<U>) -> Vec<T>
where
    T: Clone,              // we want to be able to clone the datatype held in the vector
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    static CUTOFF: usize = 100;

    // return the list if it is empty
    if list.is_empty() {
        return list;
    }

    let (left, right) = (0, list.len() - 1);

    if right - left + 1 >= CUTOFF {
        // if we aren't at or below the cutoff

        if right - left > 0 {
            // we have at least two items in this partition

            // get the pivot from a median of three
            median_of_three(&mut list, &in_order, left, right);

            let pivot_idx = right - 1;
            let pivot = list[pivot_idx].clone();
            // we don't want to compare list[j] with the pivot itself
            let (mut i, mut j) = (left, pivot_idx - 1);

            loop {
                // increment i (we are looking for an element larger than the pivot if sorting in ascending order)
                while in_order(&list[i], &pivot) {
                    i += 1;
                }

                // decrement j (we are looking for an element that is smaller than the pivot if we are sorting in ascending order)
                while j > 0 && !in_order(&list[j], &pivot) {
                    j -= 1;
                }

                // if i and j have crossed
                if i >= j {
                    break;
                }

                // swap list[i] and list[j] otherwise
                let (left_list, right_list) = list.split_at_mut(j);
                swap(&mut left_list[i], &mut right_list[0]);
            }

            // since i and j have crossed, we swap list[i] and the pivot
            list[right - 1] = list[i].clone();
            list[i] = pivot.clone();

            // the pivot is now in place at i
            // we can now recursively call quicksort on the two partitions
            let (left_list, right_list) = (list[left..i].to_vec(), list[i + 1..=right].to_vec());
            // modifying the list to make the borrow checker happy
            list = quick_sort_recursive(left_list, in_order);
            list.append(&mut vec![pivot]);
            list.append(&mut quick_sort_recursive(right_list, in_order));
        }
    } else {
        insertion_sort(&mut list, in_order);
    }

    // return modified list
    list
}

/// Performs a median-of-three ordering of elements in the `Vec` for sorting with
/// quicksort.
///
/// Sorts the leftmost, center, and rightmost elements, then swaps the center
/// element with the second-to-last element in the `Vec`.
///
/// # Params
/// - `list` - The list to perform the ordering on.
/// - `in_order` - The boxed closure to do the ordering based upon.
/// - `left` - The first (inclusive) index to do the ordering upon.
/// - `right` - The last (inclusive) index to do the ordering upon.
fn median_of_three<T, U>(list: &mut Vec<T>, in_order: &Box<U>, left: usize, right: usize)
where
    T: Clone,              // we want to be able to clone the datatype held in the vector
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    let center = ((right - left) / 2) + left;

    // compare the first and center elements
    if !in_order(&list[left], &list[center]) {
        let (first, second) = list.split_at_mut(center);
        swap(&mut first[left], &mut second[0]);
    }

    // compare the first and last elements
    if !in_order(&list[left], &list[right]) {
        let (first, second) = list.split_at_mut(right);
        swap(&mut first[left], &mut second[0]);
    }

    // compare the middle and last elements
    if !in_order(&list[center], &list[right]) {
        let (first, second) = list.split_at_mut(right);
        swap(&mut first[center], &mut second[0]);
    }

    // now swap the middle element with the second to last element
    let (first, second) = list.split_at_mut(right - 1);
    swap(&mut first[center], &mut second[0]);
}
