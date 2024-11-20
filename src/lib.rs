mod outer {
    #[cfg_attr(test, mockall::automock)]
    pub mod ffi {
        extern "C" {
            pub fn never_returns() -> !;
        }
    }
}

#[mockall_double::double]
pub use outer::ffi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() {
        let ctx = ffi::never_returns_context();
        ctx.expect();

        unsafe {
            ffi::never_returns();
        }
    }
}
