use std::collections::HashMap;

fn main()
{
    let result = part1(raw_input());
    println!("Result {}", result);

    let result = part2(raw_input());
    println!("Result {}", result);
}

fn part1(raw_input: &str) -> usize
{
    let mut input = input(raw_input);
    input.sort();

    let mut one_count = 0;
    let mut three_count = 1;

    if input[0] == 1 {
        one_count += 1;
    } else if input[0] == 3 {
        three_count += 1;
    } else {
        panic!("Unknown difference");
    }

    for i in 0..input.len() - 1 {
        if input[i + 1] - input[i] == 1 {
            one_count += 1;
        } else if input[i + 1] - input[i] == 3 {
            three_count += 1;
        } else {
            panic!("Unknown difference {}", i);
        }
    }

    one_count * three_count
}

fn part2(raw_input: &str) -> usize
{
    let mut input = input(raw_input);
    input.push(0);
    input.sort();

    let cache = HashMap::new();

    let (number_of_possibilities, _cache) = compute_part2(&input, 0, cache);
    number_of_possibilities
}

#[test]
fn test_part2()
{
    assert_eq!(43406276662336, part2(raw_input()));
    assert_eq!(4, part2("
    1
    2
    3
    "));
    assert_eq!(8, part2("
        16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4
    "));
    assert_eq!(19208, part2("
    28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3
    "));
}

fn compute_part2(input: &Vec<usize>, index: usize, mut cache: HashMap<usize, usize>) -> (usize, HashMap<usize, usize>)
{
    let value = input[index];
    if let Some(number_of_possibilities) = cache.get(&value) {
        return (*number_of_possibilities, cache);
    }

    if index == input.len() - 1 {
        return (1, cache);
    }

    let mut number_of_possibilities = 0;
    let mut j = 1;
    loop {
        if index + j >= input.len() {
            break;
        }

        let next = input[index + j];

        if next > value + 3 {
            break;
        }

        let (next_number_of_possibilities, new_cache) = compute_part2(input, index + j, cache);
        number_of_possibilities += next_number_of_possibilities;
        cache = new_cache;

        j += 1;
    }

    cache.insert(value, number_of_possibilities);
    (number_of_possibilities, cache)
}

#[test]
fn test_part1()
{
    assert_eq!(7 * 5, part1("
        16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4
    "));
    assert_eq!(22 * 10, part1("
    28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3
    "));
}

fn input(raw_input: &str) -> Vec<usize>
{
    raw_input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn raw_input() -> &'static str
{
    "
    71
    30
    134
    33
    51
    115
    122
    38
    61
    103
    21
    12
    44
    129
    29
    89
    54
    83
    96
    91
    133
    102
    99
    52
    144
    82
    22
    68
    7
    15
    93
    125
    14
    92
    1
    146
    67
    132
    114
    59
    72
    107
    34
    119
    136
    60
    20
    53
    8
    46
    55
    26
    126
    77
    65
    78
    13
    108
    142
    27
    75
    110
    90
    35
    143
    86
    116
    79
    48
    113
    101
    2
    123
    58
    19
    76
    16
    66
    135
    64
    28
    9
    6
    100
    124
    47
    109
    23
    139
    145
    5
    45
    106
    41
    "
}