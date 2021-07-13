pub fn quicksort(arr: &mut [i32]) {
    if arr.len() < 3 {
        return;
    }

    let pivot = arr[arr.len() / 2];

    let index = partition(arr, pivot);
    quicksort(&mut arr[..index]);
    quicksort(&mut arr[index..]);
}

fn partition(arr: &mut [i32], pivot: i32) -> usize {
    let mut i = 0usize;
    let mut j = arr.len() - 1;

    loop {
        while arr[i] < pivot {
            i += 1;
        }

        while arr[j] > pivot {
            j -= 1;
        }

        if i >= j || arr[i] == arr[j] {
            break j;
        }

        arr.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use crate::quicksort;

    #[test]
    fn it_sorts_1() {
        let mut arr = vec![15, 3, 7];
        quicksort(&mut arr);
        assert_eq!(&arr[..], [3, 7, 15]);
    }

    #[test]
    fn it_sorts_2() {
        let mut arr = vec![4, 5, 3, 55, 7, 23, 54, 7];
        quicksort(&mut arr);
        assert_eq!(&arr[..], [3, 4, 5, 7, 7, 23, 54, 55]);
    }

    #[test]
    fn it_sorts_3() {
        let mut arr = vec![16, 15, 22, 100, 1, 7, 3, 100];
        quicksort(&mut arr);
        assert_eq!(&arr[..], [1, 3, 7, 15, 16, 22, 100, 100]);
    }
}
