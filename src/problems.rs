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
mod problem_21;
mod problem_22;
mod problem_23;
mod problem_24;
mod problem_25;
mod problem_26;
mod problem_27;
mod problem_67;

pub fn run(i: usize) -> String {
    match i {
        1 => problem_1::run().to_string(),
        2 => problem_2::run().to_string(),
        3 => problem_3::run().to_string(),
        4 => problem_4::run().to_string(),
        5 => problem_5::run().to_string(),
        6 => problem_6::run().to_string(),
        7 => problem_7::run().to_string(),
        8 => problem_8::run().to_string(),
        9 => problem_9::run().to_string(),
        10 => problem_10::run().to_string(),
        11 => problem_11::run().to_string(),
        12 => problem_12::run().to_string(),
        13 => problem_13::run().to_string(),
        14 => problem_14::run().to_string(),
        15 => problem_15::run().to_string(),
        16 => problem_16::run().to_string(),
        17 => problem_17::run().to_string(),
        18 => problem_18::run().to_string(),
        19 => problem_19::run().to_string(),
        20 => problem_20::run().to_string(),
        21 => problem_21::run().to_string(),
        22 => problem_22::run().to_string(),
        23 => problem_23::run().to_string(),
        24 => problem_24::run().to_string(),
        25 => problem_25::run().to_string(),
        26 => problem_26::run().to_string(),
        27 => problem_27::run().to_string(),
        67 => problem_67::run().to_string(),
        _ => String::new(),
    }
}