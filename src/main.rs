mod problems;
mod util;

fn main() {
    let problem = 31;
    println!("Problem {}: {}", problem, problems::run(problem));
}