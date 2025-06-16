fn main() {
    println!("Hello, world!");

    // simple variables 
    let a: i8 = 4;
    let b: i16 = 200;
    let c: i32 = 1000;
    let d: i64 = 100000;
    let e: i128 = 1000000000000000000;

    println!("{}", a);

    // here it looks like we can redifine a variable
    let a: i8 = 5;
    let b: i16 = 230;
    let c: i32 = 5000;
    let d: i64 = 1002400;
    let e: i128 = 1002424000000000000000;

    // similarly we have bool, f8 for float 8bit , u8 for unsigned 8bit , etc
    
    // strings 

    let s = "My string";
    let s1: String = String::from("my string");

    // conditionals and loops

    let b = false;
    if b {
        print!("b is true");
    }else{
        print!("b is false");
    }

    for i in 0..100 {
        println!("{}", i);
    }
}

// Function to get the first word from a sentence
fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for ch in sentence.chars() {
        if ch == ' ' {
            break;
        }
        ans.push(ch);
    }
    ans
}

// Rust sdk is available for zerodha

// Rust sdk is available to mcp servers also

// watched the video on creating a simple webserver for my trading bot in rust

// Realised that mcp servers run locally and the way they work is different than my use case. I thought I could send request to claude but only claude can use the mcp server

// Need to get my hands on rust as fast as possible. 

// I need to start posting more on twitter

// finish the trading bot as fast as possible

// last step is to start marketing paid

// learnt how to build a chrome extension

// started understanding about the asynchronous programming in rust

// learnt about the concept of async executer , wakers ,etc 

// still trying to wrap my head around the concurrency in rust

// understood the concept of state machines and how they are used in async programming

// learnt about the pattern matching in rust.

// learnt about the async/await syntax in rust

// learnt about the futures in rust and how they are used to handle asynchronous programming