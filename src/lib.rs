pub mod unsort;

#[cfg(test)]
mod tests {
    use unsort::unsort;

    use super::*;

    #[test]
    fn unsort_merge_power_of_two() {
        let mut test = vec![0, 1, 2, 3, 4, 5, 6, 7];
        unsort(unsort::Algorithm::Merge, &mut test);
        assert_eq!(test, vec![4, 0, 6, 2, 5, 1, 7, 3]);
    }

    #[test]
    fn unsort_merge_irregular() {
        let mut test = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        unsort(unsort::Algorithm::Merge, &mut test);
        assert_eq!(test, vec![8, 4, 0, 6, 2, 5, 1, 7, 3]);
    }
}
