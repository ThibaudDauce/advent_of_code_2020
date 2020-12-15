use std::collections::HashMap;

fn main()
{
    let result = part1(raw_input());
    println!("{}", result);
}

fn part1(raw_input: &'static str) -> u64
{
    let mut records = HashMap::new();

    let mut turn = 1;
    let mut last_digit = 0;
    for digit_as_string in raw_input.split(',') {
        let digit: u64 = digit_as_string.parse().unwrap();

        records.insert(digit, turn);

        last_digit = digit;
        turn += 1;
    }

    records.remove(&last_digit);

    loop {
        let last_digit_turn = records.get(&last_digit);

        let new_digit = if let Some(last_digit_turn) = last_digit_turn {
            turn - 1 - last_digit_turn
        } else {
            0
        };
        
        records.insert(last_digit, turn - 1);
        last_digit = new_digit;


        if turn == 2020 {
            return new_digit;
        }

        turn += 1;
    }
}

#[test]
fn test_part1()
{
    assert_eq!(436, part1("0,3,6"));
    assert_eq!(1, part1("1,3,2"));
    assert_eq!(10, part1("2,1,3"));
    assert_eq!(27, part1("1,2,3"));
    assert_eq!(78, part1("2,3,1"));
    assert_eq!(438, part1("3,2,1"));
    assert_eq!(1836, part1("3,1,2"));
}

fn raw_input() -> &'static str
{
    "16,11,15,0,1,7"
}