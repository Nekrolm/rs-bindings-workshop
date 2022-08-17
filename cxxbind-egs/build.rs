fn main() {
    cxx_build::bridge("src/bridge.rs")
        .file("cxx_dep_bridge/cxx_dep_bridge.cpp")
        .flag_if_supported("-std=c++17")
        .compile("cxx-demo");
}