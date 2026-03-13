# Hash Maps — Rust Chapter 8.3

A hands-on Rust project following [Chapter 8.3: Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).

## What this covers

The `main.rs` file walks through every core concept from the chapter with annotated examples:

| Concept | Description |
|---|---|
| Creating a hash map | `HashMap::new()` and `.insert()` |
| Ownership rules | `Copy` types are copied in; owned types like `String` are moved |
| Iterating | `for (key, value) in &map` |
| Overwriting a value | Re-inserting an existing key replaces its value |
| `entry` / `or_insert` | Insert only if the key is absent |
| Updating based on old value | Using `or_insert` to get a mutable reference and increment a counter |

## Modules

### `median` — `src/median.rs`
Computes the **median** of a `Vec<i32>`, returned as `Option<f32>`.

- Sorts the vector in place before calculating.
- Returns the middle element for odd-length vectors.
- Returns the average of the two middle elements for even-length vectors.
- Returns `None` for an empty vector.

### `mode` — `src/mode.rs`
Computes the **mode** (most frequently occurring value) of a `Vec<i32>`, returned as `Option<i32>`.

- Uses a `HashMap<i32, usize>` to count occurrences of each value.
- Returns `None` for an empty vector.
- Tie-breaking between equally frequent values is not guaranteed (HashMap iteration order is non-deterministic).

### `challenge2` — `src/challenge2.rs`
*In progress.* Pig Latin converter — the first end-of-chapter exercise from 8.3:
> Convert strings to Pig Latin. The first consonant of each word is moved to the end and *-ay* is appended (`first` → `irst-fay`). Words starting with a vowel get *-hay* appended (`apple` → `apple-hay`).

### `challenge3` — `src/challenge3.rs`
*In progress.* Employee directory — the second end-of-chapter exercise from 8.3:
> Build a text interface to add employees to departments (e.g. `"Add Sally to Engineering"`), and retrieve all people in a department or the whole company, sorted alphabetically.

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

Tests are colocated with each module. `median` and `mode` each have 10 unit tests covering edge cases such as empty input, single elements, negative numbers, even/odd lengths, and large vectors.

```
median
  ▼ tests
    ✔ median_empty_vector_returns_none
    ✔ median_all_identical_elements
    ✔ median_even_count_non_integer_result
    ✔ median_even_count_returns_average_of_two_middle_elements
    ✔ median_negative_numbers
    ✔ median_single_element_returns_that_element
    ✔ median_mix_of_negative_and_positive_even_count
    ✔ median_odd_count_returns_middle_element
    ✔ median_two_elements_returns_average
    ✔ median_unsorted_input_is_sorted_before_calculating
▼ mode
  ▼ tests
    ✔ empty_vector_returns_none
    ✔ clear_single_mode
    ✔ all_same_elements_returns_that_element
    ✔ negative_numbers_mode
    ✔ single_element_returns_that_element
    ✔ mode_at_end_of_vector
    ✔ mode_at_start_of_vector
    ✔ two_elements_different_values_mode_is_one_of_them
    ✔ two_elements_same_value
    ✔ large_vector_with_clear_mode
```

## Reference

- [The Rust Programming Language — Chapter 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
