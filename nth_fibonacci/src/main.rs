use std::io;

fn main() {
    let mut index: String = String::new();
    
    println!("Enter the index of the fibonacci number in the series:");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the index value!");
    
    let index:u64 = index.trim().parse().expect("Invalid index!");

    let fibonacci_number: u64 = 
    if (index == 1) || (index == 2) {index-1}
    else {
        let mut prev: u64 = 0;
        let mut next: u64 = 1;
        let mut temp_index: u64 = index;

        while temp_index-2 != 0 {
            let temp: u64 = next;
            next += prev;
            prev = temp;

            temp_index-=1;
        }
        next
    };

    println!("Fibonacci number at index {index} is {fibonacci_number}");
}
