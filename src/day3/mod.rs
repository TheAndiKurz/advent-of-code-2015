use crate::utils::get_data;
use std::collections::HashSet;

fn move_santa(c: char, location: &mut (i32, i32)) -> Result<(), &'static str> {
    match c {
        '>' => location.0 += 1,
        '<' => location.0 -= 1,
        '^' => location.1 += 1,
        'v' => location.1 -= 1,
        _ => Err("Invalid character")?,
    };

    Ok(())
}

pub fn first_part() {
    let data = get_data("./data/day3");

    let mut location = (0, 0);

    let mut houses = HashSet::new();

    for c in data.chars() {
        move_santa(c, &mut location).unwrap();

        houses.insert(location);
    }

    println!("Houses visited: {}", houses.len());
}

pub fn second_part() {
    let data = get_data("./data/day3");

    let mut location = (0, 0);
    let mut robo_location = (0, 0);

    let mut houses = HashSet::new();

    for (i, c) in data.chars().enumerate() {
        if i % 2 == 0 {
            move_santa(c, &mut location).unwrap();
            houses.insert(location);
            continue;
        }

        move_santa(c, &mut robo_location).unwrap();
        houses.insert(robo_location);
    }

    println!("Houses visited: {}", houses.len());
}
