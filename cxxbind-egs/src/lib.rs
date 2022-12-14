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

    fn make_test_substitutor() -> cxx::UniquePtr<bridge::ffi::SubstitutorIface>{
        let pairs = &[
            bridge::ffi::SubstitutePair {
                from : "hello",
                to : "world",
            }
        ];
        bridge::ffi::make_substitutor(pairs)
    }


    #[test]
    fn test_substitutor_ok() {
        let subs = make_test_substitutor();
        let subs = subs.as_ref().expect("should not be none");
        let replaced = subs.substitute("hello").unwrap();
        assert_eq!(replaced, "world");
    }

    #[test]
    fn test_substitutor_err() {
        let subs = make_test_substitutor();
        let subs = subs.as_ref().expect("should not be none");
        let replaced_err = subs.substitute("Мир").expect_err("should be error");
        println!("{replaced_err:?}")
    }

    #[test]
    fn test_by_ref_substitutor() {
        let from = "hello".to_string();
        let to = "world".to_string();
        let strings = vec![
            bridge::ffi::SubstitutePair {
                from : &from,
                to : &to,
            }
        ];
        let subs = bridge::ffi::make_by_ref_substitutor(&strings);
        let subs = subs.as_ref().unwrap();
        let replaced = subs.substitute("hello").unwrap();
        assert_eq!(replaced, to);
        assert_eq!(replaced.as_ptr(), to.as_str().as_ptr());

        // drop(from) -- Compile error, from is borrowed by substitutor
    }
}
