mod problems;
mod util;

fn main() {
    let problem = 28;
    println!("Problem {}: {}", problem, problems::run(problem));
}