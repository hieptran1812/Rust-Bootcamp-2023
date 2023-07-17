// Exercise 1
// Make it compile
fn exercise1() {
    let x = String::from("hello, world");
    let y = x.clone();
    let z = x.clone();
}


// Exercise 2
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}   
// Only modify the code below!
fn take_ownership(s: String) -> String{
    println!("{}", s);
    s
}

// Exercise 3
// Make it compile
// Dont care about logic
fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let additions: Vec<usize> = vec![]; // Initialize with an empty vector

    println!("{:?}", values_number);

    while additions.len() > 0 {
        let mut addition: f64 = 0.0;

        // Sum values in additions
        for &element_index in &additions { // Add '&' before 'additions' to borrow the values
            let addition_aux = values[element_index];
            addition = addition_aux + addition;
        }

        // Break the loop or modify the additions vector
        break;
    }
}


// Exercise 4
// Make it compile
fn exercise4(value: u32) -> &'static str {
    let str_value = value.to_string();
    let static_str: &'static str = Box::leak(str_value.into_boxed_str());
    static_str
}

// Exercise 5
// Make it compile
use std::collections::HashMap;

fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;

    let res = match my_map.get(&key) {
        Some(child) => child,
        None => {
            let value = "3.0".to_string();
            my_map.insert(key, value.clone());
            &value
        }
    };

    println!("{}", res);
}



// Exercise 6
// Make it compile

use std::io::{self};

fn exercise8() {
    let mut accounting = vec!["Alice".to_string(), "Ben".to_string()];

    loop {
        let mut add_input = String::new();

        if io::stdin().read_line(&mut add_input).expect("Failed to read line") == 0 {
            println!("Incorrect input, try again");
            continue;
        }

        let add_vec: Vec<&str> = add_input.trim().split_whitespace().collect();

        if add_vec.is_empty() {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0].to_string();
        accounting.push(person);
    }
}
