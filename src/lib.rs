mod outer {
    #[cfg_attr(test, mockall::automock)]
    pub mod ffi {
        extern "C" {
            pub fn variadic(a: u32, b: u32, ...);
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
        let ctx = ffi::variadic_context();
        ctx.expect();

        unsafe {
            ffi::variadic(1, 2, 3);
        }
    }
}
