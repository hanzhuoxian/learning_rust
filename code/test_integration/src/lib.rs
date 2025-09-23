pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_add() {
        let sum = add(1, 2);
        assert_eq!(sum, 3)
    }
}
