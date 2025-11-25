#[cfg(test)]
mod tests {
    #[test]
    fn testa() {
        macrotest::expand("tests/expand/a.rs");
    }

    #[test]
    fn testb() {
        macrotest::expand("tests/expand/b.rs");
    }
}
