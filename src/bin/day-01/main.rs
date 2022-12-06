
static INPUT: &str = include_str!("input");

fn top_elves(input: &str) -> Vec<u32> {
    let mut top_elves = Vec::new();
    let mut add_elf = |elf| {
        let mut at = 0;
        while top_elves.get(at).map_or(false, |&other| other > elf) { at += 1; }
        top_elves.insert(at, elf);
        top_elves.truncate(3);
    };

    let mut elf = 0;
    for line in input.lines() {
        if let Some(item) = line.trim().parse::<u32>().ok() {
            elf += item;
        }
        else {
            add_elf(elf);
            elf = 0;
        }
    }
    add_elf(elf);
    top_elves
}

fn main() {
    let top_elves = top_elves(INPUT);

    let part_1 = top_elves[0];
    println!("part_1: {part_1}");

    let part_2: u32 = top_elves.iter().sum();
    println!("part_2: {part_2}");
}

