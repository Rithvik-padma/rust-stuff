fn main(){
    let x = 5;
    let x = x+2;
    {
        let x = x+5;
        println!("The value of x in the inner scope is : {x}");
    }
    println!("The value of x in the outer scope is : {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces are : {spaces}");
}
