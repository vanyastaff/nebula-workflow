pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Example public function for documentation
pub fn hello_core() -> &'static str {
    "Hello from nebula-core!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
