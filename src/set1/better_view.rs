pub fn solve(trees: &Vec<i64>) -> usize {
    //we compute the differences between neighboring tree heights
    let diffs = &trees.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();
    let num_negatives = diffs.iter().filter(|x| **x < 0).count();
    //check the number of negative height differences
    match num_negatives {
        //none -> we can remove any of the trees
        0 => trees.len(),
        //only one negative difference, we need to check if we can
        //restore the monotonicity via tree removal
        1 => {
            let neg_removal = &diffs
                .windows(2)
                .map(|x| x[0] + x[1])
                .filter(|x| *x < 0)
                .count();
            match neg_removal {
                0 => 2,
                1 => 1,
                _ => 0,
            }
        }
        //more than 1 negative height difference - monotonicity can't be
        //restored via just one tree removal
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_trees() {
        assert_eq!(super::solve(&vec![]), 0);
    }
    #[test]
    fn one_tree() {
        assert_eq!(super::solve(&vec![1]), 1);
    }
    #[test]
    fn two_trees() {
        assert_eq!(super::solve(&vec![10, 2]), 2);
    }
    #[test]
    fn right_edge_two_options() {
        assert_eq!(super::solve(&vec![3, 4, 5, 4]), 2);
    }
    #[test]
    fn right_edge_one_option() {
        assert_eq!(super::solve(&vec![3, 4, 5, 3]), 1);
    }
    #[test]
    fn left_edge_two_options() {
        assert_eq!(super::solve(&vec![5, 4, 5, 6]), 2);
    }
    #[test]
    fn left_edge_one_option() {
        assert_eq!(super::solve(&vec![10, 4, 5, 6]), 1);
    }
    #[test]
    fn middle_one_option() {
        assert_eq!(super::solve(&vec![3, 4, 6, 4, 5]), 1);
    }
}
