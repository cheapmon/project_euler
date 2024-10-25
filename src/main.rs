mod problems;
mod util;

fn main() {
    let problem = 21;
    println!("Problem {}: {}", problem, problems::run(problem));
}