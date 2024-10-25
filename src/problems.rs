mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;
mod problem_5;
mod problem_6;
mod problem_7;
mod problem_8;
mod problem_9;
mod problem_10;
mod problem_11;
mod problem_12;
mod problem_13;
mod problem_14;
mod problem_15;
mod problem_16;
mod problem_17;
mod problem_18;
mod problem_19;
mod problem_20;
mod problem_67;

pub fn run(i: usize) -> u64 {
    match i {
        1 => problem_1::run(),
        2 => problem_2::run(),
        3 => problem_3::run(),
        4 => problem_4::run(),
        5 => problem_5::run(),
        6 => problem_6::run(),
        7 => problem_7::run(),
        8 => problem_8::run(),
        9 => problem_9::run(),
        10 => problem_10::run(),
        11 => problem_11::run(),
        12 => problem_12::run(),
        13 => problem_13::run(),
        14 => problem_14::run(),
        15 => problem_15::run(),
        16 => problem_16::run(),
        17 => problem_17::run(),
        18 => problem_18::run(),
        19 => problem_19::run(),
        20 => problem_20::run(),
        67 => problem_67::run(),
        _ => 0,
    }
}