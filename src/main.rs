mod sorts;

use sorts::insertion::insertion_sort;

fn main() {
    println!("Hello, world!");

    let mut list = vec![
        5, 4, 2, 1, 3, 57, 65856, 345, 34534, 321321, 235457, 4, 32, 343, 54334634, 435345353,
    ];
    insertion_sort(&mut list, |num1, num2| -> bool { num1 < num2 });
    println!("{:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_test() {
        // create a list of elements
        let mut list = vec![4, 5, 2, 1, 3];
        // sort using a closure to sort elements in ascending order
        insertion_sort(&mut list, |num1, num2| -> bool { num1 < num2 });
        assert_eq!(vec![1, 2, 3, 4, 5], list);

        let mut list: Vec<i32> = vec![];
        insertion_sort(&mut list, |num1, num2| -> bool { num1 < num2 });
        let test: Vec<i32> = vec![];
        assert_eq!(test, list);

        // descending order sort
        let mut list: Vec<i32> = vec![4, 5, 2, 1, 3];
        insertion_sort(&mut list, |num1, num2| -> bool { num1 > num2 });
        assert_eq!(vec![5, 4, 3, 2, 1], list);
    }
}
