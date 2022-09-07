/* На бесконечной координатной сетке находится муравей. Муравей может
перемещатся на 1 клетку вверх (x,y+1), вниз (x,y-1), влево (x-1,y),
вправо (x+1,y), по одной клетке за шаг. Клетки, в которых сумма цифр в
координате X плюс сумма цифр в координате Y больше чем 25 недоступны
муравью. Например, клетка с координатами (59, 79) недоступна, т.к.
5+9+7+9=30, что больше 25. Сколько cклеток может посетить муравей если
его начальная позиция (1000,1000), (включая начальную клетку). */

use std::collections::HashSet;

type Coords = (i32, i32);

const INITIAL_POSITION: Coords = (1000, 1000);
const MAX_ALLOWED_SUM: u32 = 25;

fn solve() -> u32 {
    let mut seen: HashSet<Coords> = HashSet::new();
    let mut moves: HashSet<Coords> = HashSet::from([INITIAL_POSITION]);

    while !moves.is_empty() {
        seen.extend(&moves);
        moves = moves
            .iter()
            .flat_map(possible_moves)
            .filter(|x| !seen.contains(x))
            .collect();
    }

    seen.len().try_into().unwrap()
}

fn possible_moves(&(x, y): &Coords) -> Vec<Coords> {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .filter(coords_valid)
        .collect()
}

fn coords_valid(&(x, y): &Coords) -> bool {
    digit_sum(x) + digit_sum(y) <= MAX_ALLOWED_SUM
}

fn digit_sum(number: i32) -> u32 {
    number
        .unsigned_abs()
        .to_string()
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
        .expect("symbol in string representation is not a digit")
        .iter()
        .sum()
}

fn main() {
    println!("Solution: {}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_sum_works_for_nonzero_number() {
        assert_eq!(digit_sum(123), 6)
    }

    #[test]
    fn digit_sum_works_for_single_digit_number() {
        assert_eq!(digit_sum(5), 5)
    }

    #[test]
    fn digit_sum_works_for_zero() {
        assert_eq!(digit_sum(0), 0)
    }

    #[test]
    fn digit_sum_works_for_i32_min() {
        assert_eq!(digit_sum(i32::MIN), 47)
    }

    #[test]
    fn initial_coords_valid() {
        assert!(coords_valid(&(1000, 1000)))
    }

    #[test]
    fn coords_with_sum_25_valid() {
        assert!(coords_valid(&(555, 55)))
    }

    #[test]
    fn coords_with_sum_26_invalid() {
        assert!(!coords_valid(&(556, 55)))
    }

    #[test]
    fn solve_produces_expected_result() {
        assert_eq!(solve(), 148848)
    }
}
