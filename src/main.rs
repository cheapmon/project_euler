mod util;

fn main() {
    // println!("Problem 1: {}", problem_1());
    // println!("Problem 2: {}", problem_2());
    // println!("Problem 3: {}", problem_3());
    // println!("Problem 4: {}", problem_4());
    // println!("Problem 5: {}", problem_5());
    // println!("Problem 6: {}", problem_6());
    // println!("Problem 7: {}", problem_7());
    // println!("Problem 8: {}", problem_8());
    // println!("Problem 9: {}", problem_9());
    // println!("Problem 10: {}", problem_10());
    println!("Problem 11: {}", problem_11());
}

// Problem 1
fn problem_1() -> u64 {
    (1..1000)
        .filter(|i| i % 3 == 0 || i % 5 == 0)
        .sum()
}

// Problem 2
fn problem_2() -> u64 {
    util::fib().take_while(|i| *i < 4_000_000).filter(|i| *i % 2 == 0).sum()
}

// Problem 3
fn problem_3() -> u64 {
    let num: u64 = 600_851_475_143;
    let upper_bound: u64 = (num as f64).sqrt() as u64 + 1;

    (2..=upper_bound).rev().find(|i| util::is_prime(*i) && num % i == 0).unwrap()
}

// Problem 4
fn problem_4() -> u64 {
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

// Problem 5
fn problem_5() -> u64 {
    232_792_560
}

// Problem 6
fn problem_6() -> u64 {
    let mut sum_of_squares = 0u64;
    let mut square_of_sum = 0u64;

    for i in 1..=100 {
        sum_of_squares += i * i;
        square_of_sum += i;
    }

    square_of_sum *= square_of_sum;
    square_of_sum - sum_of_squares
}

// Problem 7
fn problem_7() -> u64 {
    util::primes().nth(10000).unwrap()
}

// Problem 8
fn problem_8() -> u64 {
    let mut s = String::from("
        73167176531330624919225119674426574742355349194934
        96983520312774506326239578318016984801869478851843
        85861560789112949495459501737958331952853208805511
        12540698747158523863050715693290963295227443043557
        66896648950445244523161731856403098711121722383113
        62229893423380308135336276614282806444486645238749
        30358907296290491560440772390713810515859307960866
        70172427121883998797908792274921901699720888093776
        65727333001053367881220235421809751254540594752243
        52584907711670556013604839586446706324415722155397
        53697817977846174064955149290862569321978468622482
        83972241375657056057490261407972968652414535100474
        82166370484403199890008895243450658541227588666881
        16427171479924442928230863465674813919123162824586
        17866458359124566529476545682848912883142607690042
        24219022671055626321111109370544217506941658960408
        07198403850962455444362981230987879927244284909188
        84580156166097919133875499200524063689912560717606
        05886116467109405077541002256983155200055935729725
        71636269561882670428252483600823257530420752963450
    ");

    s.retain(|c| !c.is_whitespace());
    s.chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .windows(13)
        .map(|window| window.iter().product())
        .max()
        .unwrap()
}

// Problem 9
fn problem_9() -> u64 {
    for i in 1..=1000 {
        for j in i..=1000 {
            for k in j..=1000 {
                if (i * i) + (j * j) == (k * k) && i + j + k == 1000 {
                    return i * j * k;
                }
            }
        }
    }

    unreachable!()
}

// Problem 10
fn problem_10() -> u64 {
    util::primes().take_while(|prime| *prime < 2_000_000).sum()
}

// Problem 11
fn problem_11() -> u64 {
    let s = String::from("
        08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
        49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
        81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
        52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
        22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
        24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
        32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
        67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
        24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
        21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
        78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
        16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
        86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
        19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
        04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
        88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
        04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
        20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
        20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
        01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
    ");

    let grid = s.trim()
        .split(" ")
        .filter(|part| !part.is_empty())
        .map(|part| part.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let (width, height) = (20, 20);
    let mut max = 0u64;
    
    for x in 0..16 {
        for y in 0..20 {
            let value: u64 = (0..4).map(|dx| grid[x + dx + y * width]).product();
            if (value > max) { 
                max = value;
            }
        }
    }
    
    for x in 0..20 {
        for y in 0..16 {
            let value: u64 = (0..4).map(|dy| grid[x + (y + dy) * width]).product();
            if (value > max) {
                max = value;
            }
        }
    }
    
    for x in 3..20 {
        for y in 0..16 {
            let value = (0..4).map(|d| grid[x - d + (y + d) * width]).product();
            if (value > max) {
                max = value;
            }
        }
    }

    for x in 0..16 {
        for y in 0..16 {
            let value = (0..4).map(|d| grid[x + d + (y + d) * width]).product();
            if (value > max) {
                max = value;
            }
        }
    }

    max
}