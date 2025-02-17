fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let number = numbers.get(10).unwrap(); // This will panic!
    println!("The number is: {}", number);
}