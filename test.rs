struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn create_user(username: String, email: String) -> User{
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn alternate_user(user: User, email: String) -> User{
    User{
        email: email,
        ..user
    }
}

fn main(){
    let mut count = 0;
    'counting_up: loop{
        println!("Count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9{
                break;
            } 
            if count == 2 {
                break 'counting_up;
            }
            remaining-=1
        }
        count+=1;
    }
    println!("End count : {count}");
    println!("");
    
    for ele in (1..4).rev() {
        println!("{ele}");
    }

    let s = String::from("Hey Rithvik!");
    println!("The length of first word is : {}", first_word_index(&s));


    // string slice introduction

    let greet = &s[..3];
    let name = &s[4..(s.len()-1)];
    println!("Greet: {} and Name: {}", greet, name);

    println!("The first word in the string is: {}", first_word(&s));
    println!("The second word in the string is: {}", second_word(&s));
}   

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut count = 0;
    let mut data = (0, 0);
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            count+=1;
            if count==1 {data.0 = i;}
            if count==2 {
                data.1 = i;
                break;
            }
        }
    }
    if count == 2{
        return &s[(1+data.0)..data.1];
    }else if count == 1{
        return &s[(1+data.0)..];
    }else {
        return &"";
    }
}


fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}