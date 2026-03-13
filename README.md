# Hash Maps — Rust Chapter 8.3

A hands-on Rust project following [Chapter 8.3: Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).

## Output

```
Yellow: 50
Blue: 10
{"Blue": 25}
{"Blue": 10, "Yellow": 50}
{"hello": 1, "wonderful": 1, "world": 2}
Median: 5.5
Mode: 9
Stupid is Tupid-say in pig latin.
E: Bob, D: Sales
E: Sally, D: Engineering
E: Amir, D: Sales
Sales Department:
- Amir
- Bob
=======================
ALL DEPARTMENTS REPORT:
----------------------
Sales Department:
- Amir
- Bob
Engineering Department:
- Sally
=======================
```

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
| Median & mode | End-of-chapter exercise 1: stats on a `Vec<i32>` using sorting and a frequency `HashMap` |
| Pig Latin | End-of-chapter exercise 2: string transformation with Unicode-aware character handling |
| Employee directory | End-of-chapter exercise 3: nested `HashMap<String, Vec>` with binary search insertion |

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

### `pig_latin` — `src/pig_latin.rs`
Pig Latin converter — the second end-of-chapter exercise from 8.3.

- Consonant-initial words: moves the first consonant to the end and appends *-ay* (`first` → `irst-fay`).
- Vowel-initial words: appends *-hay* (`apple` → `apple-hay`).
- Preserves capitalization of the original word.
- Returns `None` for an empty string.

### `directory` — `src/directory.rs`
Employee directory — the third end-of-chapter exercise from 8.3.

- Accepts text commands to add employees to departments (e.g. `"Add Sally to Engineering"`).
- Retrieves all people in a given department, sorted alphabetically.
- Retrieves all employees across the whole company by department, sorted alphabetically.
- Uses binary search insertion to maintain sorted order at storage time — O(n) per insert vs O(n log n) for push + sort.

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

Tests are colocated with each module in their respective `#[cfg(test)]` blocks.

### 1. `median` tests — `src/median.rs`

10 unit tests covering empty input, odd/even lengths, identical elements, negative numbers, unsorted input, and floating-point averages.

```
▼ median
  ▼ tests
    ✔ median_empty_vector_returns_none
    ✔ median_single_element_returns_that_element
    ✔ median_two_elements_returns_average
    ✔ median_odd_count_returns_middle_element
    ✔ median_even_count_returns_average_of_two_middle_elements
    ✔ median_even_count_non_integer_result
    ✔ median_all_identical_elements
    ✔ median_negative_numbers
    ✔ median_mix_of_negative_and_positive_even_count
    ✔ median_unsorted_input_is_sorted_before_calculating
```

### 2. `mode` tests — `src/mode.rs`

10 unit tests covering empty input, single elements, all-identical elements, mode position in the vector, negative numbers, two-element vectors, and large inputs.

```
▼ mode
  ▼ tests
    ✔ empty_vector_returns_none
    ✔ single_element_returns_that_element
    ✔ two_elements_same_value
    ✔ two_elements_different_values_mode_is_one_of_them
    ✔ all_same_elements_returns_that_element
    ✔ clear_single_mode
    ✔ mode_at_start_of_vector
    ✔ mode_at_end_of_vector
    ✔ negative_numbers_mode
    ✔ large_vector_with_clear_mode
```

### 3. `pig_latin` tests — `src/pig_latin.rs`

10 unit tests covering empty input, vowel words, consonant words, single-letter words, and capitalization handling (lowercase, title case, all-caps).

```
▼ pig_latin
  ▼ tests
    ✔ empty_string_returns_none
    ✔ single_vowel_letter_returns_hay_suffix
    ✔ single_consonant_letter_returns_consonant_word
    ✔ lowercase_vowel_word_returns_hay_suffix
    ✔ lowercase_consonant_word_returns_lowercase_consonant_word
    ✔ two_letter_consonant_word_returns_consonant_word
    ✔ capital_vowel_word_returns_capital_vowel_word
    ✔ capital_consonant_word_returns_capital_consonant_word
    ✔ uppercase_vowel_word_preserves_capitalization_with_hay_suffix
    ✔ all_caps_consonant_word_capitalizes_first_letter_of_result
```

### 4. `directory` tests — `src/directory.rs`

10 unit tests covering empty directory, adding employees to new and existing departments, command parsing, robustness against malformed input, and alphabetical ordering.

```
▼ directory
  ▼ tests
    ✔ new_creates_empty_directory
    ✔ add_employee_creates_new_department
    ✔ add_employee_stores_correct_name
    ✔ add_second_employee_to_same_department
    ✔ add_employees_to_different_departments
    ✔ send_text_command_adds_employee
    ✔ send_text_command_appends_to_existing_department
    ✔ send_text_command_empty_string_does_not_panic
    ✔ send_text_command_missing_department_does_not_panic
    ✔ employees_in_department_are_sorted_alphabetically
```

## Reference

- [The Rust Programming Language — Chapter 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
