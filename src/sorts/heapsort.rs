/// Struct to represent a Heap with generics, as it will not always be possible to
/// save the size as the first element in the vector, due to what may be stored in
/// it (could be something that isn't a number).
struct Heap<T>
where
    T: Clone,
{
    size: usize,
    list: Vec<Option<T>>,
}

/// Performs a heapsort on a list of elements.
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
/// list = heap_sort(list, &Box::new(|num1: &i32, num2: &i32| -> bool { num1 < num2 }));
/// assert_eq!(vec![1, 2, 3, 4, 5], list);
///
/// ```
pub fn heap_sort<T, U>(list: Vec<T>, in_order: &Box<U>) -> Vec<T>
where
    T: Clone,              // we want to be able to clone the datatype held in the vector
    U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
{
    let mut the_heap = Heap::new(list);
    the_heap.heapify(in_order);

    for _ in (1..=the_heap.size).rev() {
        the_heap.delete_max_or_min(in_order);
    }

    the_heap.obtain_sorted_list()
}

impl<T> Heap<T>
where
    T: Clone,
{
    /// Creates a new `Heap` from a list of `T`, inserting a `None`
    /// at the beginning to make math easier.
    ///
    /// # Params
    /// - `list` - The `Vec` of `T` to create a `Heap` from.
    pub fn new(list: Vec<T>) -> Self {
        let list_size = list.len();
        // new list starting with `None` to make math easier when performing operations on the vector
        let mut new_list = vec![None];
        for element in list {
            new_list.push(Some(element));
        }

        Self {
            size: list_size,
            list: new_list,
        }
    }

    /// Heapifies the `Heap` list, ordering the heap as a min/max heap.
    ///
    /// # Params
    /// - `in_order` - The boxed closure to sort the heap with.
    pub fn heapify<U>(&mut self, in_order: &Box<U>)
    where
        U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
    {
        let mut i = self.size / 2;
        while i > 0 {
            // percolate down from this parent
            self.percolate_down(i, in_order);
            i -= 1;
        }
    }

    /// Performs a percolate down on the heap from the parent index.
    ///
    /// # Params
    /// - `parent_idx` - The parent index to percolate down from.
    /// - `in_order` - The boxed closure to sort the heap with.
    fn percolate_down<U>(&mut self, mut parent_idx: usize, in_order: &Box<U>)
    where
        U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
    {
        while parent_idx * 2 <= self.size {
            // while the parent index has at least one child

            // find the larger of the two children (if the second child exists)
            let mut max_child_idx = parent_idx * 2;

            if max_child_idx + 1 <= self.size {
                if let (Some(first), Some(second)) =
                    (&self.list[max_child_idx], &self.list[max_child_idx + 1])
                {
                    if in_order(first, second) {
                        // if the second child exists and is larger than the first child
                        // then the second child is now the max
                        max_child_idx += 1;
                    }
                } else {
                    // NOTE: check if this code path is even possible
                    panic!("Could not get value when percolating down");
                }
            }
            if let (Some(parent), Some(child)) = (&self.list[parent_idx], &self.list[max_child_idx])
            {
                if in_order(parent, child) {
                    // swap elements
                    let temp = self.list[parent_idx].clone();
                    self.list[parent_idx] = self.list[max_child_idx].clone();
                    self.list[max_child_idx] = temp;
                }
            }

            // now set the parent_idx to be the index where we just swapped this parent to
            parent_idx = max_child_idx;
        }
    }

    /// Deletes the max/min element in the heap, adding it at the end of the heap
    /// and decreasing the size of the heap.
    ///
    /// # Params
    /// - `in_order` - The boxed closure to sort the heap with.
    fn delete_max_or_min<U>(&mut self, in_order: &Box<U>)
    where
        U: Fn(&T, &T) -> bool, // we want a closure to compare the two values and return a bool
    {
        // swap first and last element in the heap
        let temp = self.list[1].clone();
        self.list[1] = self.list[self.size].clone();
        self.list[self.size] = temp;

        self.size -= 1;

        self.percolate_down(1, in_order);
    }

    /// Obtains a sorted list from the `Heap` data, converting the
    /// `Vec<Option<T>>` back to a `Vec<T>`.
    ///
    /// # Returns
    /// - The sorted list.
    fn obtain_sorted_list(self) -> Vec<T> {
        let mut output_list = vec![];

        for element in self.list {
            match element {
                Some(value) => {
                    output_list.push(value);
                }
                None => { /* do nothing */ }
            }
        }

        output_list
    }
}
