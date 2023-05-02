use std::fs::File;
use std::io::Read;
use std::path::Path;

fn get_data() -> String {
    // create a path to the file
    let path = Path::new("./data/day1");
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

pub fn first_part() {
    let data = get_data();

    let mut floor: i64 = 0;

    for char in data.chars() {
        if char == '(' {
            floor += 1;
        } else if char == ')' {
            floor -= 1;
        } else {
            panic!(
                "coundn't match char '{}' something is wrong in the data!",
                char
            );
        }
    }

    println!("Santa will go to floor: {}", floor);
}

pub fn second_part() {
    let data = get_data();

    let mut floor: i64 = 0;

    for (idx, char) in data.chars().enumerate() {
        if char == '(' {
            floor += 1;
        } else if char == ')' {
            floor -= 1;
        } else {
            panic!(
                "coundn't match char '{}' something is wrong in the data!",
                char
            );
        }

        if floor == -1 {
            println!("Santa is going to the basement at position: {}", idx + 1);
            break;
        }
    }
}
