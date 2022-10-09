
fn main() {
    println!("I'm the current version v{}", lib::version());
    println!("I'm compat1 version v{}", lib_compat1::version());
    println!("I'm compat2 version v{}", lib_compat2::version());
}
