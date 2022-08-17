pub mod bridge;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summator_ok() {
        let mut summator = bridge::ffi::make_summator();
        let summator = summator.pin_mut();
        let sum = summator.summate(&[1,2,3,4,5]);
        assert!(matches!(sum, Ok(15)))
    }


    #[test]
    fn test_summator_err() {
        let mut summator = bridge::ffi::make_summator();
        let summator = summator.pin_mut();
        let sum = summator.summate(&[]);
        let err = sum.expect_err("there should be error");
        println!("{err:?}")
    }
}
