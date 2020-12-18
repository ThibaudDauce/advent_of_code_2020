use std::collections::HashSet;

fn main()
{
    let result = part1(raw_input());
    println!("{}", result);

    let result = part2(raw_input());
    println!("{}", result);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coordinates 
{
    x: i64,
    y: i64,
    z: i64,
}

// Part 1

fn part1(raw_input: &'static str) -> usize
{
    let mut active = HashSet::new();

    let mut min = Coordinates { x: i64::MAX, y: i64::MAX, z: i64::MAX };
    let mut max = Coordinates { x: i64::MIN, y: i64::MIN, z: i64::MIN };

    for (x, line) in raw_input.trim().lines().enumerate() {
        for (y, value) in line.trim().chars().enumerate() {
            if let '#' = value {
                let coordinates = Coordinates {x: x as i64, y: y as i64, z: 0};
                compute_min_max(&mut min, &mut max, &coordinates);
                active.insert(coordinates);
            }
        }
    }

    let mut cycle = 1;

    loop {
        let mut new_active = active.clone();

        for x in min.x-1..=max.x+1 {
            for y in min.y-1..=max.y+1 {
                for z in min.z-1..=max.z+1 {
                    let mut neighbors_active = 0;
                    for diff_x in -1..=1 {
                        for diff_y in -1..=1 {
                            for diff_z in -1..=1 {
                                if diff_x == 0 && diff_y == 0 && diff_z == 0 {
                                    continue;
                                }

                                if active.contains(&Coordinates { x: x + diff_x, y: y + diff_y, z: z + diff_z }) {
                                    neighbors_active += 1;
                                }
                            }
                        }
                    }

                    let me = Coordinates { x, y, z };
                    if active.contains(&me) {
                        if neighbors_active == 2 || neighbors_active == 3 {
                            compute_min_max(&mut min, &mut max, &me);
                        } else {
                            new_active.remove(&me);
                        }
                    } else {
                        if neighbors_active == 3 {
                            compute_min_max(&mut min, &mut max, &me);
                            new_active.insert(me);
                        }
                    }
                }
            }
        }

        if cycle == 6 {
            return new_active.len();
        }

        active = new_active;
        cycle += 1;
    }
}

fn compute_min_max(mut min: &mut Coordinates, mut max: &mut Coordinates, coordinates: &Coordinates)
{
    if coordinates.x < min.x {
        min.x = coordinates.x;
    }
    if coordinates.x > max.x {
        max.x = coordinates.x;
    }
    if coordinates.y < min.y {
        min.y = coordinates.y;
    }
    if coordinates.y > max.y {
        max.y = coordinates.y;
    }
    if coordinates.z < min.z {
        min.z = coordinates.z;
    }
    if coordinates.z > max.z {
        max.z = coordinates.z;
    }
}

#[test]
fn test_part1()
{
    assert_eq!(112, part1("
    .#.
    ..#
    ###
    "));
}

// Part 2

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct CoordinatesPart2
{
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

fn part2(raw_input: &'static str) -> usize
{
    let mut active = HashSet::new();

    let mut min = CoordinatesPart2 { x: i64::MAX, y: i64::MAX, z: i64::MAX, w: i64::MAX };
    let mut max = CoordinatesPart2 { x: i64::MIN, y: i64::MIN, z: i64::MIN, w: i64::MIN };

    for (x, line) in raw_input.trim().lines().enumerate() {
        for (y, value) in line.trim().chars().enumerate() {
            if let '#' = value {
                let coordinates = CoordinatesPart2 {x: x as i64, y: y as i64, z: 0, w: 0 };
                compute_min_max_part2(&mut min, &mut max, &coordinates);
                active.insert(coordinates);
            }
        }
    }

    let mut cycle = 1;

    loop {
        let mut new_active = active.clone();

        for x in min.x-1..=max.x+1 {
            for y in min.y-1..=max.y+1 {
                for z in min.z-1..=max.z+1 {
                    for w in min.w-1..=max.w+1 {
                        let mut neighbors_active = 0;
                        for diff_x in -1..=1 {
                            for diff_y in -1..=1 {
                                for diff_z in -1..=1 {
                                    for diff_w in -1..=1 {
                                        if diff_x == 0 && diff_y == 0 && diff_z == 0 && diff_w == 0 {
                                            continue;
                                        }

                                        if active.contains(&CoordinatesPart2 { x: x + diff_x, y: y + diff_y, z: z + diff_z, w: w + diff_w }) {
                                            neighbors_active += 1;
                                        }
                                    }
                                }
                            }
                        }

                        let me = CoordinatesPart2 { x, y, z, w };
                        if active.contains(&me) {
                            if neighbors_active == 2 || neighbors_active == 3 {
                                compute_min_max_part2(&mut min, &mut max, &me);
                            } else {
                                new_active.remove(&me);
                            }
                        } else {
                            if neighbors_active == 3 {
                                compute_min_max_part2(&mut min, &mut max, &me);
                                new_active.insert(me);
                            }
                        }
                    }
                }
            }
        }

        if cycle == 6 {
            return new_active.len();
        }

        active = new_active;
        cycle += 1;
    }
}

fn compute_min_max_part2(mut min: &mut CoordinatesPart2, mut max: &mut CoordinatesPart2, coordinates: &CoordinatesPart2)
{
    if coordinates.x < min.x {
        min.x = coordinates.x;
    }
    if coordinates.x > max.x {
        max.x = coordinates.x;
    }
    if coordinates.y < min.y {
        min.y = coordinates.y;
    }
    if coordinates.y > max.y {
        max.y = coordinates.y;
    }
    if coordinates.z < min.z {
        min.z = coordinates.z;
    }
    if coordinates.z > max.z {
        max.z = coordinates.z;
    }
    if coordinates.w < min.w {
        min.w = coordinates.w;
    }
    if coordinates.w > max.w {
        max.w = coordinates.w;
    }
}

#[test]
fn test_part2()
{
    assert_eq!(848, part2("
    .#.
    ..#
    ###
    "));
}

fn raw_input() -> &'static str
{
    "
    ######.#
    #.###.#.
    ###.....
    #.####..
    ##.#.###
    .######.
    ###.####
    ######.#
    "
}