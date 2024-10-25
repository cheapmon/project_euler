pub(super) fn run() -> u64 {
    let mut names: Vec<String> = std::fs::read_to_string("resources/0022_names.txt").unwrap()
        .split(",")
        .map(|name| name.replace("\"", ""))
        .collect();
    names.sort();
    names.iter()
        .enumerate()
        .map(|(index, name)| score_name(&name) * (index + 1) as u64)
        .sum()
}

fn score_name(name: &String) -> u64 {
    name.chars().map(|c| c as u64 - 65 + 1).sum()
}