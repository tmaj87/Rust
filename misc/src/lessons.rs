extern crate core;

use std::str;

struct SomeStruct {
    odd: bool,
    value: i32,
}

impl SomeStruct {
    fn is_positive(&self) -> bool {
        self.value > 0
    }
}

fn the_basics() {
    let _ = throwaway_value();

    let pair = ('a', 99);
    println!("pair is {:?}", pair);

    let y = SomeStruct {
        odd: false,
        value: 2,
    };
    println!("{}", y.is_positive());
}

fn throwaway_value() {
    || return;
}

fn print_number(n: i32) {
    match n {
        1 => println!("one"),
        2 => println!("two"),
        3 | 13 | 14 => println!("happy hours"),
        4..=12 => println!("for rust"),
        _ => println!("{n}"),
    }
}

fn present_me_all_the_options() {
    let opt: Option<i32> = Some(5);
    if opt.is_none() {
        return;
    }
    opt.unwrap();
}

fn utf8_return_type() {
    let melon = &[240, 159, 141, 137];
    match str::from_utf8(melon) {
        Ok(s) => println!("The string is {s}"),
        Err(e) => panic!("{e}"),
    }
    if let Ok(s) = str::from_utf8(melon) {
        println!("The string is {s}");
    }
}

fn iterate_every_bit_of_world() {
    for n in (665..999).filter(|x| x % 2 == 0) {
        print!("{} -> ", n);
    }
}

fn concurrency_in_rust() {
    "You got to see this man! Tokyo!";
}
