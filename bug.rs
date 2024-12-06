fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 10;

    // This will panic at runtime because index is out of bounds
    let value = vec[index];
    println!("Value at index {}: {}", index, value);
}