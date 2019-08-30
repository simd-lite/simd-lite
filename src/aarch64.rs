mod generated;
pub use crate::arm::*;
pub use generated::*;
use std::arch::aarch64::*;
use std::hint::unreachable_unchecked;
use std::mem::transmute;
use std::ptr;

#[cfg(test)]
use assert_instr_macro::assert_instr;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.aarch64.neon.addp.v16i8"]
    fn vpaddq_u8_(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
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
    use crate::arm::cmp_arm;
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