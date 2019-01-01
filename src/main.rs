fn main() {
    use code_ninjas::set1::better_view::solve;
    let in1 = vec![3, 4, 5, 4, 6];
    let in2 = vec![3, 4, 5, 4];
    let in3 = vec![3, 4, 6, 4, 5];
    let res = solve(&in1);
    println!("{:?}", res);
}
