mod problems;
mod util;

fn main() {
    let problem = 23;
    println!("Problem {}: {}", problem, problems::run(problem));
}