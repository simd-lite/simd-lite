
use super::*;
use crate::arm::neon::test_support::*;
use crate::{arm::*, simd::*};
use std::mem::transmute as __transmute;
use std::{i16, i32, i8, u16, u32, u8, vec::Vec};

macro_rules! transmute {
    ($v: expr) => {
        unsafe { __transmute(unsafe { $v }) }
    };
}

//    use stdarch_test::simd_test;

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vld1q_s8() {
    let a = i8x16::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let e = a;
    let r: i8x16 = unsafe { transmute!(vld1q_s8(transmute!(&a))) };
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vld1q_u8() {
    let a = u8x16::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let e = a;
    let r: u8x16 = unsafe { transmute!(vld1q_u8(transmute!(&a))) };
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vget_lane_u8() {
    let v = i8x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let r = unsafe { vget_lane_u8(transmute!(v), 1) };
    assert_eq!(r, 2);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vgetq_lane_u32() {
    let v = i32x4::new(1, 2, 3, 4);
    let r = unsafe { vgetq_lane_u32(transmute!(v), 1) };
    assert_eq!(r, 2);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vget_lane_u64() {
    let v: u64 = 1;
    let r = unsafe { vget_lane_u64(transmute!(v), 0) };
    assert_eq!(r, 1);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vgetq_lane_u16() {
    let v = i16x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let r = unsafe { vgetq_lane_u16(transmute!(v), 1) };
    assert_eq!(r, 2);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vextq_s8() {
    let a = i8x16::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let b = i8x16::new(
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 31, 32,
    );
    let e = i8x16::new(4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    let r: i8x16 = unsafe { transmute!(vextq_s8(transmute!(a), transmute!(b), 3)) };
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vextq_u8() {
    let a = u8x16::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let b = u8x16::new(
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 31, 32,
    );
    let e = u8x16::new(4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    let r: u8x16 = unsafe { transmute!(vextq_u8(transmute!(a), transmute!(b), 3)) };
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vshrq_n_u8() {
    let a = u8x16::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let e = u8x16::new(0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4);
    let r: u8x16 = unsafe { transmute!(vshrq_n_u8(transmute!(a), 2)) };
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vshlq_n_u8() {
    let a = u8x16::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let e = u8x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64);
    let r: u8x16 = unsafe { transmute!(vshlq_n_u8(transmute!(a), 2)) };
    assert_eq!(r, e);
}

#[test]
//FIXME: #[simd_test(enable = "neon")]
//    fn test_vqmovn_u64() {
//        let a = u64x2::new(1, 2);
//        let e = u32x2::new(1, 2);
//        let r: u32x2 = transmute!(vqmovn_u64(transmute!(a)));
//        assert_eq!(r, e);
//    }
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vreinterpret_u64_u32() {
    let v: i8 = 42;
    let e = i8x16::new(
        42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
    );
    let r: i8x16 = unsafe { transmute!(vdupq_n_s8(v)) };
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vdupq_n_s8() {
    let v: i8 = 42;
    let e = i8x16::new(
        42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
    );
    let r: i8x16 = unsafe { transmute!(vdupq_n_s8(v)) };
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vdupq_n_u8() {
    let v: u8 = 42;
    let e = u8x16::new(
        42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
    );
    let r: u8x16 = unsafe { transmute!(vdupq_n_u8(v)) };
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovq_n_u8() {
    let v: u8 = 42;
    let e = u8x16::new(
        42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
    );
    let r: u8x16 = unsafe { transmute!(vmovq_n_u8(v)) };
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vgetq_lane_u64() {
    let v = i64x2::new(1, 2);
    let r = unsafe { vgetq_lane_u64(transmute!(v), 1) };
    assert_eq!(r, 2);
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
fn test_vaddl_s8() {
    let v = i8::MAX;
    let a = i8x8::new(v, v, v, v, v, v, v, v);
    let v = 2 * (v as i16);
    let e = i16x8::new(v, v, v, v, v, v, v, v);
    let r: i16x8 = transmute!(vaddl_s8(transmute!(a), transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddl_s16() {
    let v = i16::MAX;
    let a = i16x4::new(v, v, v, v);
    let v = 2 * (v as i32);
    let e = i32x4::new(v, v, v, v);
    let r: i32x4 = transmute!(vaddl_s16(transmute!(a), transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddl_s32() {
    let v = i32::MAX;
    let a = i32x2::new(v, v);
    let v = 2 * (v as i64);
    let e = i64x2::new(v, v);
    let r: i64x2 = transmute!(vaddl_s32(transmute!(a), transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddl_u8() {
    let v = u8::MAX;
    let a = u8x8::new(v, v, v, v, v, v, v, v);
    let v = 2 * (v as u16);
    let e = u16x8::new(v, v, v, v, v, v, v, v);
    let r: u16x8 = transmute!(vaddl_u8(transmute!(a), transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddl_u16() {
    let v = u16::MAX;
    let a = u16x4::new(v, v, v, v);
    let v = 2 * (v as u32);
    let e = u32x4::new(v, v, v, v);
    let r: u32x4 = transmute!(vaddl_u16(transmute!(a), transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddl_u32() {
    let v = u32::MAX;
    let a = u32x2::new(v, v);
    let v = 2 * (v as u64);
    let e = u64x2::new(v, v);
    let r: u64x2 = transmute!(vaddl_u32(transmute!(a), transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvn_s8() {
    let a = i8x8::new(0, 1, 2, 3, 4, 5, 6, 7);
    let e = i8x8::new(-1, -2, -3, -4, -5, -6, -7, -8);
    let r: i8x8 = transmute!(vmvn_s8(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvnq_s8() {
    let a = i8x16::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    let e = i8x16::new(
        -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16,
    );
    let r: i8x16 = transmute!(vmvnq_s8(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvn_s16() {
    let a = i16x4::new(0, 1, 2, 3);
    let e = i16x4::new(-1, -2, -3, -4);
    let r: i16x4 = transmute!(vmvn_s16(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvnq_s16() {
    let a = i16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
    let e = i16x8::new(-1, -2, -3, -4, -5, -6, -7, -8);
    let r: i16x8 = transmute!(vmvnq_s16(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvn_s32() {
    let a = i32x2::new(0, 1);
    let e = i32x2::new(-1, -2);
    let r: i32x2 = transmute!(vmvn_s32(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvnq_s32() {
    let a = i32x4::new(0, 1, 2, 3);
    let e = i32x4::new(-1, -2, -3, -4);
    let r: i32x4 = transmute!(vmvnq_s32(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvn_u8() {
    let a = u8x8::new(0, 1, 2, 3, 4, 5, 6, 7);
    let e = u8x8::new(255, 254, 253, 252, 251, 250, 249, 248);
    let r: u8x8 = transmute!(vmvn_u8(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvnq_u8() {
    let a = u8x16::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    let e = u8x16::new(
        255, 254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240,
    );
    let r: u8x16 = transmute!(vmvnq_u8(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvn_u16() {
    let a = u16x4::new(0, 1, 2, 3);
    let e = u16x4::new(65_535, 65_534, 65_533, 65_532);
    let r: u16x4 = transmute!(vmvn_u16(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvnq_u16() {
    let a = u16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
    let e = u16x8::new(
        65_535, 65_534, 65_533, 65_532, 65_531, 65_530, 65_529, 65_528,
    );
    let r: u16x8 = transmute!(vmvnq_u16(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvn_u32() {
    let a = u32x2::new(0, 1);
    let e = u32x2::new(4_294_967_295, 4_294_967_294);
    let r: u32x2 = transmute!(vmvn_u32(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvnq_u32() {
    let a = u32x4::new(0, 1, 2, 3);
    let e = u32x4::new(4_294_967_295, 4_294_967_294, 4_294_967_293, 4_294_967_292);
    let r: u32x4 = transmute!(vmvnq_u32(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvn_p8() {
    let a = u8x8::new(0, 1, 2, 3, 4, 5, 6, 7);
    let e = u8x8::new(255, 254, 253, 252, 251, 250, 249, 248);
    let r: u8x8 = transmute!(vmvn_p8(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmvnq_p8() {
    let a = u8x16::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    let e = u8x16::new(
        255, 254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240,
    );
    let r: u8x16 = transmute!(vmvnq_p8(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovn_s16() {
    let a = i16x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let e = i8x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let r: i8x8 = transmute!(vmovn_s16(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovn_s32() {
    let a = i32x4::new(1, 2, 3, 4);
    let e = i16x4::new(1, 2, 3, 4);
    let r: i16x4 = transmute!(vmovn_s32(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovn_s64() {
    let a = i64x2::new(1, 2);
    let e = i32x2::new(1, 2);
    let r: i32x2 = transmute!(vmovn_s64(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovn_u16() {
    let a = u16x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let e = u8x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let r: u8x8 = transmute!(vmovn_u16(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovn_u32() {
    let a = u32x4::new(1, 2, 3, 4);
    let e = u16x4::new(1, 2, 3, 4);
    let r: u16x4 = transmute!(vmovn_u32(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovn_u64() {
    let a = u64x2::new(1, 2);
    let e = u32x2::new(1, 2);
    let r: u32x2 = transmute!(vmovn_u64(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovl_s8() {
    let e = i16x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let a = i8x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let r: i16x8 = transmute!(vmovl_s8(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovl_s16() {
    let e = i32x4::new(1, 2, 3, 4);
    let a = i16x4::new(1, 2, 3, 4);
    let r: i32x4 = transmute!(vmovl_s16(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovl_s32() {
    let e = i64x2::new(1, 2);
    let a = i32x2::new(1, 2);
    let r: i64x2 = transmute!(vmovl_s32(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovl_u8() {
    let e = u16x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let a = u8x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let r: u16x8 = transmute!(vmovl_u8(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovl_u16() {
    let e = u32x4::new(1, 2, 3, 4);
    let a = u16x4::new(1, 2, 3, 4);
    let r: u32x4 = transmute!(vmovl_u16(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmovl_u32() {
    let e = u64x2::new(1, 2);
    let a = u32x2::new(1, 2);
    let r: u64x2 = transmute!(vmovl_u32(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vrsqrt_f32() {
    let a = f32x2::new(1.0, 2.0);
    let e = f32x2::new(0.9980469, 0.7050781);
    let r: f32x2 = transmute!(vrsqrte_f32(transmute!(a)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmin_s8() {
    let a = i8x8::new(1, -2, 3, -4, 5, 6, 7, 8);
    let b = i8x8::new(0, 3, 2, 5, 4, 7, 6, 9);
    let e = i8x8::new(-2, -4, 5, 7, 0, 2, 4, 6);
    let r: i8x8 = transmute!(vpmin_s8(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmin_s16() {
    let a = i16x4::new(1, 2, 3, -4);
    let b = i16x4::new(0, 3, 2, 5);
    let e = i16x4::new(1, -4, 0, 2);
    let r: i16x4 = transmute!(vpmin_s16(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmin_s32() {
    let a = i32x2::new(1, -2);
    let b = i32x2::new(0, 3);
    let e = i32x2::new(-2, 0);
    let r: i32x2 = transmute!(vpmin_s32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmin_u8() {
    let a = u8x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let b = u8x8::new(0, 3, 2, 5, 4, 7, 6, 9);
    let e = u8x8::new(1, 3, 5, 7, 0, 2, 4, 6);
    let r: u8x8 = transmute!(vpmin_u8(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmin_u16() {
    let a = u16x4::new(1, 2, 3, 4);
    let b = u16x4::new(0, 3, 2, 5);
    let e = u16x4::new(1, 3, 0, 2);
    let r: u16x4 = transmute!(vpmin_u16(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmin_u32() {
    let a = u32x2::new(1, 2);
    let b = u32x2::new(0, 3);
    let e = u32x2::new(1, 0);
    let r: u32x2 = transmute!(vpmin_u32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmin_f32() {
    let a = f32x2::new(1., -2.);
    let b = f32x2::new(0., 3.);
    let e = f32x2::new(-2., 0.);
    let r: f32x2 = transmute!(vpmin_f32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmax_s8() {
    let a = i8x8::new(1, -2, 3, -4, 5, 6, 7, 8);
    let b = i8x8::new(0, 3, 2, 5, 4, 7, 6, 9);
    let e = i8x8::new(1, 3, 6, 8, 3, 5, 7, 9);
    let r: i8x8 = transmute!(vpmax_s8(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmax_s16() {
    let a = i16x4::new(1, 2, 3, -4);
    let b = i16x4::new(0, 3, 2, 5);
    let e = i16x4::new(2, 3, 3, 5);
    let r: i16x4 = transmute!(vpmax_s16(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmax_s32() {
    let a = i32x2::new(1, -2);
    let b = i32x2::new(0, 3);
    let e = i32x2::new(1, 3);
    let r: i32x2 = transmute!(vpmax_s32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmax_u8() {
    let a = u8x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let b = u8x8::new(0, 3, 2, 5, 4, 7, 6, 9);
    let e = u8x8::new(2, 4, 6, 8, 3, 5, 7, 9);
    let r: u8x8 = transmute!(vpmax_u8(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmax_u16() {
    let a = u16x4::new(1, 2, 3, 4);
    let b = u16x4::new(0, 3, 2, 5);
    let e = u16x4::new(2, 4, 3, 5);
    let r: u16x4 = transmute!(vpmax_u16(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmax_u32() {
    let a = u32x2::new(1, 2);
    let b = u32x2::new(0, 3);
    let e = u32x2::new(2, 3);
    let r: u32x2 = transmute!(vpmax_u32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmax_f32() {
    let a = f32x2::new(1., -2.);
    let b = f32x2::new(0., 3.);
    let e = f32x2::new(1., 3.);
    let r: f32x2 = transmute!(vpmax_f32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
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
