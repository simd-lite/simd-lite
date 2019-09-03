use super::*;
use crate::arm::neon::test_support::*;
use crate::arm::*;
use std::mem::transmute as __transmute;
use std::{i16, i32, i8, u16, u32, u8, vec::Vec};

macro_rules! transmute {
    ($v: expr) => {
        unsafe { __transmute(unsafe { $v }) }
    };
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vadd_s8() {
    test_ari_s8(
        |i, j| unsafe { vadd_s8(i, j) },
        |a: i8, b: i8| -> i8 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddq_s8() {
    testq_ari_s8(
        |i, j| unsafe { vaddq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vadd_s16() {
    test_ari_s16(
        |i, j| unsafe { vadd_s16(i, j) },
        |a: i16, b: i16| -> i16 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddq_s16() {
    testq_ari_s16(
        |i, j| unsafe { vaddq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vadd_s32() {
    test_ari_s32(
        |i, j| unsafe { vadd_s32(i, j) },
        |a: i32, b: i32| -> i32 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddq_s32() {
    testq_ari_s32(
        |i, j| unsafe { vaddq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a.overflowing_add(b).0 },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vadd_u8() {
    test_ari_u8(
        |i, j| unsafe { vadd_u8(i, j) },
        |a: u8, b: u8| -> u8 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddq_u8() {
    testq_ari_u8(
        |i, j| unsafe { vaddq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vadd_u16() {
    test_ari_u16(
        |i, j| unsafe { vadd_u16(i, j) },
        |a: u16, b: u16| -> u16 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddq_u16() {
    testq_ari_u16(
        |i, j| unsafe { vaddq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vadd_u32() {
    test_ari_u32(
        |i, j| unsafe { vadd_u32(i, j) },
        |a: u32, b: u32| -> u32 { a.overflowing_add(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddq_u32() {
    testq_ari_u32(
        |i, j| unsafe { vaddq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a.overflowing_add(b).0 },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vadd_f32() {
    test_ari_f32(
        |i, j| unsafe { vadd_f32(i, j) },
        |a: f32, b: f32| -> f32 { a + b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddq_f32() {
    testq_ari_f32(
        |i, j| unsafe { vaddq_f32(i, j) },
        |a: f32, b: f32| -> f32 { a + b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vand_s8() {
    test_bit_s8(
        |i, j| unsafe { vand_s8(i, j) },
        |a: i8, b: i8| -> i8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vandq_s8() {
    testq_bit_s8(
        |i, j| unsafe { vandq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vand_s16() {
    test_bit_s16(
        |i, j| unsafe { vand_s16(i, j) },
        |a: i16, b: i16| -> i16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vandq_s16() {
    testq_bit_s16(
        |i, j| unsafe { vandq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vand_s32() {
    test_bit_s32(
        |i, j| unsafe { vand_s32(i, j) },
        |a: i32, b: i32| -> i32 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vandq_s32() {
    testq_bit_s32(
        |i, j| unsafe { vandq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vand_s64() {
    test_bit_s64(
        |i, j| unsafe { vand_s64(i, j) },
        |a: i64, b: i64| -> i64 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vandq_s64() {
    testq_bit_s64(
        |i, j| unsafe { vandq_s64(i, j) },
        |a: i64, b: i64| -> i64 { a & b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vand_u8() {
    test_bit_u8(
        |i, j| unsafe { vand_u8(i, j) },
        |a: u8, b: u8| -> u8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vandq_u8() {
    testq_bit_u8(
        |i, j| unsafe { vandq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vand_u16() {
    test_bit_u16(
        |i, j| unsafe { vand_u16(i, j) },
        |a: u16, b: u16| -> u16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vandq_u16() {
    testq_bit_u16(
        |i, j| unsafe { vandq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vand_u32() {
    test_bit_u32(
        |i, j| unsafe { vand_u32(i, j) },
        |a: u32, b: u32| -> u32 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vandq_u32() {
    testq_bit_u32(
        |i, j| unsafe { vandq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vand_u64() {
    test_bit_u64(
        |i, j| unsafe { vand_u64(i, j) },
        |a: u64, b: u64| -> u64 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vandq_u64() {
    testq_bit_u64(
        |i, j| unsafe { vandq_u64(i, j) },
        |a: u64, b: u64| -> u64 { a & b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorr_s8() {
    test_bit_s8(
        |i, j| unsafe { vorr_s8(i, j) },
        |a: i8, b: i8| -> i8 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorrq_s8() {
    testq_bit_s8(
        |i, j| unsafe { vorrq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorr_s16() {
    test_bit_s16(
        |i, j| unsafe { vorr_s16(i, j) },
        |a: i16, b: i16| -> i16 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorrq_s16() {
    testq_bit_s16(
        |i, j| unsafe { vorrq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorr_s32() {
    test_bit_s32(
        |i, j| unsafe { vorr_s32(i, j) },
        |a: i32, b: i32| -> i32 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorrq_s32() {
    testq_bit_s32(
        |i, j| unsafe { vorrq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorr_s64() {
    test_bit_s64(
        |i, j| unsafe { vorr_s64(i, j) },
        |a: i64, b: i64| -> i64 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorrq_s64() {
    testq_bit_s64(
        |i, j| unsafe { vorrq_s64(i, j) },
        |a: i64, b: i64| -> i64 { a | b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorr_u8() {
    test_bit_u8(
        |i, j| unsafe { vorr_u8(i, j) },
        |a: u8, b: u8| -> u8 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorrq_u8() {
    testq_bit_u8(
        |i, j| unsafe { vorrq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorr_u16() {
    test_bit_u16(
        |i, j| unsafe { vorr_u16(i, j) },
        |a: u16, b: u16| -> u16 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorrq_u16() {
    testq_bit_u16(
        |i, j| unsafe { vorrq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorr_u32() {
    test_bit_u32(
        |i, j| unsafe { vorr_u32(i, j) },
        |a: u32, b: u32| -> u32 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorrq_u32() {
    testq_bit_u32(
        |i, j| unsafe { vorrq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorr_u64() {
    test_bit_u64(
        |i, j| unsafe { vorr_u64(i, j) },
        |a: u64, b: u64| -> u64 { a | b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vorrq_u64() {
    testq_bit_u64(
        |i, j| unsafe { vorrq_u64(i, j) },
        |a: u64, b: u64| -> u64 { a | b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veor_s8() {
    test_bit_s8(
        |i, j| unsafe { veor_s8(i, j) },
        |a: i8, b: i8| -> i8 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veorq_s8() {
    testq_bit_s8(
        |i, j| unsafe { veorq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veor_s16() {
    test_bit_s16(
        |i, j| unsafe { veor_s16(i, j) },
        |a: i16, b: i16| -> i16 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veorq_s16() {
    testq_bit_s16(
        |i, j| unsafe { veorq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veor_s32() {
    test_bit_s32(
        |i, j| unsafe { veor_s32(i, j) },
        |a: i32, b: i32| -> i32 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veorq_s32() {
    testq_bit_s32(
        |i, j| unsafe { veorq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veor_s64() {
    test_bit_s64(
        |i, j| unsafe { veor_s64(i, j) },
        |a: i64, b: i64| -> i64 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veorq_s64() {
    testq_bit_s64(
        |i, j| unsafe { veorq_s64(i, j) },
        |a: i64, b: i64| -> i64 { a ^ b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veor_u8() {
    test_bit_u8(
        |i, j| unsafe { veor_u8(i, j) },
        |a: u8, b: u8| -> u8 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veorq_u8() {
    testq_bit_u8(
        |i, j| unsafe { veorq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veor_u16() {
    test_bit_u16(
        |i, j| unsafe { veor_u16(i, j) },
        |a: u16, b: u16| -> u16 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veorq_u16() {
    testq_bit_u16(
        |i, j| unsafe { veorq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veor_u32() {
    test_bit_u32(
        |i, j| unsafe { veor_u32(i, j) },
        |a: u32, b: u32| -> u32 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veorq_u32() {
    testq_bit_u32(
        |i, j| unsafe { veorq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veor_u64() {
    test_bit_u64(
        |i, j| unsafe { veor_u64(i, j) },
        |a: u64, b: u64| -> u64 { a ^ b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_veorq_u64() {
    testq_bit_u64(
        |i, j| unsafe { veorq_u64(i, j) },
        |a: u64, b: u64| -> u64 { a ^ b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_s8() {
    test_cmp_s8(
        |i, j| unsafe { vceq_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a == b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_s8() {
    testq_cmp_s8(
        |i, j| unsafe { vceqq_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a == b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_s16() {
    test_cmp_s16(
        |i, j| unsafe { vceq_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a == b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_s16() {
    testq_cmp_s16(
        |i, j| unsafe { vceqq_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a == b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_s32() {
    test_cmp_s32(
        |i, j| unsafe { vceq_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a == b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_s32() {
    testq_cmp_s32(
        |i, j| unsafe { vceqq_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a == b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_u8() {
    test_cmp_u8(
        |i, j| unsafe { vceq_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a == b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_u8() {
    testq_cmp_u8(
        |i, j| unsafe { vceqq_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a == b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_u16() {
    test_cmp_u16(
        |i, j| unsafe { vceq_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a == b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_u16() {
    testq_cmp_u16(
        |i, j| unsafe { vceqq_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a == b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_u32() {
    test_cmp_u32(
        |i, j| unsafe { vceq_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a == b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_u32() {
    testq_cmp_u32(
        |i, j| unsafe { vceqq_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a == b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_f32() {
    test_cmp_f32(
        |i, j| unsafe { vcge_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a == b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_f32() {
    testq_cmp_f32(
        |i, j| unsafe { vcgeq_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a == b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_s8() {
    test_cmp_s8(
        |i, j| unsafe { vcgt_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a > b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_s8() {
    testq_cmp_s8(
        |i, j| unsafe { vcgtq_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a > b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_s16() {
    test_cmp_s16(
        |i, j| unsafe { vcgt_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a > b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_s16() {
    testq_cmp_s16(
        |i, j| unsafe { vcgtq_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a > b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_s32() {
    test_cmp_s32(
        |i, j| unsafe { vcgt_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a > b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_s32() {
    testq_cmp_s32(
        |i, j| unsafe { vcgtq_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a > b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_u8() {
    test_cmp_u8(
        |i, j| unsafe { vcgt_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a > b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_u8() {
    testq_cmp_u8(
        |i, j| unsafe { vcgtq_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a > b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_u16() {
    test_cmp_u16(
        |i, j| unsafe { vcgt_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a > b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_u16() {
    testq_cmp_u16(
        |i, j| unsafe { vcgtq_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a > b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_u32() {
    test_cmp_u32(
        |i, j| unsafe { vcgt_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a > b {
                0xFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_u32() {
    testq_cmp_u32(
        |i, j| unsafe { vcgtq_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a > b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_f32() {
    test_cmp_f32(
        |i, j| unsafe { vcgt_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a > b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_f32() {
    testq_cmp_f32(
        |i, j| unsafe { vcgtq_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a > b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vclt_s8() {
    test_cmp_s8(
        |i, j| unsafe { vclt_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a < b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_s8() {
    testq_cmp_s8(
        |i, j| unsafe { vcltq_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a < b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vclt_s16() {
    test_cmp_s16(
        |i, j| unsafe { vclt_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a < b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_s16() {
    testq_cmp_s16(
        |i, j| unsafe { vcltq_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a < b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vclt_s32() {
    test_cmp_s32(
        |i, j| unsafe { vclt_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a < b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_s32() {
    testq_cmp_s32(
        |i, j| unsafe { vcltq_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a < b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vclt_u8() {
    test_cmp_u8(
        |i, j| unsafe { vclt_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a < b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_u8() {
    testq_cmp_u8(
        |i, j| unsafe { vcltq_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a < b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vclt_u16() {
    test_cmp_u16(
        |i, j| unsafe { vclt_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a < b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_u16() {
    testq_cmp_u16(
        |i, j| unsafe { vcltq_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a < b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vclt_u32() {
    test_cmp_u32(
        |i, j| unsafe { vclt_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a < b {
                0xFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_u32() {
    testq_cmp_u32(
        |i, j| unsafe { vcltq_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a < b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vclt_f32() {
    test_cmp_f32(
        |i, j| unsafe { vclt_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a < b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_f32() {
    testq_cmp_f32(
        |i, j| unsafe { vcltq_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a < b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcle_s8() {
    test_cmp_s8(
        |i, j| unsafe { vcle_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a <= b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_s8() {
    testq_cmp_s8(
        |i, j| unsafe { vcleq_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a <= b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcle_s16() {
    test_cmp_s16(
        |i, j| unsafe { vcle_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a <= b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_s16() {
    testq_cmp_s16(
        |i, j| unsafe { vcleq_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a <= b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcle_s32() {
    test_cmp_s32(
        |i, j| unsafe { vcle_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a <= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_s32() {
    testq_cmp_s32(
        |i, j| unsafe { vcleq_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a <= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcle_u8() {
    test_cmp_u8(
        |i, j| unsafe { vcle_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a <= b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_u8() {
    testq_cmp_u8(
        |i, j| unsafe { vcleq_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a <= b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcle_u16() {
    test_cmp_u16(
        |i, j| unsafe { vcle_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a <= b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_u16() {
    testq_cmp_u16(
        |i, j| unsafe { vcleq_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a <= b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcle_u32() {
    test_cmp_u32(
        |i, j| unsafe { vcle_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a <= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_u32() {
    testq_cmp_u32(
        |i, j| unsafe { vcleq_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a <= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcle_f32() {
    test_cmp_f32(
        |i, j| unsafe { vcle_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a <= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_f32() {
    testq_cmp_f32(
        |i, j| unsafe { vcleq_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a <= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcge_s8() {
    test_cmp_s8(
        |i, j| unsafe { vcge_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a >= b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_s8() {
    testq_cmp_s8(
        |i, j| unsafe { vcgeq_s8(i, j) },
        |a: i8, b: i8| -> u8 {
            if a >= b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcge_s16() {
    test_cmp_s16(
        |i, j| unsafe { vcge_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a >= b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_s16() {
    testq_cmp_s16(
        |i, j| unsafe { vcgeq_s16(i, j) },
        |a: i16, b: i16| -> u16 {
            if a >= b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcge_s32() {
    test_cmp_s32(
        |i, j| unsafe { vcge_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a >= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_s32() {
    testq_cmp_s32(
        |i, j| unsafe { vcgeq_s32(i, j) },
        |a: i32, b: i32| -> u32 {
            if a >= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcge_u8() {
    test_cmp_u8(
        |i, j| unsafe { vcge_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a >= b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_u8() {
    testq_cmp_u8(
        |i, j| unsafe { vcgeq_u8(i, j) },
        |a: u8, b: u8| -> u8 {
            if a >= b {
                0xFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcge_u16() {
    test_cmp_u16(
        |i, j| unsafe { vcge_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a >= b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_u16() {
    testq_cmp_u16(
        |i, j| unsafe { vcgeq_u16(i, j) },
        |a: u16, b: u16| -> u16 {
            if a >= b {
                0xFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcge_u32() {
    test_cmp_u32(
        |i, j| unsafe { vcge_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a >= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_u32() {
    testq_cmp_u32(
        |i, j| unsafe { vcgeq_u32(i, j) },
        |a: u32, b: u32| -> u32 {
            if a >= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcge_f32() {
    test_cmp_f32(
        |i, j| unsafe { vcge_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a >= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_f32() {
    testq_cmp_f32(
        |i, j| unsafe { vcgeq_f32(i, j) },
        |a: f32, b: f32| -> u32 {
            if a >= b {
                0xFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsub_s8() {
    test_ari_s8(
        |i, j| unsafe { vqsub_s8(i, j) },
        |a: i8, b: i8| -> i8 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsubq_s8() {
    testq_ari_s8(
        |i, j| unsafe { vqsubq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsub_s16() {
    test_ari_s16(
        |i, j| unsafe { vqsub_s16(i, j) },
        |a: i16, b: i16| -> i16 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsubq_s16() {
    testq_ari_s16(
        |i, j| unsafe { vqsubq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsub_s32() {
    test_ari_s32(
        |i, j| unsafe { vqsub_s32(i, j) },
        |a: i32, b: i32| -> i32 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsubq_s32() {
    testq_ari_s32(
        |i, j| unsafe { vqsubq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a.saturating_sub(b) },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsub_u8() {
    test_ari_u8(
        |i, j| unsafe { vqsub_u8(i, j) },
        |a: u8, b: u8| -> u8 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsubq_u8() {
    testq_ari_u8(
        |i, j| unsafe { vqsubq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsub_u16() {
    test_ari_u16(
        |i, j| unsafe { vqsub_u16(i, j) },
        |a: u16, b: u16| -> u16 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsubq_u16() {
    testq_ari_u16(
        |i, j| unsafe { vqsubq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsub_u32() {
    test_ari_u32(
        |i, j| unsafe { vqsub_u32(i, j) },
        |a: u32, b: u32| -> u32 { a.saturating_sub(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqsubq_u32() {
    testq_ari_u32(
        |i, j| unsafe { vqsubq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a.saturating_sub(b) },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhadd_s8() {
    test_ari_s8(
        |i, j| unsafe { vhadd_s8(i, j) },
        |a: i8, b: i8| -> i8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhaddq_s8() {
    testq_ari_s8(
        |i, j| unsafe { vhaddq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhadd_s16() {
    test_ari_s16(
        |i, j| unsafe { vhadd_s16(i, j) },
        |a: i16, b: i16| -> i16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhaddq_s16() {
    testq_ari_s16(
        |i, j| unsafe { vhaddq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhadd_s32() {
    test_ari_s32(
        |i, j| unsafe { vhadd_s32(i, j) },
        |a: i32, b: i32| -> i32 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhaddq_s32() {
    testq_ari_s32(
        |i, j| unsafe { vhaddq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a & b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhadd_u8() {
    test_ari_u8(
        |i, j| unsafe { vhadd_u8(i, j) },
        |a: u8, b: u8| -> u8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhaddq_u8() {
    testq_ari_u8(
        |i, j| unsafe { vhaddq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhadd_u16() {
    test_ari_u16(
        |i, j| unsafe { vhadd_u16(i, j) },
        |a: u16, b: u16| -> u16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhaddq_u16() {
    testq_ari_u16(
        |i, j| unsafe { vhaddq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhadd_u32() {
    test_ari_u32(
        |i, j| unsafe { vhadd_u32(i, j) },
        |a: u32, b: u32| -> u32 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhaddq_u32() {
    testq_ari_u32(
        |i, j| unsafe { vhaddq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a & b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhadd_s8() {
    test_ari_s8(
        |i, j| unsafe { vrhadd_s8(i, j) },
        |a: i8, b: i8| -> i8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhaddq_s8() {
    testq_ari_s8(
        |i, j| unsafe { vrhaddq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhadd_s16() {
    test_ari_s16(
        |i, j| unsafe { vrhadd_s16(i, j) },
        |a: i16, b: i16| -> i16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhaddq_s16() {
    testq_ari_s16(
        |i, j| unsafe { vrhaddq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhadd_s32() {
    test_ari_s32(
        |i, j| unsafe { vrhadd_s32(i, j) },
        |a: i32, b: i32| -> i32 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhaddq_s32() {
    testq_ari_s32(
        |i, j| unsafe { vrhaddq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a & b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhadd_u8() {
    test_ari_u8(
        |i, j| unsafe { vrhadd_u8(i, j) },
        |a: u8, b: u8| -> u8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhaddq_u8() {
    testq_ari_u8(
        |i, j| unsafe { vrhaddq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhadd_u16() {
    test_ari_u16(
        |i, j| unsafe { vrhadd_u16(i, j) },
        |a: u16, b: u16| -> u16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhaddq_u16() {
    testq_ari_u16(
        |i, j| unsafe { vrhaddq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhadd_u32() {
    test_ari_u32(
        |i, j| unsafe { vrhadd_u32(i, j) },
        |a: u32, b: u32| -> u32 { a & b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrhaddq_u32() {
    testq_ari_u32(
        |i, j| unsafe { vrhaddq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a & b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqadd_s8() {
    test_ari_s8(
        |i, j| unsafe { vqadd_s8(i, j) },
        |a: i8, b: i8| -> i8 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqaddq_s8() {
    testq_ari_s8(
        |i, j| unsafe { vqaddq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqadd_s16() {
    test_ari_s16(
        |i, j| unsafe { vqadd_s16(i, j) },
        |a: i16, b: i16| -> i16 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqaddq_s16() {
    testq_ari_s16(
        |i, j| unsafe { vqaddq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqadd_s32() {
    test_ari_s32(
        |i, j| unsafe { vqadd_s32(i, j) },
        |a: i32, b: i32| -> i32 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqaddq_s32() {
    testq_ari_s32(
        |i, j| unsafe { vqaddq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a.saturating_add(b) },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqadd_u8() {
    test_ari_u8(
        |i, j| unsafe { vqadd_u8(i, j) },
        |a: u8, b: u8| -> u8 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqaddq_u8() {
    testq_ari_u8(
        |i, j| unsafe { vqaddq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqadd_u16() {
    test_ari_u16(
        |i, j| unsafe { vqadd_u16(i, j) },
        |a: u16, b: u16| -> u16 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqaddq_u16() {
    testq_ari_u16(
        |i, j| unsafe { vqaddq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqadd_u32() {
    test_ari_u32(
        |i, j| unsafe { vqadd_u32(i, j) },
        |a: u32, b: u32| -> u32 { a.saturating_add(b) },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vqaddq_u32() {
    testq_ari_u32(
        |i, j| unsafe { vqaddq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a.saturating_add(b) },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmul_s8() {
    test_ari_s8(
        |i, j| unsafe { vmul_s8(i, j) },
        |a: i8, b: i8| -> i8 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmulq_s8() {
    testq_ari_s8(
        |i, j| unsafe { vmulq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmul_s16() {
    test_ari_s16(
        |i, j| unsafe { vmul_s16(i, j) },
        |a: i16, b: i16| -> i16 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmulq_s16() {
    testq_ari_s16(
        |i, j| unsafe { vmulq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmul_s32() {
    test_ari_s32(
        |i, j| unsafe { vmul_s32(i, j) },
        |a: i32, b: i32| -> i32 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmulq_s32() {
    testq_ari_s32(
        |i, j| unsafe { vmulq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a.overflowing_mul(b).0 },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmul_u8() {
    test_ari_u8(
        |i, j| unsafe { vmul_u8(i, j) },
        |a: u8, b: u8| -> u8 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmulq_u8() {
    testq_ari_u8(
        |i, j| unsafe { vmulq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmul_u16() {
    test_ari_u16(
        |i, j| unsafe { vmul_u16(i, j) },
        |a: u16, b: u16| -> u16 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmulq_u16() {
    testq_ari_u16(
        |i, j| unsafe { vmulq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmul_u32() {
    test_ari_u32(
        |i, j| unsafe { vmul_u32(i, j) },
        |a: u32, b: u32| -> u32 { a.overflowing_mul(b).0 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmulq_u32() {
    testq_ari_u32(
        |i, j| unsafe { vmulq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a.overflowing_mul(b).0 },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmul_f32() {
    test_ari_f32(
        |i, j| unsafe { vmul_f32(i, j) },
        |a: f32, b: f32| -> f32 { a * b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmulq_f32() {
    testq_ari_f32(
        |i, j| unsafe { vmulq_f32(i, j) },
        |a: f32, b: f32| -> f32 { a * b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsub_s8() {
    test_ari_s8(
        |i, j| unsafe { vsub_s8(i, j) },
        |a: i8, b: i8| -> i8 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsubq_s8() {
    testq_ari_s8(
        |i, j| unsafe { vsubq_s8(i, j) },
        |a: i8, b: i8| -> i8 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsub_s16() {
    test_ari_s16(
        |i, j| unsafe { vsub_s16(i, j) },
        |a: i16, b: i16| -> i16 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsubq_s16() {
    testq_ari_s16(
        |i, j| unsafe { vsubq_s16(i, j) },
        |a: i16, b: i16| -> i16 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsub_s32() {
    test_ari_s32(
        |i, j| unsafe { vsub_s32(i, j) },
        |a: i32, b: i32| -> i32 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsubq_s32() {
    testq_ari_s32(
        |i, j| unsafe { vsubq_s32(i, j) },
        |a: i32, b: i32| -> i32 { a - b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsub_u8() {
    test_ari_u8(
        |i, j| unsafe { vsub_u8(i, j) },
        |a: u8, b: u8| -> u8 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsubq_u8() {
    testq_ari_u8(
        |i, j| unsafe { vsubq_u8(i, j) },
        |a: u8, b: u8| -> u8 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsub_u16() {
    test_ari_u16(
        |i, j| unsafe { vsub_u16(i, j) },
        |a: u16, b: u16| -> u16 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsubq_u16() {
    testq_ari_u16(
        |i, j| unsafe { vsubq_u16(i, j) },
        |a: u16, b: u16| -> u16 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsub_u32() {
    test_ari_u32(
        |i, j| unsafe { vsub_u32(i, j) },
        |a: u32, b: u32| -> u32 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsubq_u32() {
    testq_ari_u32(
        |i, j| unsafe { vsubq_u32(i, j) },
        |a: u32, b: u32| -> u32 { a - b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsub_f32() {
    test_ari_f32(
        |i, j| unsafe { vsub_f32(i, j) },
        |a: f32, b: f32| -> f32 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsubq_f32() {
    testq_ari_f32(
        |i, j| unsafe { vsubq_f32(i, j) },
        |a: f32, b: f32| -> f32 { a - b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsub_s8() {
    test_ari_s8(
        |i, j| unsafe { vhsub_s8(i, j) },
        |a: i8, b: i8| -> i8 { (((a as i16) - (b as i16)) / 2) as i8 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsubq_s8() {
    testq_ari_s8(
        |i, j| unsafe { vhsubq_s8(i, j) },
        |a: i8, b: i8| -> i8 { (((a as i16) - (b as i16)) / 2) as i8 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsub_s16() {
    test_ari_s16(
        |i, j| unsafe { vhsub_s16(i, j) },
        |a: i16, b: i16| -> i16 { (((a as i32) - (b as i32)) / 2) as i16 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsubq_s16() {
    testq_ari_s16(
        |i, j| unsafe { vhsubq_s16(i, j) },
        |a: i16, b: i16| -> i16 { (((a as i32) - (b as i32)) / 2) as i16 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsub_s32() {
    test_ari_s32(
        |i, j| unsafe { vhsub_s32(i, j) },
        |a: i32, b: i32| -> i32 { (((a as i64) - (b as i64)) / 2) as i32 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsubq_s32() {
    testq_ari_s32(
        |i, j| unsafe { vhsubq_s32(i, j) },
        |a: i32, b: i32| -> i32 { (((a as i64) - (b as i64)) / 2) as i32 },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsub_u8() {
    test_ari_u8(
        |i, j| unsafe { vhsub_u8(i, j) },
        |a: u8, b: u8| -> u8 { (((a as u16) - (b as u16)) / 2) as u8 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsubq_u8() {
    testq_ari_u8(
        |i, j| unsafe { vhsubq_u8(i, j) },
        |a: u8, b: u8| -> u8 { (((a as u16) - (b as u16)) / 2) as u8 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsub_u16() {
    test_ari_u16(
        |i, j| unsafe { vhsub_u16(i, j) },
        |a: u16, b: u16| -> u16 { (((a as u16) - (b as u16)) / 2) as u16 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsubq_u16() {
    testq_ari_u16(
        |i, j| unsafe { vhsubq_u16(i, j) },
        |a: u16, b: u16| -> u16 { (((a as u16) - (b as u16)) / 2) as u16 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsub_u32() {
    test_ari_u32(
        |i, j| unsafe { vhsub_u32(i, j) },
        |a: u32, b: u32| -> u32 { (((a as u64) - (b as u64)) / 2) as u32 },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vhsubq_u32() {
    testq_ari_u32(
        |i, j| unsafe { vhsubq_u32(i, j) },
        |a: u32, b: u32| -> u32 { (((a as u64) - (b as u64)) / 2) as u32 },
    );
}
