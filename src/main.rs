use num::{BigInt, ToPrimitive};
use std::collections::HashMap;

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
    // println!("Problem 11: {}", problem_11());
    // println!("Problem 12: {}", problem_12());
    // println!("Problem 13: {}", problem_13());
    // println!("Problem 14: {}", problem_14());
    // println!("Problem 15: {}", problem_15());
    // println!("Problem 16: {}", problem_16());
    println!("Problem 17: {}", problem_17());
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
    let width = 20;
    let mut max = 0u64;

    for x in 0..16 {
        for y in 0..20 {
            let value: u64 = (0..4).map(|dx| grid[x + dx + y * width]).product();
            if value > max {
                max = value;
            }
        }
    }

    for x in 0..20 {
        for y in 0..16 {
            let value: u64 = (0..4).map(|dy| grid[x + (y + dy) * width]).product();
            if value > max {
                max = value;
            }
        }
    }

    for x in 3..20 {
        for y in 0..16 {
            let value = (0..4).map(|d| grid[x - d + (y + d) * width]).product();
            if value > max {
                max = value;
            }
        }
    }

    for x in 0..16 {
        for y in 0..16 {
            let value = (0..4).map(|d| grid[x + d + (y + d) * width]).product();
            if value > max {
                max = value;
            }
        }
    }

    max
}

// Problem 12
fn problem_12() -> u64 {
    fn count_factors(i: u64) -> u64 {
        let mut factors: HashMap<u64, u64> = HashMap::new();

        for prime in util::prime_factors(i) {
            if factors.contains_key(&prime) {
                factors.insert(prime, factors[&prime] + 1);
            } else {
                factors.insert(prime, 1);
            }
        }

        factors.values().map(|i| i + 1).product()
    }

    util::triangle_numbers().find(|i| count_factors(*i) > 500).unwrap()
}

