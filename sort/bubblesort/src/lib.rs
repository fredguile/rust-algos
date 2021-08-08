pub fn bubblesort(arr: &mut [i32]) {
    let mut counter = 0usize;

    loop {
        let mut swapped = false;

        for i in 1..arr.len() - counter {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }

        counter += 1;

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use rand::distributions::Uniform;
    use rand::{self, thread_rng, Rng};

    use super::bubblesort;

    #[test]
    pub fn bubblesort_works() {
        let rng = &mut thread_rng();

        for _ in 0..5000 {
            let len = rng.gen::<usize>() % 32;
            let mut v = rng
                .sample_iter(Uniform::new(0, 100))
                .take(len + 1)
                .collect::<Vec<i32>>();
            let original = &v.to_owned();

            bubblesort(&mut v);
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
