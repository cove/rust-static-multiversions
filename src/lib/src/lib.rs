pub mod lib {
        pub fn version() {}
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() -> &'static str {
    return VERSION;
}
