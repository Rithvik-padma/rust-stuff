fn main() {
    let new_lines = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let days_number = [
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eigth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    println!("The first day of Christmas,");
    println!("My true love lent to me");
    println!("A partridge in a pear tree.");
        
    for i in 0..11 {
        println!("\nThe {} day of Christmas,", days_number[i]);
        println!("My true love lent to me");
        for j in (0..=i).rev() {println!("{},", new_lines[j]);}
        println!("and, A partridge in a pear tree.")
    }
}
