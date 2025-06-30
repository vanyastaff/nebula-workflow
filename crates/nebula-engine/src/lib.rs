pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Example public function for documentation
pub fn hello_engine() -> &'static str {
    "Hello from nebula-engine!"
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
