use std::mem::swap;

/// Performs a bubble sort on a list of elements.
/// Implemented using the non-naive approach (checks if the `Vec` is already sorted
/// and breaks early if so).
///
/// # Params
/// - `list` - The `Vec` to sort.
/// - `comparison_closure` - The closure to use to sort the array.
///
/// # Example
///
/// ```rust norun
///
/// // create a list of elements
/// let mut list = vec![4,5,2,1,3];
/// // sort using a closure to sort elements in ascending order
/// bubble_sort(&mut list, |first, second| -> bool { first <= second });
/// assert_eq!(vec![1, 2, 3, 4, 5], list);
///
/// ```
pub fn bubble_sort<T, U>(list: &mut Vec<T>, in_order: U)
where
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    for i in (1..list.len()).rev() {
        // assume it is sorted until we find something that is not in order
        let mut is_sorted = true;
        for j in 1..=i {
            if !in_order(&list[j - 1], &list[j]) {
                // we found something out of order
                is_sorted = false;

                // split the vector to be able to swap behind a mutable reference
                let (first, second) = list.split_at_mut(j);

                // swap positions j-1 and j
                swap(&mut first[j - 1], &mut second[0]);
            }
        }

        // break if we find out that the vector is already sorted
        if is_sorted {
            break;
        }
    }
}
