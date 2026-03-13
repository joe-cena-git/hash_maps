pub fn get_median(vector_of_integers: &mut Vec<i32>) -> Option<f32> {
    // sort vector
    vector_of_integers.sort();

    // count number of integers in the vector
    let vector_length: usize = vector_of_integers.len();

    if vector_length == 0 {
        // vector is empty, return none
        return None;
    } else if vector_length == 1 {
        // vector has only one element, return that element
        return Some(vector_of_integers[0] as f32);
    } else {
        // vector has more than 1 element
        let midpoint: usize = vector_length / 2;
        let midpoint_value: f32 = vector_of_integers[midpoint] as f32;
        if vector_length % 2 == 0 {
            // vector has an even number of elements
            let midpoint_lower_value: f32 = vector_of_integers[midpoint - 1] as f32;
            let median: f32 = (midpoint_value + midpoint_lower_value) / 2.0;
            return Some(median);
        } else {
            // vector has an odd number of elements
            return Some(midpoint_value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_empty_vector_returns_none() {
        let result = get_median(&mut vec![]);
        assert_eq!(result, None);
    }

    #[test]
    fn median_single_element_returns_that_element() {
        let result = get_median(&mut vec![7]);
        assert_eq!(result, Some(7.0));
    }

    #[test]
    fn median_odd_count_returns_middle_element() {
        // sorted: [1, 3, 5] — middle is index 1
        let result = get_median(&mut vec![5, 1, 3]);
        assert_eq!(result, Some(3.0));
    }

    #[test]
    fn median_even_count_returns_average_of_two_middle_elements() {
        // sorted: [1, 2, 3, 4] — middle two are 2 and 3, average is 2.5
        let result = get_median(&mut vec![4, 1, 3, 2]);
        assert_eq!(result, Some(2.5));
    }

    #[test]
    fn median_unsorted_input_is_sorted_before_calculating() {
        // Same values in a different order should give the same result
        let result_a = get_median(&mut vec![10, 1, 5]);
        let result_b = get_median(&mut vec![1, 5, 10]);
        assert_eq!(result_a, result_b);
    }

    #[test]
    fn median_two_elements_returns_average() {
        // Smallest possible even-length case: [1, 3] → (1 + 3) / 2 = 2.0
        let result = get_median(&mut vec![3, 1]);
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn median_all_identical_elements() {
        // Every element is the same, so median must equal that value
        let result = get_median(&mut vec![5, 5, 5, 5, 5]);
        assert_eq!(result, Some(5.0));
    }

    #[test]
    fn median_negative_numbers() {
        // sorted: [-5, -3, -1] — middle is -3
        let result = get_median(&mut vec![-1, -5, -3]);
        assert_eq!(result, Some(-3.0));
    }

    #[test]
    fn median_mix_of_negative_and_positive_even_count() {
        // sorted: [-2, -1, 1, 2] — average of middle two: (-1 + 1) / 2 = 0.0
        let result = get_median(&mut vec![1, -1, 2, -2]);
        assert_eq!(result, Some(0.0));
    }

    #[test]
    fn median_even_count_non_integer_result() {
        // sorted: [1, 2, 3, 4, 5, 6] — average of 3 and 4 = 3.5
        // confirms the result is a true f32 average, not truncated
        let result = get_median(&mut vec![6, 3, 1, 4, 2, 5]);
        assert_eq!(result, Some(3.5));
    }
}
