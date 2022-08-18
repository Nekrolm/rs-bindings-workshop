
#[cxx::bridge(namespace = "cxx_dep")]
pub(crate) mod ffi {

    struct SubstitutePair<'a> {
        from : &'a str,
        to: &'a str,
    }

    extern "Rust" {
        fn log_message(buf: &CxxString);
    }

    unsafe extern "C++" {
        include!("cxxbind-egs/cxx_dep_bridge/cxx_dep_bridge.hpp");

        type SummatorIface;
        type SubstitutorIface;

        type ByRefSubstitutor<'a>;

        fn make_summator() -> UniquePtr<SummatorIface>;
        fn make_substitutor(pairs: &[SubstitutePair]) -> UniquePtr<SubstitutorIface>;
        fn make_by_ref_substitutor<'a>(pairs: &[SubstitutePair<'a>]) -> UniquePtr<ByRefSubstitutor<'a>>;
        

        fn summate(self: Pin<&mut SummatorIface>, parts: &[i64]) -> Result<i64>;
        fn substitute(self: &SubstitutorIface, orig: &str) -> Result<String>;
        fn substitute<'a>(self: &ByRefSubstitutor<'a>, orig: &str) -> Result<&'a str>;
    }
}

pub fn log_message(buf: &cxx::CxxString) {
    println!("{}", buf);
}