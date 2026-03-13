use std::collections::HashMap;

mod median;
mod mode;

mod pig_latin;
mod challenge3;

fn main() {
    // Hash maps store their data on the heap
    {
        let mut scores: HashMap<String, i32> = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values.
    {
        let field_name: String = String::from("Favorite color");
        let field_value: String = String::from("Blue");

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(field_name, field_value);

        // field_name and field_value are invalid at this point, try using them and
        // see what compiler error you get! (Error: value borrowed after move)
        // println!("{field_name}: {field_value}");

        // If we insert references to values into the hash map, the values won't be moved into the hash map.
        // The values that the references point to must be valid for at least as long as the hash map is valid.
    }

    // Overwriting a value
    {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{scores:?}"); // prints {"Blue": 25}
    }

    // Adding a key and value only if a key is not present
    {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{scores:?}");
    }

    // Updating a value based on an old value
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{map:?}");
    }

    let vector_of_integers = &mut vec!{5, 6, 2, 3, 9, 5, 9, 9, 10, 1, 2, 7 };
    
    let median: Option<f32> = median::get_median(vector_of_integers);
    match median {
        Some(value) => println!("Median: {value}"),
        None => println!("Median: n/a")
    }
    
    let mode: Option<i32> = mode::get_mode(vector_of_integers);
    match mode {
        Some(value) => println!("Mode: {value}"),
        None => println!("Mode: n/a")
    };
    
    let word: String = String::from("Stupid");
    let pig_latin_word = pig_latin::to_pig_latin(&word);
    match pig_latin_word {
        Some(value) => println!("{word} is {value} in pig latin."),
        None => println!("Can't convert n/a to pig latin.")
    }
    

    challenge3::challenge3func();


}