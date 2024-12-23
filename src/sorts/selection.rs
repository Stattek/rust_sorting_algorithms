use std::mem::swap;

/// Performs a selection sort on a list of elements.
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
/// selection_sort(&mut list, |first, second| -> bool { first < second });
/// assert_eq!(vec![1, 2, 3, 4, 5], list);
///
/// ```
pub fn selection_sort<T, U>(list: &mut Vec<T>, in_order: U)
where
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    // loop through the list
    for i in 0..list.len() {
        let mut element_to_add_idx = i;
        // find the value to add to the current position
        for j in (i + 1)..list.len() {
            if !in_order(&list[element_to_add_idx], &list[j]) {
                element_to_add_idx = j;
            }
        }

        // if we want to swap with an element that isn't our current one
        if i != element_to_add_idx {
            // swap the elements
            let (left, right) = list.split_at_mut(element_to_add_idx);
            std::mem::swap(&mut left[i], &mut right[0]);
        }
    }
}
