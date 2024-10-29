mod problems;
mod util;

fn main() {
    let problem = 27;
    println!("Problem {}: {}", problem, problems::run(problem));
}