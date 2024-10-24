use num::ToPrimitive;
use std::collections::HashMap;

pub fn run() -> u64 {
    let list = HashMap::from([
        (1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five"), (6, "six"), (7, "seven"),
        (8, "eight"), (9, "nine"), (10, "ten"), (11, "eleven"), (12, "twelve"), (13, "thirteen"),
        (14, "fourteen"), (15, "fifteen"), (16, "sixteen"), (17, "seventeen"), (18, "eighteen"),
        (19, "nineteen"), (20, "twenty"), (30, "thirty"), (40, "forty"), (50, "fifty"),
        (60, "sixty"), (70, "seventy"), (80, "eighty"), (90, "ninety")
    ]);

    let print_tens_and_digits = |i: u64| -> Option<String> {
        let i = i % 100;

        if i == 0 {
            None
        } else if i <= 20 || i % 10 == 0 {
            Some(format!("{}", list[&i]))
        } else {
            let a = i / 10 * 10;
            let b = i % 10;
            Some(format!("{}-{}", list[&a], list[&b]))
        }
    };

    let print_hundreds = |i: u64| -> Option<String> {
        let i = i % 1_000;
        let a = i / 100;
        let tens_and_digits = print_tens_and_digits(i);

        if i == 0 {
            None
        } else if a == 0 {
            tens_and_digits
        } else if tens_and_digits.is_none() {
            Some(format!("{} hundred", list[&a]))
        } else {
            Some(format!("{} hundred and {}", list[&a], tens_and_digits.unwrap()))
        }
    };

    let print_thousands = |i: u64| -> Option<String> {
        let i = i % 10_000;
        let a = i / 1000;
        let hundreds = print_hundreds(i);

        if i == 0 {
            None
        } else if a == 0 {
            hundreds
        } else if hundreds.is_none() {
            Some(format!("{} thousand", list[&a]))
        } else {
            Some(format!("{} thousand {}", list[&a], hundreds.unwrap()))
        }
    };

    let mut sum = 0;

    for i in 1..=1000 {
        let s = print_thousands(i).unwrap();
        sum += s.chars().filter(|c| !c.is_whitespace() && *c != '-').count();
    }

    sum.to_u64().unwrap()
}