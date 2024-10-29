mod problems;
mod util;

fn main() {
    let problem = 29;
    println!("Problem {}: {}", problem, problems::run(problem));
}