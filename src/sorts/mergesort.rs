use std::thread;

/// Performs a merge sort on a list of elements.
///
/// # Params
/// - `list` - The `Vec<T>` to sort.
/// - `in_order` - The closure to use to sort the array. Determines if its `first`
/// argument comes before its `second` argument.
///
/// # Returns
/// - The sorted `Vec<T>`.
///
/// # Example
///
/// ```rust norun
///
/// // sort using a closure to sort elements in ascending order
/// let list = insertion_sort(&vec![4, 5, 2, 1, 3], &|first, second| -> bool { first <= second });
/// assert_eq!(vec![1, 2, 3, 4, 5], list);
///
/// ```
pub fn merge_sort_top_down<T, U>(list: &Vec<T>, in_order: &U) -> Vec<T>
where
    T: Clone,              // we want to be able to clone the datatype held in the vector
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    if list.len() <= 1 {
        // we have a sorted list
        return list.to_vec();
    }
    let midpoint = list.len() / 2;
    let (first, second) = list.split_at(midpoint);
    let left = merge_sort_top_down(&first.to_vec(), in_order);
    let right = merge_sort_top_down(&second.to_vec(), in_order);

    merge_vecs(&left, &right, in_order)
}

/// Merges the elements from two different vectors into one vector, in the
/// proper sorting order.
///
/// # Params
/// - `left` - The left `Vec` to merge.
/// - `right` - The right `Vec` to merge.
/// - `in_order` - The closure that determines if its `first` argument comes before its `second` argument.
///
/// # Returns
/// - The merged `Vec`.
fn merge_vecs<T, U>(left: &Vec<T>, right: &Vec<T>, in_order: &U) -> Vec<T>
where
    T: Clone,
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    let mut merged = vec![];

    // add all the elements to the resulting vec
    let mut indicies = vec![(0, left), (0, right)];
    loop {
        let element_to_add = find_next_element(&mut indicies, in_order);

        match element_to_add {
            None => {
                // break from the loop and cause the resulting merged list to be returned
                break;
            }
            Some(element) => {
                merged.push(element);
            }
        }
    }

    merged
}

/// Merges the elements from one or more different vectors into one vector, in the
/// proper sorting order.
///
/// # Params
/// - `left` - The left `Vec` to merge.
/// - `right` - The right `Vec` to merge.
/// - `in_order` - The closure that determines if its `first` argument comes before its `second` argument.
///
/// # Returns
/// - The merged `Vec`.
#[allow(dead_code)]
fn merge_multiple<T, U>(lists: &Vec<Vec<T>>, in_order: &U) -> Vec<T>
where
    T: Clone,
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    let mut merged = vec![];

    let mut indicies = vec![];
    // add all the elements to the resulting vec
    for inner_list in lists {
        indicies.push((0, inner_list));
    }

    loop {
        let element_to_add = find_next_element(&mut indicies, in_order);

        match element_to_add {
            None => {
                // break from the loop and cause the resulting merged list to be returned
                break;
            }
            Some(element) => {
                merged.push(element);
            }
        }
    }

    merged
}

/// Finds the element to add (such as a minimum or maximum) value from a `Vec` of `Vec`s.
///
/// # Params
/// - `indices` - A `Vec` of tuples, containing the (current index, list for that index). Keeps track of
/// the current index into that `Vec` and also the `Vec` itself.
/// - `in_order` - The closure that determines if its `first` argument comes before its `second` argument.
///
/// # Returns
/// - `Some(T)` when successfully finding the next element or `None` upon failure.
fn find_next_element<T, U>(indices: &mut Vec<(usize, &Vec<T>)>, in_order: &U) -> Option<T>
where
    T: Clone,
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    // add all of the elements to the merged vec
    let mut element_to_add: Option<&T> = None;
    let mut element_list_idx: Option<usize> = None;
    for i in 0..indices.len() {
        let (idx, list) = indices[i];
        if idx < list.len() {
            // if we haven't taken an element or the current element is out of the order we want
            if element_to_add.is_none()
                || !in_order(
                    element_to_add.expect("Attempted to compare None value with element"),
                    &list[idx],
                )
            {
                // this is our new element to add
                element_to_add = Some(&list[idx]);
                element_list_idx = Some(i);
            }
        }
    }

    let output = match element_to_add {
        None => None,
        Some(element) => {
            match element_list_idx {
                None => None,
                Some(old_idx) => {
                    // update the indicies vec, as we have pulled out a value
                    let (new_idx, vec) = indices[old_idx];

                    // incrementing the index
                    indices[old_idx] = (new_idx + 1, vec);
                    Some(element.clone())
                }
            }
        }
    };

    output
}

pub fn merge_sort_top_down_multithread<T, U>(
    list: Vec<T>,
    in_order: &'static U,
    num_threads: u32,
) -> Result<Vec<T>, &'static str>
where
    T: Clone + Send + Sync + 'static, // we want to be able to clone the datatype held in the vector
    U: Fn(&T, &T) -> bool + Sync + Send + Clone, // we want a closure to compare the two values and return a bool
{
    if num_threads == 0 {
        return Err("Cannot perform a merge sort with no threads");
    }

    let mut handlers = vec![];
    let work = list.len() / num_threads as usize;

    for i in 0..num_threads {
        // create a thread
        let start = i as usize * work;
        let end = if i == num_threads - 1 {
            list.len()
        } else {
            start + work
        };

        let new_list = list[start..end].to_vec();
        handlers.push(thread::spawn(move || {
            merge_sort_top_down(&new_list, in_order)
        }));
    }

    let mut merged = vec![];
    for handler in handlers {
        let result = handler.join();
        match result {
            Ok(the_vec) => merged = merge_vecs(&merged, &the_vec, in_order),
            Err(_) => {
                return Err("Error when joining thread in merge sort");
            }
        }
    }

    Ok(merged)
}
