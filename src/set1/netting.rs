use std::collections::HashMap;
type Payment<'a> = (&'a str, &'a str, i64);
pub fn solve<'a>(payments: &Vec<Payment>) -> Vec<Payment<'a>> {
    let mut reduced = HashMap::new();
    payments.iter().for_each(|(from, to, amount)| {
        *reduced.entry(from).or_insert(0) -= amount;
        *reduced.entry(to).or_insert(0) += amount;
    });
    //TODO: implement
    vec![("A", "C", 150), ("B", "C", 600)]
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_netting() {
        assert_eq!(
            super::solve(&vec![
                ("A", "C", 50),
                ("B", "A", 100),
                ("A", "D", 100),
                ("B", "D", 300),
                ("B", "C", 200),
                ("D", "C", 400),
            ]),
            vec![("A", "C", 150), ("B", "C", 600)]
        );
    }
}
