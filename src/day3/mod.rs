use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn get_data() -> String {
    // create a path to the file
    let path = Path::new("./data/day3");
    let display = path.display();

    // open the path in read-only mode
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut data = String::new();

    match file.read_to_string(&mut data) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    return data;
}

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
    let data = get_data();

    let mut location = (0, 0);

    let mut houses = HashSet::new();

    for c in data.chars() {
        move_santa(c, &mut location).unwrap();

        houses.insert(location.clone());
    }

    println!("Houses visited: {}", houses.len());
}

pub fn second_part() {
    let data = get_data();

    let mut location = (0, 0);
    let mut robo_location = (0, 0);

    let mut houses = HashSet::new();

    for (i, c) in data.chars().enumerate() {
        if i % 2 == 0 {
            move_santa(c, &mut location).unwrap();
            houses.insert(location.clone());
            continue;
        }

        move_santa(c, &mut robo_location).unwrap();
        houses.insert(robo_location.clone());
    }

    println!("Houses visited: {}", houses.len());
}
