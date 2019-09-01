#![allow(non_camel_case_types)]

mod generated;
pub use crate::arm::*;
pub use generated::*;
pub use std::arch::aarch64::*;
use std::hint::unreachable_unchecked;
use std::mem::transmute;
use std::ptr;

#[cfg(test)]
use assert_instr_macro::assert_instr;

//FIXME move to macros or see if we can reuse
macro_rules! types {
    ($(
        $(#[$doc:meta])*
        pub struct $name:ident($($fields:tt)*);
    )*) => ($(
        $(#[$doc])*
        #[derive(Copy, Clone, Debug)]
        #[allow(non_camel_case_types)]
        #[repr(simd)]
        #[allow(clippy::missing_inline_in_public_items)]
        pub struct $name($($fields)*);
    )*)
}

types! {
    /// ARM-specific 64-bit wide vector of one packed `p64`.
    pub struct poly64_t(i64); // FIXME: check this!
    /// ARM-specific 128-bit wide vector of one packed `p64`.
    pub struct poly128_t(i128); // FIXME: check this!
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.aarch64.neon.pmull64"]
    fn vmull_p64_(a: i64, b: i64) -> int8x16_t;

    #[link_name = "llvm.aarch64.neon.addp.v16i8"]
    fn vpaddq_u8_(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
}

/// Polynomial multiply long
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(test, assert_instr(pmull))]
pub unsafe fn vmull_p64(a: poly64_t, b: poly64_t) -> poly128_t {
    transmute(vmull_p64_(transmute(a), transmute(b)))
}

/// Add pairwise
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn vpaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    vpaddq_u8_(a, b)
}

#[cfg(test)]
mod tests {
    use crate::aarch64::*;
    use crate::cmparm::cmp_arm;
    use simd_test_macro::simd_test;
    use std::arch::aarch64::*;
    use std::mem::transmute;
    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vpaddq_u8() {
        unsafe {
            let a: [i8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
            let b: [i8; 16] = [
                17, 18, 19, 20, 20, 21, 22, 23, 24, 25, 26, 27, 29, 29, 30, 31,
            ];
            let r: [i8; 16] = [1, 5, 9, 13, 17, 21, 25, 29, 35, 39, 41, 45, 49, 53, 58, 61];
            let e = vpaddq_u8(transmute(a), transmute(b));
            assert!(cmp_arm(transmute(r), e));
        }
    }

    //#[simd_test(enable = "neon")]
    fn test_vmull_p64() {
        unsafe {
            // FIXME: I've a hard time writing a test for this as the documentation
            // from arm is a bit thin as to waht exactly it does
            let a: i64 = 8;
            let b: i64 = 7;
            let e: i128 = 56;
            let r: i128 = transmute(vmull_p64(transmute(a), transmute(b)));
            assert_eq!(r, e);
        }
    }
}

//#[cfg(test)]
pub(crate) mod neon {
    #[cfg(test)]
    pub(crate) mod test_support;
    #[cfg(test)]
    mod tests;
    //    #[cfg(test)]
    //    #[cfg(target_endian = "little")]
    //    #[path = "../../arm/neon/table_lookup_tests.rs"]
    //    mod table_lookup_tests;
}
