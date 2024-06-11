use std::io;

fn main(){
    println!("Temperature: ");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp);
    let temp: f32 = temp.trim().parse().expect("Only Float types are allowed!");
    
    println!("What operation do you want to do?");
    println!("1 -> Celsius to Fahrenheit");
    println!("2 -> Fahrenheit to Celsius");     
    println!("Choice: ");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice);
    let choice: usize = choice.trim().parse().expect("Invalid input!");
    
    match choice {
        1 => {
            let temp: f32 = (9.0/5.0)*temp+32.0;
            println!("The temperature in Fahrenheit is : {:.2}F", temp);
        },
        2 => {
            let temp: f32 = (5.0/9.0)*(temp-32.0);
            println!("The temperature in Celsius is : {:.2}°C", temp);
        },
        _ => println!("Only 1 and 2 are allowed as choices!"),
    };
}