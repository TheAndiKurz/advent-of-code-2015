use crate::utils::get_data;
use std::cmp::min;

pub fn first_part() {
    let data = get_data("./data/day2");

    let lines = data.split('\n');

    let mut total_sq: i64 = 0;

    for line in lines {
        let dims = line.split('x').collect::<Vec<&str>>();

        if dims.len() != 3 {
            continue;
        }

        let l = dims[0].parse::<i64>().unwrap();
        let w = dims[1].parse::<i64>().unwrap();
        let h = dims[2].parse::<i64>().unwrap();

        let smallest_side: i64 = min(min(l * w, w * h), l * h);

        let side_area1: i64 = 2 * l * w;
        let side_area2: i64 = 2 * w * h;
        let side_area3: i64 = 2 * h * l;

        total_sq += side_area1 + side_area2 + side_area3 + smallest_side;
    }

    println!(
        "The elves need to order {} square feet of wrapping paper!",
        total_sq
    );
}

pub fn second_part() {
    let data = get_data("./data/day2");

    let lines = data.split('\n');

    let mut total_ribbon: i64 = 0;

    for line in lines {
        let dims = line.split('x').collect::<Vec<&str>>();

        if dims.len() != 3 {
            continue;
        }

        let l = dims[0].parse::<i64>().unwrap();
        let w = dims[1].parse::<i64>().unwrap();
        let h = dims[2].parse::<i64>().unwrap();

        total_ribbon += l * w * h;

        total_ribbon += min(2 * (l + w), min(2 * (l + h), 2 * (w + h)));
    }

    println!("The elves need to order {} feet of ribbon!", total_ribbon);
}
