use crate::utils::get_data;

fn get_hash(data: &str, zeros: usize) -> i32 {
    (0..)
        .find(|number| {
            let hash = md5::compute(format!("{}{}", data, number));

            return format!("{:x}", hash).starts_with("0".repeat(zeros).as_str());
        })
        .unwrap()
}

pub fn first_part() {
    let data = get_data("./data/day4");
    println!("The number is: {}", get_hash(&data, 5));
}

pub fn second_part() {
    let data = get_data("./data/day4");

    println!("The number is: {}", get_hash(&data, 6));
}
