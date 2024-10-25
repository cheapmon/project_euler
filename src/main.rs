mod problems;
mod util;

fn main() {
    let problem = 22;
    println!("Problem {}: {}", problem, problems::run(problem));
}