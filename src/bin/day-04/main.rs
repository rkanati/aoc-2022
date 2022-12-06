
use std::ops::RangeInclusive;

static INPUT: &str = include_str!("input");

trait Intersect<With = Self> {
    type Intersection;
    fn intersect(&self, with: &With) -> Option<Self::Intersection>;
}

impl<T> Intersect for RangeInclusive<T> where T: Ord + Clone {
    type Intersection = Self;

    fn intersect(&self, with: &Self) -> Option<Self> {
        let lo = self.start().max(with.start()).clone();
        let hi = self.end().min(with.end()).clone();
        if lo <= hi {Some(lo ..= hi)}
        else        {None}
    }
}

#[test]
fn test_intersect() {
    assert_eq!((1..=10).intersect(&(1..=10)), Some(1..=10));
    assert_eq!((1..=10).intersect(&(10..=20)), Some(10..=10));
    assert_eq!((1..=10).intersect(&(20..=30)), None);
    assert_eq!((1..=20).intersect(&(5..=15)), Some(5..=15));
}

fn main() {
    let (part_1, part_2) = INPUT.lines()
        .map(|line| {
            let (a, b) = line.trim().split_once(',').unwrap();
            let parse = |s: &str| -> std::ops::RangeInclusive<i32> {
                let (min, max) = s.trim().split_once('-').unwrap();
                min.parse().unwrap() ..= max.parse().unwrap()
            };
            (parse(a), parse(b))
        })
        .map(|(a, b)| {
            let x = a.intersect(&b);
            let overlapped = x.is_some();
            let contained = x.map_or(false, |x| x == a || x == b);
            (contained as i32, overlapped as i32)
        })
        .reduce(|(a, b), (c, d)| (a+c, b+d))
        .unwrap();
    println!("part 1: {part_1}\n\
              part 2: {part_2}");
}

