struct NumberIter {
    number: i32,
    prev: i32,
    next: i32,
}

impl NumberIter {
    fn new(number: i32) -> Self {
        Self {
            number,
            prev: -1,
            next: -1,
        }
    }
}

// Iterates from last to first number
impl Iterator for NumberIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.number >= 1 {
            let next = self.number % 10;
            self.prev = self.next;
            self.next = next;
            self.number /= 10;

            return Some((self.next, self.prev));
        }
        None
    }
}

pub fn part_one(min: i32, max: i32) -> i32 {
    let min_count = ((min as f32 + 1.0).log10()).ceil();
    let max_count = ((max as f32 + 1.0).log10()).ceil();

    if min_count != 6.0 || max_count != 6.0 {
        return -1;
    }

    (min..max)
        .filter(|i| valididate_password_part_one(*i))
        .count() as i32
}

fn valididate_password_part_one(n: i32) -> bool {
    let mut found_adjacent = false;

    for (next, prev) in NumberIter::new(n) {
        if prev != -1 {
            if next == prev {
                found_adjacent = true;
            }

            if prev < next {
                return false;
            }
        }
    }

    return found_adjacent;
}

pub fn part_two(min: i32, max: i32) -> i32 {
    let mut adjecents = [0; 10];

    (min..max)
        .filter(|i| valididate_password_part_two(*i, &mut adjecents))
        .count() as i32
}

fn valididate_password_part_two(n: i32, adjecents: &mut [i32]) -> bool {
    let mut decrease_only = true;

    for (next, prev) in NumberIter::new(n) {
        if prev != -1 {
            if next == prev {
                adjecents[next as usize] += 1;
            }

            if prev < next {
                decrease_only = false;
                break;
            }
        }
    }

    if decrease_only == false {
        // Reset adjecents
        for elem in adjecents.iter_mut() {
            *elem = 0;
        }

        return false;
    }

    let valid = adjecents
        .iter_mut()
        .filter(|i| **i % 2 == 1)
        .any(|n| *n == 1);

    // Reset adjecents
    for elem in adjecents.iter_mut() {
        *elem = 0;
    }

    return valid;
}

#[test]
fn test_iterator() {
    let mut iter = NumberIter::new(456);
    let (next, prev) = iter.next().unwrap();
    assert!(next == 6 && prev == -1);
    let (next, prev) = iter.next().unwrap();
    assert!(next == 5 && prev == 6);
    let (next, prev) = iter.next().unwrap();
    assert!(next == 4 && prev == 5);
    assert!(iter.next() == None);
}

#[test]
fn test_part_one() {
    assert!(valididate_password_part_one(111111) == true);
    assert!(valididate_password_part_one(223450) == false);
    assert!(valididate_password_part_one(123789) == false);
    assert!(valididate_password_part_one(123445) == true);
    assert!(valididate_password_part_one(111110) == false);
}

#[test]
fn test_part_two() {
    let mut adjecents = [0; 10];
    assert!(valididate_password_part_two(111110, &mut adjecents) == false);
    assert!(valididate_password_part_two(223450, &mut adjecents) == false);
    assert!(valididate_password_part_two(123789, &mut adjecents) == false);
    assert!(valididate_password_part_two(123445, &mut adjecents) == true);
    assert!(valididate_password_part_two(111110, &mut adjecents) == false);
    assert!(valididate_password_part_two(112233, &mut adjecents) == true);
    assert!(valididate_password_part_two(123444, &mut adjecents) == false);
    assert!(valididate_password_part_two(111122, &mut adjecents) == true);
    assert!(valididate_password_part_two(111122, &mut adjecents) == true);
}