// Problem 13
fn problem_13() -> u64 {
    let s = String::from("
        37107287533902102798797998220837590246510135740250
        46376937677490009712648124896970078050417018260538
        74324986199524741059474233309513058123726617309629
        91942213363574161572522430563301811072406154908250
        23067588207539346171171980310421047513778063246676
        89261670696623633820136378418383684178734361726757
        28112879812849979408065481931592621691275889832738
        44274228917432520321923589422876796487670272189318
        47451445736001306439091167216856844588711603153276
        70386486105843025439939619828917593665686757934951
        62176457141856560629502157223196586755079324193331
        64906352462741904929101432445813822663347944758178
        92575867718337217661963751590579239728245598838407
        58203565325359399008402633568948830189458628227828
        80181199384826282014278194139940567587151170094390
        35398664372827112653829987240784473053190104293586
        86515506006295864861532075273371959191420517255829
        71693888707715466499115593487603532921714970056938
        54370070576826684624621495650076471787294438377604
        53282654108756828443191190634694037855217779295145
        36123272525000296071075082563815656710885258350721
        45876576172410976447339110607218265236877223636045
        17423706905851860660448207621209813287860733969412
        81142660418086830619328460811191061556940512689692
        51934325451728388641918047049293215058642563049483
        62467221648435076201727918039944693004732956340691
        15732444386908125794514089057706229429197107928209
        55037687525678773091862540744969844508330393682126
        18336384825330154686196124348767681297534375946515
        80386287592878490201521685554828717201219257766954
        78182833757993103614740356856449095527097864797581
        16726320100436897842553539920931837441497806860984
        48403098129077791799088218795327364475675590848030
        87086987551392711854517078544161852424320693150332
        59959406895756536782107074926966537676326235447210
        69793950679652694742597709739166693763042633987085
        41052684708299085211399427365734116182760315001271
        65378607361501080857009149939512557028198746004375
        35829035317434717326932123578154982629742552737307
        94953759765105305946966067683156574377167401875275
        88902802571733229619176668713819931811048770190271
        25267680276078003013678680992525463401061632866526
        36270218540497705585629946580636237993140746255962
        24074486908231174977792365466257246923322810917141
        91430288197103288597806669760892938638285025333403
        34413065578016127815921815005561868836468420090470
        23053081172816430487623791969842487255036638784583
        11487696932154902810424020138335124462181441773470
        63783299490636259666498587618221225225512486764533
        67720186971698544312419572409913959008952310058822
        95548255300263520781532296796249481641953868218774
        76085327132285723110424803456124867697064507995236
        37774242535411291684276865538926205024910326572967
        23701913275725675285653248258265463092207058596522
        29798860272258331913126375147341994889534765745501
        18495701454879288984856827726077713721403798879715
        38298203783031473527721580348144513491373226651381
        34829543829199918180278916522431027392251122869539
        40957953066405232632538044100059654939159879593635
        29746152185502371307642255121183693803580388584903
        41698116222072977186158236678424689157993532961922
        62467957194401269043877107275048102390895523597457
        23189706772547915061505504953922979530901129967519
        86188088225875314529584099251203829009407770775672
        11306739708304724483816533873502340845647058077308
        82959174767140363198008187129011875491310547126581
        97623331044818386269515456334926366572897563400500
        42846280183517070527831839425882145521227251250327
        55121603546981200581762165212827652751691296897789
        32238195734329339946437501907836945765883352399886
        75506164965184775180738168837861091527357929701337
        62177842752192623401942399639168044983993173312731
        32924185707147349566916674687634660915035914677504
        99518671430235219628894890102423325116913619626622
        73267460800591547471830798392868535206946944540724
        76841822524674417161514036427982273348055556214818
        97142617910342598647204516893989422179826088076852
        87783646182799346313767754307809363333018982642090
        10848802521674670883215120185883543223812876952786
        71329612474782464538636993009049310363619763878039
        62184073572399794223406235393808339651327408011116
        66627891981488087797941876876144230030984490851411
        60661826293682836764744779239180335110989069790714
        85786944089552990653640447425576083659976645795096
        66024396409905389607120198219976047599490197230297
        64913982680032973156037120041377903785566085089252
        16730939319872750275468906903707539413042652315011
        94809377245048795150954100921645863754710598436791
        78639167021187492431995700641917969777599028300699
        15368713711936614952811305876380278410754449733078
        40789923115535562561142322423255033685442488917353
        44889911501440648020369068063960672322193204149535
        41503128880339536053299340368006977710650566631954
        81234880673210146739058568557934581403627822703280
        82616570773948327592232845941706525094512325230608
        22918802058777319719839450180888072429661980811197
        77158542502016545090413245809786882778948721859617
        72107838435069186155435662884062257473692284509516
        20849603980134001723930671666823555245252804609722
        53503534226472524250874054075591789781264330331690
    ");

    let sum: BigInt = s.lines()
        .skip(1)
        .take(100)
        .map(|line| line.trim().parse::<BigInt>().unwrap())
        .sum();
    sum.to_string()[0..10].parse::<u64>().unwrap()
}

// Problem 14
fn problem_14() -> u64 {
    let mut longest_chain = 1;
    let mut number_which_produces_longest_chain = 1;

    let mut cache: HashMap<u64, u64> = HashMap::new();
    cache.insert(1, 1);

    fn collatz(i: u64) -> u64 {
        if i % 2 == 0 {
            i / 2
        } else {
            3 * i + 1
        }
    }

    fn compute_length_of_chain(i: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if cache.contains_key(&i) {
            cache[&i]
        } else {
            let next = collatz(i);
            let length_of_chain = compute_length_of_chain(next, cache) + 1;

            cache.insert(i, length_of_chain);
            length_of_chain
        }
    }

    for i in 2..1_000_000 {
        let length_of_chain = compute_length_of_chain(i, &mut cache);

        if length_of_chain > longest_chain {
            longest_chain = length_of_chain;
            number_which_produces_longest_chain = i;
        }
    }

    number_which_produces_longest_chain
}

// Problem 15
fn problem_15() -> u64 {
    ((21..=40).product::<BigInt>() / (2..=20).product::<BigInt>()).to_u64().unwrap()
}

// Problem 16
fn problem_16() -> u64 {
    let mut i = BigInt::from(2).pow(1000);
    let mut sum = BigInt::from(0);

    while i > 0.into() {
        sum += &i % 10;
        i /= 10;
    }

    sum.to_u64().unwrap()
}

// Problem 17
fn problem_17() -> u64 {
    let list = HashMap::from([
        (1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five"), (6, "six"), (7, "seven"),
        (8, "eight"), (9, "nine"), (10, "ten"), (11, "eleven"), (12, "twelve"), (13, "thirteen"),
        (14, "fourteen"), (15, "fifteen"), (16, "sixteen"), (17, "seventeen"), (18, "eighteen"),
        (19, "nineteen"), (20, "twenty"), (30, "thirty"), (40, "forty"), (50, "fifty"),
        (60, "sixty"), (70, "seventy"), (80, "eighty"), (90, "ninety")
    ]);

    let print_tens_and_digits = |i: u64| -> Option<String> {
        let i = i % 100;

        if i == 0 {
            None
        } else if i <= 20 || i % 10 == 0 {
            Some(format!("{}", list[&i]))
        } else {
            let a = i / 10 * 10;
            let b = i % 10;
            Some(format!("{}-{}", list[&a], list[&b]))
        }
    };

    let print_hundreds = |i: u64| -> Option<String> {
        let i = i % 1_000;
        let a = i / 100;
        let tens_and_digits = print_tens_and_digits(i);

        if i == 0 {
            None
        } else if a == 0 {
            tens_and_digits
        } else if tens_and_digits.is_none() {
            Some(format!("{} hundred", list[&a]))
        } else {
            Some(format!("{} hundred and {}", list[&a], tens_and_digits.unwrap()))
        }
    };

    let print_thousands = |i: u64| -> Option<String> {
        let i = i % 10_000;
        let a = i / 1000;
        let hundreds = print_hundreds(i);

        if i == 0 {
            None
        } else if a == 0 {
            hundreds
        } else if hundreds.is_none() {
            Some(format!("{} thousand", list[&a]))
        } else {
            Some(format!("{} thousand {}", list[&a], hundreds.unwrap()))
        }
    };

    let mut sum = 0;

    for i in 1..=1000 {
        let s = print_thousands(i).unwrap();
        sum += s.chars().filter(|c| !c.is_whitespace() && *c != '-').count();
    }

    sum.to_u64().unwrap()
}