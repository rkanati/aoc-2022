
use std::collections::VecDeque;

static INPUT: &str = include_str!("input");

fn main() {
    let (config, prog) = INPUT.split_once("\n\n").unwrap();

    let s0 = config.lines()
        .take_while(|line| !line.is_empty())
        .fold(Vec::new(), |mut state, line| {
            let chunks = line.as_bytes().chunks(4);
            while state.len() < chunks.len() {state.push(VecDeque::new())}
            chunks.enumerate()
                .for_each(|(column_i, chunk)| match chunk {
                    &[b'[', c,    b']', ..] => state[column_i].push_back(c as char),
                    //&[b' ', b' ', b' ', ..] => { },
                    _ => { }
                });
            state
        });

    let (s1, s2) = prog.lines()
        .fold((s0.clone(), s0.clone()), |(mut s1, mut s2), line| {
            let words = line.split_ascii_whitespace().collect::<Vec<_>>();
            if let &["move", n, "from", src, "to", dst] = &words[..] {
                let n = n.parse::<usize>().unwrap();
                let src = src.parse::<usize>().unwrap() - 1;
                let dst = dst.parse::<usize>().unwrap() - 1;

                // part 1 (cratemover 9000)
                for _ in 0..n {
                    let cr = s1[src].pop_front().unwrap();
                    s1[dst].push_front(cr);
                }

                // part 2 (cratemover 9001)
                let mut stack = s2[src].drain(..n).collect::<VecDeque<_>>();
                stack.append(&mut s2[dst]);
                s2[dst] = stack;
            }
            (s1, s2)
        });

    let tops = |s: &[VecDeque<char>]| s.iter().map(|stack| *stack.front().unwrap()).collect::<String>();
    let part_1 = tops(&s1);
    println!("part 1: {part_1}");
    let part_2 = tops(&s2);
    println!("part 2: {part_2}");
}

