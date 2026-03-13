use std::collections::HashMap;

pub fn get_mode(vector_of_integers: &Vec<i32>) -> Option<i32> {
    let vector_length: usize = vector_of_integers.len();
    if vector_length == 0 {
        // vector has no elements
        return None;
    } else if vector_length == 1 {
        // vector has one element, return that element
        return Some(vector_of_integers[0]);
    } else {
        // vector has more than 1 element

        // hash map of <i, count of i> in vector
        let mut mode_hashmap: HashMap<i32, usize> = HashMap::new();

        for i in vector_of_integers {
            // if i does not exist in the hash map, add it as a key, with a count of 0
            let count = mode_hashmap.entry(*i).or_insert(0);

            // dereference the count (go to the house at the address of count) and increment it
            *count += 1;
        }

        // display hash map keys and values
        let mut most_common_value: Option<i32> = None;
        let mut highest_frequency: usize = 0;
        for (k, v) in mode_hashmap {
            if v > highest_frequency {
                highest_frequency = v;
                most_common_value = Some(k);
            }
        }

        return most_common_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vector_returns_none() {
        assert_eq!(get_mode(&vec![]), None);
    }

    #[test]
    fn single_element_returns_that_element() {
        assert_eq!(get_mode(&vec![42]), Some(42));
    }

    #[test]
    fn all_same_elements_returns_that_element() {
        assert_eq!(get_mode(&vec![7, 7, 7, 7]), Some(7));
    }

    #[test]
    fn clear_single_mode() {
        assert_eq!(get_mode(&vec![1, 2, 2, 3]), Some(2));
    }

    #[test]
    fn mode_at_start_of_vector() {
        assert_eq!(get_mode(&vec![5, 5, 5, 1, 2, 3]), Some(5));
    }

    #[test]
    fn mode_at_end_of_vector() {
        assert_eq!(get_mode(&vec![1, 2, 3, 9, 9, 9]), Some(9));
    }

    #[test]
    fn negative_numbers_mode() {
        assert_eq!(get_mode(&vec![-3, -3, -1, 0, 2]), Some(-3));
    }

    #[test]
    fn large_vector_with_clear_mode() {
        let mut v = vec![1; 1000];
        v.extend(vec![2; 999]);
        assert_eq!(get_mode(&v), Some(1));
    }

    #[test]
    fn two_elements_same_value() {
        assert_eq!(get_mode(&vec![4, 4]), Some(4));
    }

    #[test]
    fn two_elements_different_values_mode_is_one_of_them() {
        // With only two distinct values each appearing once, the result must
        // be one of the two values (tie-breaking is implementation-defined).
        let result = get_mode(&vec![3, 7]);
        assert!(result == Some(3) || result == Some(7));
    }
}
