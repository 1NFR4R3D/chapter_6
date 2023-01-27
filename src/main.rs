enum IpAddr {
    // V1(i16),         // 16
    V4(u8,u8,u8,u8), // 32
    V6(String), //      64 + 64 = 128
} //                    128

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Message {
    Quit,
    Move {_x: i32, _y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Call hua!")
    }
}

fn main() {
    println!("Chapter 6 - Enums!");
    enum_stuff();
    match_stuff();
}

fn match_stuff() {
    println!("\n\tThe \"match\" statement");
    let monies_penny = Coin::Penny;
    println!("Value is {}",value_in_cents(monies_penny));
    let monies_nickel = Coin::Nickel;
    println!("Value is {}",value_in_cents(monies_nickel));
    let monies_dime = Coin::Dime;
    println!("Value is {}",value_in_cents(monies_dime));
    let monies_quarter = Coin::Quarter(UsState::Alaska);
    println!("Value is {}",value_in_cents(monies_quarter));
    let monies_quarter2 = Coin::Quarter(UsState::Alabama);
    println!("Value is {}",value_in_cents(monies_quarter2));

    println!("\tMatching with Option<T>");
    let five = Some(5);
    println!("{:?}",five);
    let six = plus_one(five);
    println!("{:?}",six);
    let none = plus_one(None);
    println!("{:?}",none);
    dice_roll_proc(3);
    dice_roll_proc(4);
    dice_roll_proc(7);
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny Dreadful!");
            1},
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}",state);
            25},
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Without covering all cases, compiler throws error
        Some(i) => Some(i + 1),
    }
}

fn dice_roll_proc(dice_roll: u8) {
    match dice_roll {
        3 => println!("add_fancy_hat!"),
        7 => println!("remove_fancy_hat!"),
        // _ => println!("reroll 🙁"),
        _ => (),
    }
}

fn enum_stuff() {
    ip_example();
    msg_example();
    opt_example();
}

fn ip_example() {
    println!("Example - storing IP addresses (See source)");
    let _home = IpAddr::V4(127,0,0,1);
    let _loopback = IpAddr::V6(String::from("::1"));
}

fn msg_example() {
    println!("Example - Message enum (See source)");
    let mut m = Message::Write(String::from("hello"));
    m.call();
    m = Message::ChangeColor(255, 255, 255);
    m.call();
    m = Message::Move { _x: 0, _y: 0 };
    m.call();
    m = Message::Quit;
    m.call();
}

fn opt_example() {
    println!("Example - Option enum");
    let test1 = Some("Hello, world!".to_string());
    if test1.is_some() {
        println!("Test1 is something!")
    }
    if test1.is_none() {
        println!("Test1 is nothing!")
    }

    let test2: Option<u32> = None;
    if test2.is_none() {
        println!("Test2 is nothing!")
    }
    if test2.is_some() {
        println!("Test2 is something!")
    }

    // Check documentation, Option has a LOT of methods
}