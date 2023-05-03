use crate::utils::get_data;

pub fn first_part() {
    let data = get_data("./data/day1");

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
    let data = get_data("./data/day1");

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
