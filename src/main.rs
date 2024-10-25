mod problems;
mod util;

fn main() {
    let problem = 20;
    println!("Problem {}: {}", problem, problems::run(problem));
}