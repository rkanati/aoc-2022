
static INPUT: &str = include_str!("input");

fn main() {
    let (p1, p2) = solve(INPUT);
    println!("part 1: {p1}");
    println!("part 2: {p2}");
}

fn solve(input: &'static str) -> (i32, i32) {
    input.lines()
        .filter_map(|line| {
            let &[l, _, r] = line.as_bytes() else {return None};
            let parse = |c| (c-b'A') as i32 % 23;
            let (l, r) = (parse(l), parse(r));
            let eval = |q| ((q-l+4)%3)*3 + q+1;
            (eval(r), eval((r+l+2)%3)).into()
        })
        .reduce(|(a, b), (c, d)| (a+c, b+d))
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        let strats = crate::INPUT.lines()
            .map(|line| {
                let &[lt, b' ', rt] = line.as_bytes() else {panic!()};
                Strat(lt.into(), rt.into())
            })
            .collect::<Vec<_>>();

        let part_1: i32 = strats.iter().map(|strat| strat.eval_part_1()).sum();
        let part_2: i32 = strats.iter().map(|strat| strat.eval_part_2()).sum();

        let (p1, p2) = crate::solve(crate::INPUT);
        assert_eq!(p1, part_1);
        assert_eq!(p2, part_2);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Play {
        A, B, C,
        X, Y, Z,
    }

    impl From<u8> for Play {
        fn from(ch: u8) -> Self {
            match ch {
                b'A' => Self::A,
                b'B' => Self::B,
                b'C' => Self::C,
                b'X' => Self::X,
                b'Y' => Self::Y,
                b'Z' => Self::Z,
                _ => panic!(),
            }
        }
    }


    #[derive(Debug, Clone, Copy)]
    struct Strat(Play, Play);

    impl Strat {
        fn eval_part_1(self) -> i32 {
            let Strat(lt, rt) = self;

            use Play::*;
            let rt = match rt {
                X => A, Y => B, Z => C,
                p => p,
            };

            let outcome_score = match (lt, rt) {
                (A, B) | (B, C) | (C, A) => 6, // win
                (A, A) | (B, B) | (C, C) => 3, // draw
                (A, C) | (B, A) | (C, B) => 0, // lose
                _ => unreachable!()
            };

            let play_score = match rt {
                A => 1, B => 2, C => 3,
                _ => unreachable!()
            };

            outcome_score + play_score
        }

        fn eval_part_2(self) -> i32 {
            let Strat(lt, rt) = self;

            use Play::*;
            let play = match rt {
                X => match lt { A => C, B => A, C => B, _ => unreachable!() },
                Y => lt,
                Z => match lt { A => B, B => C, C => A, _ => unreachable!() },
                _ => unreachable!()
            };

            Strat(lt, play).eval_part_1()
        }
    }
}

