fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    match numbers.get(10) {
        Some(number) => println!("The number is: {}", number),
        None => println!("Index out of bounds"),
    }
} 