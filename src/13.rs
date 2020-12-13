fn main()
{
    let result = part1(raw_input());
    println!("Part 1: {}", result);

    let result = part2(raw_input());
    println!("Part 2: {}", result);
}

fn part1(raw_input: &'static str) -> i64
{
    let mut lines = raw_input.trim().lines();
    let arrival: i64 = lines.next().unwrap().trim().parse().unwrap();

    let buses: Vec<i64> = lines.next().unwrap().trim().split(',').filter_map(|value| {
        match value {
            "x" => None,
            _ => Some(value.parse().expect(&format!("Cannot parse {}", value))),
        }
    }).collect();

    let mut waiting_time_min = i64::MAX;
    let mut bus_min = 0;
    for bus in buses {
        let mut waiting_time = bus - (arrival % bus);
        if waiting_time == bus {
            waiting_time = 0;
        }


        if waiting_time < waiting_time_min {
            waiting_time_min = waiting_time;
            bus_min = bus;
        }
    }

    waiting_time_min * bus_min
}

#[test]
fn test_part1()
{
    assert_eq!(295, part1("
    939
    7,13,x,x,59,x,31,19
    "));
}

fn part2(raw_input: &'static str) -> u64
{
    let mut lines = raw_input.trim().lines();
    lines.next().unwrap();

    let mut buses: Vec<(u64, u64)> = vec![];
    for (index, value) in lines.next().unwrap().trim().split(',').enumerate() {
        if value == "x" {
            continue;
        }

        let bus = value.parse().unwrap();
        dbg!(index, index as u64, bus - (index as u64));
        let mut diff = (index as u64);  

        buses.push((bus, diff));
    }

    dbg!(&buses);

    let mut i: u64 = 0;
    let mut step = 1;

    'main_loop: loop {
        let mut new_step = 1;

        if i % 100_000_000 == 0 {
            println!("i is {}", i);
        }

        for (bus, diff) in &buses {
            if (i + diff) % bus != 0 {
                step = new_step;
                i += step;
                continue 'main_loop;
            } else {
                new_step *= bus;
            }
        }

        return i;
    }
}

#[test]
fn test_part2()
{
    assert_eq!(3417, part2("
    42
    17,x,13,19
    "));
    assert_eq!(4, part2(" 
    42
    2,x,x,x,x,3
    "));
    assert_eq!(754018, part2("
    42
    67,7,59,61
    "));
    assert_eq!(779210, part2("
    42
    67,x,7,59,61
    "));
    assert_eq!(1261476, part2("
    42
    67,7,x,59,61
    "));
    assert_eq!(1202161486, part2("
    42
    1789,37,47,1889
    "));
    assert_eq!(1068781, part2("
    42
    7,13,x,x,59,x,31,19
    "));
}

fn raw_input() -> &'static str
{
    "
    1002578
    19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,751,x,29,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,431,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,17
    "
}