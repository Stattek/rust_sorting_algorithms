mod sorts;

use sorts::insertion::insertion_sort;

fn main() {
    println!("Hello, world!");

    // sort an i32 vec
    let mut list = vec![
        5, 4, 2, 1, 3, 57, 65856, 345, 34534, 321321, 235457, 4, 32, 343, 54334634, 435345353,
    ];
    insertion_sort(&mut list, |num1, num2| -> bool { num1 < num2 });
    println!("{:?}", list);

    // now we can try sorting with u32!
    let mut list: Vec<u32> = vec![0, 5, 6, 32, 78, 43];
    insertion_sort(&mut list, |first, second| -> bool { first < second });
    println!("{:?}", list);
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

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

        // descending order sort
        let mut list: Vec<i32> = vec![4, 5, 2, 1, 3];
        insertion_sort(&mut list, descending_sort_closure);
        assert_eq!(vec![5, 4, 3, 2, 1], list);

        let mut list = generate_rand_vec(4000);
        insertion_sort(&mut list, ascending_sort_closure);
        is_sorted(&mut list, ascending_sort_closure);
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
}
