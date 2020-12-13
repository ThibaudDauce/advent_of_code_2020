fn main()
{
    let result = part1(raw_input());
    println!("Part 1: {}", result);
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

fn raw_input() -> &'static str
{
    "
    1002578
    19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,751,x,29,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,431,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,17
    "
}