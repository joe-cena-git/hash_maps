# Hash Maps — The Rust Programming Language, Chapter 8.3

A hands-on Rust project following [Chapter 8.3: Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) of *The Rust Programming Language*.

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

## Reference

- [The Rust Programming Language — Chapter 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
