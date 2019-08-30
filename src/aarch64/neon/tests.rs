
use crate::aarch64::neon::test_support::*;
use crate::arm::neon::test_support::*;
use crate::{aarch64::neon::*, aarch64::*, simd::*};
use std::mem::transmute as __transmute;

macro_rules! transmute {
    ($v: expr) => {
        unsafe { __transmute(unsafe { $v }) }
    };
}

//    use stdarch_test::simd_test;

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpaddq_u8() {
    let a = i8x16::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    let b = i8x16::new(
        17, 18, 19, 20, 20, 21, 22, 23, 24, 25, 26, 27, 29, 29, 30, 31,
    );
    let r = i8x16(1, 5, 9, 13, 17, 21, 25, 29, 35, 39, 41, 45, 49, 53, 58, 61);
    let e: i8x16 = unsafe { transmute!(vpaddq_u8(transmute!(a), transmute!(b))) };
    assert_eq!(r, e);
}

//    #[test]  //FIXME: #[simd_test(enable = "neon")]
//    fn test_vmull_p64() {
//        // FIXME: I've a hard time writing a test for this as the documentation
//        // from arm is a bit thin as to waht exactly it does
//        let a: i64 = 8;
//        let b: i64 = 7;
//        let e: i128 = 56;
//        let r: i128 = unsafe { transmute!(vmull_p64(transmute!(a), transmute!(b))) };
//        assert_eq!(r, e);
//
//        /*
//        let a: i64 = 5;
//        let b: i64 = 5;
//        let e: i128 = 25;
//        let r: i128 = transmute!(vmull_p64(a, b));
//
//        assert_eq!(r, e);
//        let a: i64 = 6;
//        let b: i64 = 6;
//        let e: i128 = 36;
//        let r: i128 = transmute!(vmull_p64(a, b));
//        assert_eq!(r, e);
//
//        let a: i64 = 7;
//        let b: i64 = 6;
//        let e: i128 = 42;
//        let r: i128 = transmute!(vmull_p64(a, b));
//        assert_eq!(r, e);
//        */
//    }
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vadd_f64() {
    let a = 1.;
    let b = 8.;
    let e = 9.;
    let r: f64 = transmute!(vadd_f64(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddq_f64() {
    let a = f64x2::new(1., 2.);
    let b = f64x2::new(8., 7.);
    let e = f64x2::new(9., 9.);
    let r: f64x2 = transmute!(vaddq_f64(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddd_s64() {
    let a = 1_i64;
    let b = 8_i64;
    let e = 9_i64;
    let r: i64 = transmute!(vaddd_s64(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vaddd_u64() {
    let a = 1_u64;
    let b = 8_u64;
    let e = 9_u64;
    let r: u64 = transmute!(vaddd_u64(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxv_s8() {
    let r = unsafe { vmaxv_s8(transmute!(i8x8::new(1, 2, 3, 4, -8, 6, 7, 5))) };
    assert_eq!(r, 7_i8);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxvq_s8() {
    #[rustfmt::skip]
            let r = unsafe { vmaxvq_s8(transmute!(i8x16::new(
            1, 2, 3, 4,
            -16, 6, 7, 5,
            8, 1, 1, 1,
            1, 1, 1, 1,
        ))) };
    assert_eq!(r, 8_i8);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxv_s16() {
    let r = unsafe { vmaxv_s16(transmute!(i16x4::new(1, 2, -4, 3))) };
    assert_eq!(r, 3_i16);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxvq_s16() {
    let r = unsafe { vmaxvq_s16(transmute!(i16x8::new(1, 2, 7, 4, -16, 6, 7, 5))) };
    assert_eq!(r, 7_i16);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxv_s32() {
    let r = unsafe { vmaxv_s32(transmute!(i32x2::new(1, -4))) };
    assert_eq!(r, 1_i32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxvq_s32() {
    let r = unsafe { vmaxvq_s32(transmute!(i32x4::new(1, 2, -32, 4))) };
    assert_eq!(r, 4_i32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxv_u8() {
    let r = unsafe { vmaxv_u8(transmute!(u8x8::new(1, 2, 3, 4, 8, 6, 7, 5))) };
    assert_eq!(r, 8_u8);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxvq_u8() {
    #[rustfmt::skip]
            let r = unsafe { vmaxvq_u8(transmute!(u8x16::new(
            1, 2, 3, 4,
            16, 6, 7, 5,
            8, 1, 1, 1,
            1, 1, 1, 1,
        ))) };
    assert_eq!(r, 16_u8);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxv_u16() {
    let r = unsafe { vmaxv_u16(transmute!(u16x4::new(1, 2, 4, 3))) };
    assert_eq!(r, 4_u16);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxvq_u16() {
    let r = unsafe { vmaxvq_u16(transmute!(u16x8::new(1, 2, 7, 4, 16, 6, 7, 5))) };
    assert_eq!(r, 16_u16);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxv_u32() {
    let r = unsafe { vmaxv_u32(transmute!(u32x2::new(1, 4))) };
    assert_eq!(r, 4_u32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxvq_u32() {
    let r = unsafe { vmaxvq_u32(transmute!(u32x4::new(1, 2, 32, 4))) };
    assert_eq!(r, 32_u32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxv_f32() {
    let r = unsafe { vmaxv_f32(transmute!(f32x2::new(1., 4.))) };
    assert_eq!(r, 4_f32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxvq_f32() {
    let r = unsafe { vmaxvq_f32(transmute!(f32x4::new(1., 2., 32., 4.))) };
    assert_eq!(r, 32_f32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmaxvq_f64() {
    let r = unsafe { vmaxvq_f64(transmute!(f64x2::new(1., 4.))) };
    assert_eq!(r, 4_f64);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminv_s8() {
    let r = unsafe { vminv_s8(transmute!(i8x8::new(1, 2, 3, 4, -8, 6, 7, 5))) };
    assert_eq!(r, -8_i8);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminvq_s8() {
    #[rustfmt::skip]
            let r = unsafe { vminvq_s8(transmute!(i8x16::new(
            1, 2, 3, 4,
            -16, 6, 7, 5,
            8, 1, 1, 1,
            1, 1, 1, 1,
        ))) };
    assert_eq!(r, -16_i8);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminv_s16() {
    let r = unsafe { vminv_s16(transmute!(i16x4::new(1, 2, -4, 3))) };
    assert_eq!(r, -4_i16);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminvq_s16() {
    let r = unsafe { vminvq_s16(transmute!(i16x8::new(1, 2, 7, 4, -16, 6, 7, 5))) };
    assert_eq!(r, -16_i16);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminv_s32() {
    let r = unsafe { vminv_s32(transmute!(i32x2::new(1, -4))) };
    assert_eq!(r, -4_i32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminvq_s32() {
    let r = unsafe { vminvq_s32(transmute!(i32x4::new(1, 2, -32, 4))) };
    assert_eq!(r, -32_i32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminv_u8() {
    let r = unsafe { vminv_u8(transmute!(u8x8::new(1, 2, 3, 4, 8, 6, 7, 5))) };
    assert_eq!(r, 1_u8);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminvq_u8() {
    #[rustfmt::skip]
            let r = unsafe { vminvq_u8(transmute!(u8x16::new(
            1, 2, 3, 4,
            16, 6, 7, 5,
            8, 1, 1, 1,
            1, 1, 1, 1,
        ))) };
    assert_eq!(r, 1_u8);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminv_u16() {
    let r = unsafe { vminv_u16(transmute!(u16x4::new(1, 2, 4, 3))) };
    assert_eq!(r, 1_u16);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminvq_u16() {
    let r = unsafe { vminvq_u16(transmute!(u16x8::new(1, 2, 7, 4, 16, 6, 7, 5))) };
    assert_eq!(r, 1_u16);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminv_u32() {
    let r = unsafe { vminv_u32(transmute!(u32x2::new(1, 4))) };
    assert_eq!(r, 1_u32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminvq_u32() {
    let r = unsafe { vminvq_u32(transmute!(u32x4::new(1, 2, 32, 4))) };
    assert_eq!(r, 1_u32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminv_f32() {
    let r = unsafe { vminv_f32(transmute!(f32x2::new(1., 4.))) };
    assert_eq!(r, 1_f32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminvq_f32() {
    let r = unsafe { vminvq_f32(transmute!(f32x4::new(1., 2., 32., 4.))) };
    assert_eq!(r, 1_f32);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vminvq_f64() {
    let r = unsafe { vminvq_f64(transmute!(f64x2::new(1., 4.))) };
    assert_eq!(r, 1_f64);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpminq_s8() {
    #[cfg_attr(rustfmt, skip)]
    let a = i8x16::new(1, -2, 3, -4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8);
    #[cfg_attr(rustfmt, skip)]
    let b = i8x16::new(0, 3, 2, 5, 4, 7, 6, 9, 0, 3, 2, 5, 4, 7, 6, 9);
    #[cfg_attr(rustfmt, skip)]
    let e = i8x16::new(-2, -4, 5, 7, 1, 3, 5, 7, 0, 2, 4, 6, 0, 2, 4, 6);
    let r: i8x16 = transmute!(vpminq_s8(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpminq_s16() {
    let a = i16x8::new(1, -2, 3, 4, 5, 6, 7, 8);
    let b = i16x8::new(0, 3, 2, 5, 4, 7, 6, 9);
    let e = i16x8::new(-2, 3, 5, 7, 0, 2, 4, 6);
    let r: i16x8 = transmute!(vpminq_s16(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpminq_s32() {
    let a = i32x4::new(1, -2, 3, 4);
    let b = i32x4::new(0, 3, 2, 5);
    let e = i32x4::new(-2, 3, 0, 2);
    let r: i32x4 = transmute!(vpminq_s32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpminq_u8() {
    #[cfg_attr(rustfmt, skip)]
    let a = u8x16::new(1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8);
    #[cfg_attr(rustfmt, skip)]
    let b = u8x16::new(0, 3, 2, 5, 4, 7, 6, 9, 0, 3, 2, 5, 4, 7, 6, 9);
    #[cfg_attr(rustfmt, skip)]
    let e = u8x16::new(1, 3, 5, 7, 1, 3, 5, 7, 0, 2, 4, 6, 0, 2, 4, 6);
    let r: u8x16 = transmute!(vpminq_u8(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpminq_u16() {
    let a = u16x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let b = u16x8::new(0, 3, 2, 5, 4, 7, 6, 9);
    let e = u16x8::new(1, 3, 5, 7, 0, 2, 4, 6);
    let r: u16x8 = transmute!(vpminq_u16(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpminq_u32() {
    let a = u32x4::new(1, 2, 3, 4);
    let b = u32x4::new(0, 3, 2, 5);
    let e = u32x4::new(1, 3, 0, 2);
    let r: u32x4 = transmute!(vpminq_u32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmin_f32() {
    let a = f32x4::new(1., -2., 3., 4.);
    let b = f32x4::new(0., 3., 2., 5.);
    let e = f32x4::new(-2., 3., 0., 2.);
    let r: f32x4 = transmute!(vpminq_f32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmin_f64() {
    let a = f64x2::new(1., -2.);
    let b = f64x2::new(0., 3.);
    let e = f64x2::new(-2., 0.);
    let r: f64x2 = transmute!(vpminq_f64(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmaxq_s8() {
    #[cfg_attr(rustfmt, skip)]
    let a = i8x16::new(1, -2, 3, -4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8);
    #[cfg_attr(rustfmt, skip)]
    let b = i8x16::new(0, 3, 2, 5, 4, 7, 6, 9, 0, 3, 2, 5, 4, 7, 6, 9);
    #[cfg_attr(rustfmt, skip)]
    let e = i8x16::new(1, 3, 6, 8, 2, 4, 6, 8, 3, 5, 7, 9, 3, 5, 7, 9);
    let r: i8x16 = transmute!(vpmaxq_s8(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmaxq_s16() {
    let a = i16x8::new(1, -2, 3, 4, 5, 6, 7, 8);
    let b = i16x8::new(0, 3, 2, 5, 4, 7, 6, 9);
    let e = i16x8::new(1, 4, 6, 8, 3, 5, 7, 9);
    let r: i16x8 = transmute!(vpmaxq_s16(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmaxq_s32() {
    let a = i32x4::new(1, -2, 3, 4);
    let b = i32x4::new(0, 3, 2, 5);
    let e = i32x4::new(1, 4, 3, 5);
    let r: i32x4 = transmute!(vpmaxq_s32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmaxq_u8() {
    #[cfg_attr(rustfmt, skip)]
    let a = u8x16::new(1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8);
    #[cfg_attr(rustfmt, skip)]
    let b = u8x16::new(0, 3, 2, 5, 4, 7, 6, 9, 0, 3, 2, 5, 4, 7, 6, 9);
    #[cfg_attr(rustfmt, skip)]
    let e = u8x16::new(2, 4, 6, 8, 2, 4, 6, 8, 3, 5, 7, 9, 3, 5, 7, 9);
    let r: u8x16 = transmute!(vpmaxq_u8(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmaxq_u16() {
    let a = u16x8::new(1, 2, 3, 4, 5, 6, 7, 8);
    let b = u16x8::new(0, 3, 2, 5, 4, 7, 6, 9);
    let e = u16x8::new(2, 4, 6, 8, 3, 5, 7, 9);
    let r: u16x8 = transmute!(vpmaxq_u16(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmaxq_u32() {
    let a = u32x4::new(1, 2, 3, 4);
    let b = u32x4::new(0, 3, 2, 5);
    let e = u32x4::new(2, 4, 3, 5);
    let r: u32x4 = transmute!(vpmaxq_u32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmax_f32() {
    let a = f32x4::new(1., -2., 3., 4.);
    let b = f32x4::new(0., 3., 2., 5.);
    let e = f32x4::new(1., 4., 3., 5.);
    let r: f32x4 = transmute!(vpmaxq_f32(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpmax_f64() {
    let a = f64x2::new(1., -2.);
    let b = f64x2::new(0., 3.);
    let e = f64x2::new(1., 3.);
    let r: f64x2 = transmute!(vpmaxq_f64(transmute!(a), transmute!(b)));
    assert_eq!(r, e);
}

macro_rules! test_vcombine {
        ($test_id:ident => $fn_id:ident ([$($a:expr),*], [$($b:expr),*])) => {
            #[allow(unused_assignments)]
            #[test]  //FIXME: #[simd_test(enable = "neon")]
            fn $test_id() {
                unsafe {
                    let a = [$($a),*];
                    let b = [$($b),*];
                    let e = [$($a),* $(, $b)*];
                    let c = $fn_id(transmute!(a), transmute!(b));
                    let mut d = e;
                    d = transmute!(c);
                    assert_eq!(d, e);
                }
            }
        }
    }

test_vcombine!(test_vcombine_s8 => vcombine_s8([3_i8, -4, 5, -6, 7, 8, 9, 10], [13_i8, -14, 15, -16, 17, 18, 19, 110]));
test_vcombine!(test_vcombine_u8 => vcombine_u8([3_u8, 4, 5, 6, 7, 8, 9, 10], [13_u8, 14, 15, 16, 17, 18, 19, 110]));
test_vcombine!(test_vcombine_p8 => vcombine_p8([3_u8, 4, 5, 6, 7, 8, 9, 10], [13_u8, 14, 15, 16, 17, 18, 19, 110]));

test_vcombine!(test_vcombine_s16 => vcombine_s16([3_i16, -4, 5, -6], [13_i16, -14, 15, -16]));
test_vcombine!(test_vcombine_u16 => vcombine_u16([3_u16, 4, 5, 6], [13_u16, 14, 15, 16]));
test_vcombine!(test_vcombine_p16 => vcombine_p16([3_u16, 4, 5, 6], [13_u16, 14, 15, 16]));
// FIXME: 16-bit floats
// test_vcombine!(test_vcombine_f16 => vcombine_f16([3_f16, 4., 5., 6.],
// [13_f16, 14., 15., 16.]));

test_vcombine!(test_vcombine_s32 => vcombine_s32([3_i32, -4], [13_i32, -14]));
test_vcombine!(test_vcombine_u32 => vcombine_u32([3_u32, 4], [13_u32, 14]));
// note: poly32x4 does not exist, and neither does vcombine_p32
test_vcombine!(test_vcombine_f32 => vcombine_f32([3_f32, -4.], [13_f32, -14.]));

test_vcombine!(test_vcombine_s64 => vcombine_s64([-3_i64], [13_i64]));
test_vcombine!(test_vcombine_u64 => vcombine_u64([3_u64], [13_u64]));
test_vcombine!(test_vcombine_p64 => vcombine_p64([3_u64], [13_u64]));
test_vcombine!(test_vcombine_f64 => vcombine_f64([-3_f64], [13_f64]));

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_u64() {
    test_cmp_u64(
        |i, j| unsafe { vceq_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a == b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_u64() {
    testq_cmp_u64(
        |i, j| unsafe { vceqq_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a == b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_s64() {
    test_cmp_s64(
        |i, j| unsafe { vceq_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a == b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_s64() {
    testq_cmp_s64(
        |i, j| unsafe { vceqq_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a == b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_p64() {
    test_cmp_p64(
        |i, j| unsafe { vceq_p64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a == b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_p64() {
    testq_cmp_p64(
        |i, j| unsafe { vceqq_p64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a == b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceq_f64() {
    test_cmp_f64(
        |i, j| unsafe { vceq_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a == b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vceqq_f64() {
    testq_cmp_f64(
        |i, j| unsafe { vceqq_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a == b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_s64() {
    test_cmp_s64(
        |i, j| unsafe { vcgt_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a > b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_s64() {
    testq_cmp_s64(
        |i, j| unsafe { vcgtq_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a > b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_u64() {
    test_cmp_u64(
        |i, j| unsafe { vcgt_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a > b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_u64() {
    testq_cmp_u64(
        |i, j| unsafe { vcgtq_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a > b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgt_f64() {
    test_cmp_f64(
        |i, j| unsafe { vcgt_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a > b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgtq_f64() {
    testq_cmp_f64(
        |i, j| unsafe { vcgtq_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a > b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vclt_s64() {
    test_cmp_s64(
        |i, j| unsafe { vclt_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a < b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_s64() {
    testq_cmp_s64(
        |i, j| unsafe { vcltq_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a < b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vclt_u64() {
    test_cmp_u64(
        |i, j| unsafe { vclt_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a < b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_u64() {
    testq_cmp_u64(
        |i, j| unsafe { vcltq_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a < b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vltq_f64() {
    test_cmp_f64(
        |i, j| unsafe { vclt_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a < b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcltq_f64() {
    testq_cmp_f64(
        |i, j| unsafe { vcltq_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a < b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcle_s64() {
    test_cmp_s64(
        |i, j| unsafe { vcle_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a <= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_s64() {
    testq_cmp_s64(
        |i, j| unsafe { vcleq_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a <= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcle_u64() {
    test_cmp_u64(
        |i, j| unsafe { vcle_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a <= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_u64() {
    testq_cmp_u64(
        |i, j| unsafe { vcleq_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a <= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vleq_f64() {
    test_cmp_f64(
        |i, j| unsafe { vcle_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a <= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcleq_f64() {
    testq_cmp_f64(
        |i, j| unsafe { vcleq_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a <= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcge_s64() {
    test_cmp_s64(
        |i, j| unsafe { vcge_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a >= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_s64() {
    testq_cmp_s64(
        |i, j| unsafe { vcgeq_s64(i, j) },
        |a: i64, b: i64| -> u64 {
            if a >= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcge_u64() {
    test_cmp_u64(
        |i, j| unsafe { vcge_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a >= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_u64() {
    testq_cmp_u64(
        |i, j| unsafe { vcgeq_u64(i, j) },
        |a: u64, b: u64| -> u64 {
            if a >= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vgeq_f64() {
    test_cmp_f64(
        |i, j| unsafe { vcge_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a >= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vcgeq_f64() {
    testq_cmp_f64(
        |i, j| unsafe { vcgeq_f64(i, j) },
        |a: f64, b: f64| -> u64 {
            if a >= b {
                0xFFFFFFFFFFFFFFFF
            } else {
                0
            }
        },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmul_f64() {
    test_ari_f64(
        |i, j| unsafe { vmul_f64(i, j) },
        |a: f64, b: f64| -> f64 { a * b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vmulq_f64() {
    testq_ari_f64(
        |i, j| unsafe { vmulq_f64(i, j) },
        |a: f64, b: f64| -> f64 { a * b },
    );
}

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsub_f64() {
    test_ari_f64(
        |i, j| unsafe { vsub_f64(i, j) },
        |a: f64, b: f64| -> f64 { a - b },
    );
}
#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vsubq_f64() {
    testq_ari_f64(
        |i, j| unsafe { vsubq_f64(i, j) },
        |a: f64, b: f64| -> f64 { a - b },
    );
}
