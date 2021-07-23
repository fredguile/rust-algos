use std::vec::Vec;

pub fn mergesort(arr: &[i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mid_index = (arr.len() as f32 / 2.0).floor() as usize;

    let left = mergesort(&arr[..mid_index]);
    let right = mergesort(&arr[mid_index..]);

    merge(&left, &right)
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0usize;
    let mut j = 0usize;

    loop {
        if i == left.len() && j == right.len() {
            break result;
        }

        if i == left.len() {
            result.push(right[j]);
            j += 1;
            continue;
        }

        if j == right.len() {
            result.push(left[i]);
            i += 1;
            continue;
        }

        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use rand::distributions::Uniform;
    use rand::{self, thread_rng, Rng};

    use super::mergesort;

    #[test]
    pub fn mergesort_works() {
        let rng = &mut thread_rng();

        for _ in 0..5000 {
            let len = rng.gen::<usize>() % 32;
            let v = rng
                .sample_iter(Uniform::new(0, 100))
                .take(len + 1)
                .collect::<Vec<i32>>();

            let result = mergesort(&v);
            for i in 0..result.len() - 1 {
                assert!(
                    result[i] <= result[i + 1],
                    "array not sorted: {:?} got instead: {:?}",
                    &v,
                    &result
                );
            }
        }
    }
}
