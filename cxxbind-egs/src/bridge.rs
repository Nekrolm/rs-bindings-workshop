
#[cxx::bridge(namespace = "cxx_dep")]
pub(crate) mod ffi {
    extern "Rust" {
        fn log_message(buf: &CxxString);
    }

    unsafe extern "C++" {
        include!("cxxbind-egs/cxx_dep_bridge/cxx_dep_bridge.hpp");

        type SummatorIface;

        fn make_summator() -> UniquePtr<SummatorIface>;
        fn summate(self: Pin<&mut Self>, parts: &[i64]) -> Result<i64>;
    }
}

pub fn log_message(buf: &cxx::CxxString) {
    println!("{}", buf);
}