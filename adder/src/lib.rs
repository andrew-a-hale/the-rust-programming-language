pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if add(2, 2) == 3 {
            Ok(())
        } else {
            Err("it does not work".to_string())
        }
    }
}
