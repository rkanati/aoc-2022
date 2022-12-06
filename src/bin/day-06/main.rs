
static INPUT: &[u8] = include_bytes!("input");

fn main() {
    let find = |n| INPUT.windows(n)
        .enumerate()
        .find_map(|(i, w)| {
            if (0..w.len()).any(|d| w[d+1..].contains(&w[d])) {None}
            else {Some(i+w.len())}
        })
        .unwrap();
    println!("part 1: {}", find(4));
    println!("part 2: {}", find(14));
}
