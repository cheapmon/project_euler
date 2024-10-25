mod problems;
mod util;

fn main() {
    let problem = 25;
    println!("Problem {}: {}", problem, problems::run(problem));
}