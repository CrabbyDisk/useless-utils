use std::{fmt::Debug, ptr::slice_from_raw_parts};
#[derive(PartialEq, Clone)]
pub enum Algorithm {
    Merge,
    Bubble,
    Heap,
    Quick(Pivot),
}

#[derive(PartialEq, Clone)]
pub enum Pivot {
    Center,
    Left,
    Right,
}

pub fn unsort<T: Ord + Copy + Debug>(alg: Algorithm, list: &mut [T]) {
    match alg {
        Algorithm::Merge => {
            unsort_merge(list);
        }
        Algorithm::Bubble => unsort_rev(list),
        Algorithm::Heap => unsort_rev(list),
        Algorithm::Quick(Pivot::Left) => {
            list.sort();
        }
        Algorithm::Quick(Pivot::Right) => unsort_rev(list),
        Algorithm::Quick(Pivot::Center) => todo!(),
    }
}

#[cfg(feature = "undeterministic")]
pub fn unsort_undeterministic<T: Ord + Copy + Debug>(alg: Algorithm, list: &mut [T]) {
    use rand::{seq::SliceRandom, thread_rng};
    let mut rng = thread_rng();

    list.shuffle(&mut rng);
    match alg {
        Algorithm::Merge => {
            unsort_merge(list);
        }
        Algorithm::Bubble => unsort_rev(list),
        Algorithm::Heap => unsort_rev(list),
        Algorithm::Quick(Pivot::Left) => {
            list.sort();
        }
        Algorithm::Quick(Pivot::Right) => unsort_rev(list),
        Algorithm::Quick(Pivot::Center) => todo!(),
    }
}
fn unsort_rev<T: Ord + Clone + Debug>(list: &mut [T]) {
    list.sort();
    list.reverse();
}

/* fn unsort_quick_center<T: Ord + Clone + Debug>(mut arr: &[T]) -> &[T] {
    if arr.len() <= 1 {
        return arr;
    }

    // Find the largest element to use as the pivot
    let max_element = *arr.iter().max().unwrap();

    // Partition the array into left and right subarrays
    let left: Vec<T> = arr.iter().cloned().filter(|&x| x < max_element).collect();
    let mid = arr.len() / 2;
    let right: Vec<T> = arr.iter().cloned().filter(|&x| x > max_element).collect();

    // Recursively generate worst-case subarrays
    let mut result = unsort_quick_center(&left);
    result.push(max_element);
    result.extend(unsort_quick_center(&right));
    result
} */

fn merge<T: Ord + Clone + Copy>(arr: &mut [T], left: &[T], right: &[T]) {
    let left_len = left.len();
    let right_len = right.len();

    arr[..left_len].copy_from_slice(left);
    arr[left_len..left_len + right_len].copy_from_slice(right);
}

// Pass a sorted array here
fn unsort_merge<T: Ord + Clone + Copy>(arr: &mut [T]) {
    if arr.len() == 1 {
    } else if arr.len() == 2 {
        arr.swap(0, 1);
    } else {
        let m = (arr.len() + 1) / 2;
        let mut left = Vec::with_capacity(m);
        let mut right = Vec::with_capacity(arr.len() - m);

        for j in (0..arr.len()).step_by(2) {
            left.push(arr[j]);
        }

        for j in (1..arr.len()).step_by(2) {
            right.push(arr[j]);
        }

        unsort_merge(&mut left);
        unsort_merge(&mut right);
        merge(arr, &left, &right);
    }
}
