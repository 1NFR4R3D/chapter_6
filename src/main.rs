enum IpAddr {
    // V1(i16),         // 16
    V4(u8,u8,u8,u8), // 32
    V6(String), //      64 + 64 = 128
} //                    128

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
}

fn match_stuff() {
    
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