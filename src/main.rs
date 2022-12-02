use std::{cmp::max, io};

fn main() {
    println!("day one!");

    let mut maximum = -1;
    let mut totals = vec![0];
    let mut current_count = 0;

    loop {
        let buf = &mut String::new();
        io::stdin().read_line(buf).unwrap();

        if buf.is_empty() {
            break;
        } else if buf == "\n" {
            maximum = max(maximum, current_count);
            totals.push(current_count);
            current_count = 0;
            continue;
        }

        let num = buf.trim().parse::<i32>().unwrap();
        current_count += num;
    }

    println!("Part 1: {maximum}");

    totals.sort();
    totals.reverse();

    println!("Part 2: {}", totals[0] + totals[1] + totals[2]);
}
