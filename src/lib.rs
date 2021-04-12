fn quicksort<T>(arr: &mut Vec<T>, low: i32, high: i32)
where
    T: PartialOrd + Copy,
{
    if low < high {
        let p = partition(arr, low, high);
        quicksort(arr, low, p - 1); // before partition
        quicksort(arr, p + 1, high) // after partition
    }
}

fn partition<T>(arr: &mut Vec<T>, low: i32, high: i32) -> i32
where
    T: PartialOrd + Copy,
{
    let mut pivot = arr[high as usize]; // pivot
    let mut i = low - 1; // right position of pivot
    let mut j = low;
    while j < high {
        // if current is smaller than pivot
        if arr[j as usize] < pivot {
            i += 1;
            swap(arr, i, j);
        }
        j += 1;
    }
    i += 1;
    // put pivot in correct spot and new pivot is swaped in
    swap(arr, i, high);
    i
}

// helper function for swaping vec values
fn swap<T>(arr: &mut Vec<T>, pos_1: i32, pos_2: i32)
where
    T: PartialOrd + Copy,
{
    let temp = arr[pos_1 as usize];
    arr[pos_1 as usize] = arr[pos_2 as usize];
    arr[pos_2 as usize] = temp;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quicksort_test() {
        let mut numbers:Vec<i8> = vec![4, 3, 7, 5, 6, 9, 1, 8];
        let last = numbers.len() - 1;
        quicksort(&mut numbers, 0, last as i32);
        let sorted_numbers: Vec<i8> = vec![1, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sorted_numbers, numbers);
    }
    #[test]
    fn swap_test() {
        let mut numbers:Vec<i8>  = vec![4, 3, 6];
        swap(&mut numbers, 0, 2);
        let swaped_numbers:Vec<i8>  = vec![6, 3, 4];
        assert_eq!(swaped_numbers, numbers);
    }
    #[test]
    fn partition_test() {
        let mut numbers:Vec<i8>  = vec![4, 2, 6, 3, 8, 5];
        let last = numbers.len() - 1;
        partition(&mut numbers, 0, last as i32);
        let partitioned_numbers:Vec<i8>  = vec![4, 2, 3, 5, 8, 6];
        assert_eq!(partitioned_numbers, numbers);
    }
}
