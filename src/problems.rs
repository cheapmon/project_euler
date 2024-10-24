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

pub fn run(i: usize) -> u64 {
    let problems = vec![
        problem_1::run,
        problem_2::run,
        problem_3::run,
        problem_4::run,
        problem_5::run,
        problem_6::run,
        problem_7::run,
        problem_8::run,
        problem_9::run,
        problem_10::run,
        problem_11::run,
        problem_12::run,
        problem_13::run,
        problem_14::run,
        problem_15::run,
        problem_16::run,
        problem_17::run,
        problem_18::run,
    ];

    problems[i - 1]()
}