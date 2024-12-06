fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;

    //Check if the index is within bounds before accessing
    match vec.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    };
} 
