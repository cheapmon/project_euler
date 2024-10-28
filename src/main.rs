mod problems;
mod util;

fn main() {
    let problem = 26;
    println!("Problem {}: {}", problem, problems::run(problem));
}