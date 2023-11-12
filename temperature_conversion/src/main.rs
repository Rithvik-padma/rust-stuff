use std::io;

fn main() {
    let mut temp_type: String = String::new();
    let mut temp_to_convert: String = String::new();

    println!("Enter the type of temperature to convert");
    println!("For Fahrheneit: input F");
    println!("For Celsius: input C");
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Unable to read the temperature type!");

    println!("Enter the temperture value: ");
    io::stdin()
        .read_line(&mut temp_to_convert)
        .expect("Unable to read the temperature value!");

    let temp_to_convert:f32 = temp_to_convert.trim().parse().expect("Invalid Type!");

    let converted_temp:f32 = 
    if (temp_type.trim() == "F") || (temp_type.trim() == "f") {((temp_to_convert-32.0)*5.0)/9.0}
    else if (temp_type.trim() == "C") || (temp_type.trim() == "c") {(9.0*temp_to_convert/5.0)+32.0}
    else {f32::NAN};

    if !converted_temp.is_nan() {println!("Converted temperature value: {converted_temp}");}
    else {println!("Invalid temperature type!")}
}
