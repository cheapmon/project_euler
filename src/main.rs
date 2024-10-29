mod problems;
mod util;

fn main() {
    let problem = 30;
    println!("Problem {}: {}", problem, problems::run(problem));
}