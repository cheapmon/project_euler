pub fn run() -> u64 {
    let mut largest = 0;

    for i in (100..=999).rev() {
        for j in (100..=999).rev() {
            let product = i * j;
            let string = product.to_string();
            let reversed: String = string.chars().rev().collect();

            if reversed == string && product > largest {
                largest = product;
            }
        }
    }

    largest
}