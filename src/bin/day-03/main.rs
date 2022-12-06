
#![feature(iter_array_chunks)]

static INPUT: &str = include_str!("input");

fn priority(item: char) -> i32 {
    let item = item as u32 as u8;
    let pri = match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!()
    };
    pri as i32
}

fn main() {
    let part_1: i32 = INPUT.lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let common = a.chars().find(|&ch| b.contains(ch)).unwrap();
            priority(common)
        })
        .sum();
    println!("part 1: {part_1}");

    let part_2: i32 = INPUT.lines()
        .array_chunks()
        .map(|[a, b, c]| {
            let common = a.chars().find(|&ch| b.contains(ch) && c.contains(ch)).unwrap();
            priority(common)
        })
        .sum();
    println!("part 2: {part_2}");
}
