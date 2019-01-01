//TODO:
//1) add some tests with randomly shuffled input, also with longer input lists
//2) try to rewrite the ugly break logic inside the loop{}

use std::collections::HashMap;
use std::collections::VecDeque;
type Segment = (i64, i64);

pub fn solve(segments: &Vec<Segment>) -> Option<Vec<Segment>> {
    //hash maps indexed by start / end of the segment

    if segments.is_empty() {
        Some(segments.clone())
    } else {
        //our starting point - the first segment from the list
        let (mut cur_left, mut cur_right) = segments[0];
        let mut result: VecDeque<Segment> = vec![segments[0]].into();
        //iterator over everything but the fist element
        let seg_tail_iter = segments.iter().skip(1);
        let mut left_m: HashMap<i64, Segment> =
            seg_tail_iter.clone().map(|&(l, r)| (l, (l, r))).collect();
        let mut right_m: HashMap<i64, Segment> = seg_tail_iter.map(|&(l, r)| (r, (l, r))).collect();
        loop {
            let right_op = right_m.remove(&cur_left).map(|left_add| {
                result.push_front(left_add);
            });
            result.front().map(|&l| cur_left = l.0);
            if result.len() == segments.len() {
                break;
            };
            let left_op = left_m.remove(&cur_right).map(|right_add| {
                result.push_back(right_add);
            });
            result.back().map(|&r| cur_right = r.1);
            if result.len() == segments.len() {
                break;
            };
            if right_op == None && left_op == None {
                break;
            };
        }
        if result.len() == segments.len() {
            Some(result.into())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    //in case when segments do form a closed loop, rotate the
    //reference sequence to match the result
    use super::Segment;
    fn rotate_to(input: &Vec<Segment>, start: &Segment) -> Vec<Segment> {
        if input.is_empty() || input[0] == *start {
            input.clone()
        } else {
            input
                .iter()
                .cycle()
                .skip_while(|x| *x != start)
                .take(input.len())
                .cloned()
                .collect()
        }
    }
    #[test]
    fn simple_path() {
        let input = vec![(4, 5), (9, 4), (5, 1), (11, 9)];
        let expected = vec![(11, 9), (9, 4), (4, 5), (5, 1)];
        assert_eq!(super::solve(&input), Some(expected));
    }
    #[test]
    fn trivial_loop() {
        let input = vec![(1, 2), (2, 1)];
        let expected = vec![(2, 1), (1, 2)];
        let solution = super::solve(&input);
        let check_res = solution.map(|result| {
            assert!(
                !result.is_empty(),
                "Solution should not be empty in this case!"
            );
            assert_eq!(result, rotate_to(&expected, &result[0]));
        });
        assert!(
            check_res != None,
            "There should be a solution in this case!"
        );
    }
    #[test]
    fn longer_loop() {
        let input = vec![(3, 5), (1, 3), (5, 1)];
        let expected = vec![(1, 3), (3, 5), (5, 1)];
        let solution = super::solve(&input);
        let check_res = solution.map(|result| {
            assert!(
                !result.is_empty(),
                "Solution should not be empty in this case!"
            );
            assert_eq!(result, rotate_to(&expected, &result[0]));
        });
        assert!(
            check_res != None,
            "There should be a solution in this case!"
        );
    }
    #[test]
    fn end_loop() {
        let input = vec![(1, 2), (4, 3), (2, 3), (3, 4)];
        let expected = Some(vec![(1, 2), (2, 3), (3, 4), (4, 3)]);
        assert_eq!(super::solve(&input), expected);
    }
    #[test]
    fn discontious_path() {
        let input = vec![(4, 5), (5, 1), (11, 9)];
        assert_eq!(super::solve(&input), None);
    }
    #[test]
    fn two_segments_one_destination() {
        let input = vec![(1, 2), (3, 2)];
        assert_eq!(super::solve(&input), None);
    }
    #[test]
    fn two_segments_one_destination_longer() {
        let input = vec![(1, 2), (3, 2), (2, 5), (5, 10)];
        assert_eq!(super::solve(&input), None);
    }
    #[test]
    fn empty() {
        let input = vec![];
        let expected = Some(vec![]);
        assert_eq!(super::solve(&input), expected);
    }
}
