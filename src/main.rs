mod problems;
mod util;

fn main() {
    let problem = 24;
    println!("Problem {}: {}", problem, problems::run(problem));
}