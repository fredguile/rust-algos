pub fn selectionsort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        let mut min = i;
        for j in i..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }

        if min != i {
            arr.swap(i, min);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use rand::distributions::Uniform;
    use rand::{self, thread_rng, Rng};

    use super::selectionsort;

    #[test]
    pub fn selectionsort_works() {
        let rng = &mut thread_rng();

        for _ in 0..5000 {
            let len = rng.gen::<usize>() % 32;
            let mut v = rng
                .sample_iter(Uniform::new(0, 100))
                .take(len + 1)
                .collect::<Vec<i32>>();
            let original = &v.to_owned();

            selectionsort(&mut v);
            for i in 0..v.len() - 1 {
                assert!(
                    v[i] <= v[i + 1],
                    "array not sorted: {:?} got instead: {:?}",
                    &original,
                    &v
                );
            }
        }
    }
}
