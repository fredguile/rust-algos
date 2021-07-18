pub fn quicksort1(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }

    let index = luomo_partition(arr);
    quicksort1(&mut arr[..index]);
    quicksort1(&mut arr[index + 1..]);
}

fn luomo_partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1]; // last
    let mut i = 0;

    for j in 0..arr.len() - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);
    i
}

pub fn quicksort2(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }

    let index = hoare_partition(arr);
    quicksort2(&mut arr[..index + 1]);
    quicksort2(&mut arr[index + 1..]);
}

fn hoare_partition(arr: &mut [i32]) -> usize {
    let pivot_index = ((arr.len() - 1) as f32 / 2.0).floor() as usize; // floor
    let pivot = arr[pivot_index];
    let mut i = -1isize;
    let mut j = arr.len();

    loop {
        loop {
            i += 1;
            if arr[i as usize] >= pivot {
                break;
            }
        }

        loop {
            j -= 1;
            if arr[j] <= pivot {
                break;
            }
        }

        if i as usize >= j {
            break j;
        }

        arr.swap(i as usize, j);
    }
}

#[cfg(test)]
pub mod tests {
    use rand::distributions::Uniform;
    use rand::{self, Rng};

    use super::{quicksort1, quicksort2};

    #[test]
    pub fn quicksort1_works() {
        let rng = &mut rand::thread_rng();

        for _ in 0..5000 {
            let len = rng.gen::<usize>() % 32;
            let mut v = rng
                .sample_iter(Uniform::new(0, 100))
                .take(len + 1)
                .collect::<Vec<i32>>();
            let original = &v.to_owned();

            quicksort1(&mut v);
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

    #[test]
    pub fn quicksort2_works() {
        let rng = &mut rand::thread_rng();

        for _ in 0..5000 {
            let len = rng.gen::<usize>() % 32;
            let mut v = rng
                .sample_iter(Uniform::new(0, 100))
                .take(len + 1)
                .collect::<Vec<i32>>();
            let original = &v.to_owned();

            quicksort2(&mut v);
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
