
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

        fn make_summator() -> UniquePtr<SummatorIface>;
        fn make_substitutor(pairs: &[SubstitutePair]) -> UniquePtr<SubstitutorIface>;

        fn summate(self: Pin<&mut SummatorIface>, parts: &[i64]) -> Result<i64>;
        fn substitute(self: &SubstitutorIface, orig: &str) -> Result<String>;
    }
}

pub fn log_message(buf: &cxx::CxxString) {
    println!("{}", buf);
}