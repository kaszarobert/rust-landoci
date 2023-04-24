fn main() {
    println!("{}", add_numbers(1, 2));
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::add_numbers;

    #[test]
    fn positive_number_adding() {
        let a = 1;
        let b = 2;
        let expected: i32 = 3;
        let actual: i32 = add_numbers(a, b);
        assert_eq!(expected, actual);
    }

    #[test]
    fn negative_number_adding() {
        let a = -1;
        let b = -2;
        let expected: i32 = -3;
        let actual: i32 = add_numbers(a, b);
        assert_eq!(expected, actual);
    }
}
