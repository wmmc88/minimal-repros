#[cfg(not(test))]
#[link(name = "foo")]
unsafe extern "C" {
}

#[cfg(test)]
#[link(name = "testfoo")]
unsafe extern "C" {}

unsafe extern "C" {
    fn print_library_info();
    #[cfg(not(test))]
    fn foo_specific_function();
    #[cfg(test)]
    fn testfoo_specific_function();
}

fn foo() {
    println!("Testing conditional linking with cfg_if:");
    println!("  - cargo build links to libfoo");
    println!("  - cargo test links to libtestfoo");
    println!("");
    println!("print_library_info: ");
    unsafe {print_library_info()};

    #[cfg(test)]
    println!("testfoo_specific_function: ");
    #[cfg(test)]
    unsafe {testfoo_specific_function()};

    #[cfg(not(test))]
    println!("foo_specific_function: ");
    #[cfg(not(test))]
    unsafe {foo_specific_function()};
}

fn main() {
    foo();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_linking() {
        foo();
    }
}
