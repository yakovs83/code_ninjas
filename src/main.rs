fn main() {
    use code_ninjas::set1::sort_segments::solve;
    let input = vec![(3, 5), (1, 3), (5, 1)]; //simple loop
                                              //let input = vec![(4, 5), (9, 4), (5, 1), (11, 9)]; //simple continuous path
    let res = solve(&input);
    println!("{:?}", res);
}
