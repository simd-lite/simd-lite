mod generated;
use crate::simd_llvm::*;
use crate::NeonInit;
pub use generated::*;
#[cfg(target_arch = "aarch64")]
pub use std::arch::aarch64::*;
#[cfg(target_arch = "arm")]
pub use std::arch::arm::*;
use std::hint::unreachable_unchecked;
use std::mem::transmute;
use std::ptr;

#[cfg(test)]
use assert_instr_macro::assert_instr;

#[allow(improper_ctypes)]
extern "C" {
    //uint32x2_t vqmovn_u64 (uint64x2_t a)
    #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqmovnu.v2i32")]
    #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqxtn.v2i32")]
    fn vqmovn_u64_(a: uint64x2_t) -> uint32x2_t;
}

/// Unsigned saturating extract narrow.
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vqmovnu))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqxtn))]
pub unsafe fn vqmovn_u64(a: uint64x2_t) -> uint32x2_t {
    vqmovn_u64_(a)
}

/// Move vector element to general-purpose register
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[rustc_args_required_const(1)]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mov, imm5 = 1))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mov, imm5 = 1))]
// Based on the discussioj in https://github.com/rust-lang/stdarch/pull/792
// `mov` seems to be an acceptable intrinsic to compile to
// #[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(vmov, imm5 = 1))]
pub unsafe fn vgetq_lane_u64(v: uint64x2_t, imm5: i32) -> u64 {
    if (imm5) < 0 || (imm5) > 1 {
        unreachable_unchecked()
    }
    let imm5 = (imm5 & 0b1) as u32;
    simd_extract(v, imm5)
}

/// Move vector element to general-purpose register
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[rustc_args_required_const(1)]
#[cfg_attr(test, assert_instr(fmov, imm5 = 0))]
// gcc also turns this into a fmov instead of a umove
// https://clang.godbolt.org/z/J5xS2T
// #[cfg_attr(test, assert_instr(umov, imm5 = 0))]
pub unsafe fn vget_lane_u64(v: uint64x1_t, imm5: i32) -> u64 {
    if imm5 != 0 {
        unreachable_unchecked()
    }
    simd_extract(v, 0)
}

/// Move vector element to general-purpose register
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[rustc_args_required_const(1)]
#[cfg_attr(test, assert_instr(umov, imm5 = 0))]
pub unsafe fn vgetq_lane_u16(v: uint16x8_t, imm5: i32) -> u16 {
    if (imm5) < 0 || (imm5) > 7 {
        unreachable_unchecked()
    }
    let imm5 = (imm5 & 0b111) as u32;
    simd_extract(v, imm5)
}

/// Move vector element to general-purpose register
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[rustc_args_required_const(1)]
// see: https://clang.godbolt.org/z/J5xS2T
//#[cfg_attr(test, assert_instr(umov, imm5 = 0))]
#[cfg_attr(test, assert_instr(fmov, imm5 = 0))]
pub unsafe fn vgetq_lane_u32(v: uint32x4_t, imm5: i32) -> u32 {
    if (imm5) < 0 || (imm5) > 3 {
        unreachable_unchecked()
    }
    let imm5 = (imm5 & 0b11) as u32;
    simd_extract(v, imm5)
}

/// Move vector element to general-purpose register
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[rustc_args_required_const(1)]
#[cfg_attr(test, assert_instr(umov, imm5 = 0))]
pub unsafe fn vget_lane_u8(v: uint8x8_t, imm5: i32) -> u8 {
    if (imm5) < 0 || (imm5) > 7 {
        unreachable_unchecked()
    }
    let imm5 = (imm5 & 7) as u32;
    simd_extract(v, imm5)
}

/// Duplicate vector element to vector or scalar
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(dup))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(dup))]
pub unsafe fn vdupq_n_s8(value: i8) -> int8x16_t {
    transmute([
        value, value, value, value, value, value, value, value, value, value, value, value, value,
        value, value, value,
    ])
}

/// Duplicate vector element to vector or scalar
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(dup))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(dup))]
pub unsafe fn vdupq_n_u8(value: u8) -> uint8x16_t {
    transmute([
        value, value, value, value, value, value, value, value, value, value, value, value, value,
        value, value, value,
    ])
}

/// Duplicate vector element to vector or scalar
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(dup))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(dup))]
pub unsafe fn vmovq_n_u8(value: u8) -> uint8x16_t {
    vdupq_n_u8(value)
}

/// Duplicate vector element to vector or scalar
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(dup))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(dup))]
pub unsafe fn vmovq_n_s8(value: i8) -> int8x16_t {
    vdupq_n_s8(value)
}

/// Vector reinterpret cast operation
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(nop))]
pub unsafe fn vreinterpret_u64_u32(a: uint32x2_t) -> uint64x1_t {
    transmute(a)
}

/// Vector reinterpret cast operation
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(nop))]
pub unsafe fn vreinterpretq_s8_u8(a: uint8x16_t) -> int8x16_t {
    transmute(a)
}

/// Vector reinterpret cast operation
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(nop))]
pub unsafe fn vreinterpretq_u16_u8(a: uint8x16_t) -> uint16x8_t {
    transmute(a)
}

/// Vector reinterpret cast operation
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(nop))]
pub unsafe fn vreinterpretq_u32_u8(a: uint8x16_t) -> uint32x4_t {
    transmute(a)
}

/// Vector reinterpret cast operation
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(nop))]
pub unsafe fn vreinterpretq_u64_u8(a: uint8x16_t) -> uint64x2_t {
    transmute(a)
}

/// Vector reinterpret cast operation
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(nop))]
pub unsafe fn vreinterpretq_u8_s8(a: int8x16_t) -> uint8x16_t {
    transmute(a)
}

/// Unsigned shift right
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(ushr, imm3 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn vshrq_n_u8(a: uint8x16_t, imm3: i32) -> uint8x16_t {
    if imm3 < 0 || imm3 > 7 {
        unreachable_unchecked();
    } else {
        let imm3 = (imm3 & 0b111) as u8;
        simd_shr(
            a,
            transmute([
                imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3,
                imm3, imm3,
            ]),
        )
    }
}

/// Shift right
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(shl, imm3 = 1))]
#[rustc_args_required_const(1)]
pub unsafe fn vshlq_n_u8(a: uint8x16_t, imm3: i32) -> uint8x16_t {
    if imm3 < 0 || imm3 > 7 {
        unreachable_unchecked();
    } else {
        let imm3 = (imm3 & 0b111) as u8;
        simd_shl(
            a,
            transmute([
                imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3, imm3,
                imm3, imm3,
            ]),
        )
    }
}

/// Extract vector from pair of vectors
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(ext, n = 3))]
#[rustc_args_required_const(2)]
pub unsafe fn vextq_s8(a: int8x16_t, b: int8x16_t, n: i32) -> int8x16_t {
    if n < 0 || n > 15 {
        unreachable_unchecked();
    };
    match n & 0b1111 {
        0 => simd_shuffle16(a, b, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        1 => simd_shuffle16(
            a,
            b,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        ),
        2 => simd_shuffle16(
            a,
            b,
            [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
        ),
        3 => simd_shuffle16(
            a,
            b,
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        ),
        4 => simd_shuffle16(
            a,
            b,
            [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        ),
        5 => simd_shuffle16(
            a,
            b,
            [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
        ),
        6 => simd_shuffle16(
            a,
            b,
            [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21],
        ),
        7 => simd_shuffle16(
            a,
            b,
            [7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22],
        ),
        8 => simd_shuffle16(
            a,
            b,
            [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23],
        ),
        9 => simd_shuffle16(
            a,
            b,
            [
                9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            ],
        ),
        10 => simd_shuffle16(
            a,
            b,
            [
                10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            ],
        ),
        11 => simd_shuffle16(
            a,
            b,
            [
                11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
            ],
        ),
        12 => simd_shuffle16(
            a,
            b,
            [
                12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
            ],
        ),
        13 => simd_shuffle16(
            a,
            b,
            [
                13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            ],
        ),
        14 => simd_shuffle16(
            a,
            b,
            [
                14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            ],
        ),
        15 => simd_shuffle16(
            a,
            b,
            [
                15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            ],
        ),
        _ => unreachable_unchecked(),
    }
}

/// Extract vector from pair of vectors
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(ext, n = 3))]
#[rustc_args_required_const(2)]
pub unsafe fn vextq_u8(a: uint8x16_t, b: uint8x16_t, n: i32) -> uint8x16_t {
    if n < 0 || n > 15 {
        unreachable_unchecked();
    };
    match n & 0b1111 {
        0 => simd_shuffle16(a, b, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        1 => simd_shuffle16(
            a,
            b,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        ),
        2 => simd_shuffle16(
            a,
            b,
            [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
        ),
        3 => simd_shuffle16(
            a,
            b,
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        ),
        4 => simd_shuffle16(
            a,
            b,
            [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        ),
        5 => simd_shuffle16(
            a,
            b,
            [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
        ),
        6 => simd_shuffle16(
            a,
            b,
            [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21],
        ),
        7 => simd_shuffle16(
            a,
            b,
            [7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22],
        ),
        8 => simd_shuffle16(
            a,
            b,
            [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23],
        ),
        9 => simd_shuffle16(
            a,
            b,
            [
                9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            ],
        ),
        10 => simd_shuffle16(
            a,
            b,
            [
                10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            ],
        ),
        11 => simd_shuffle16(
            a,
            b,
            [
                11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
            ],
        ),
        12 => simd_shuffle16(
            a,
            b,
            [
                12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
            ],
        ),
        13 => simd_shuffle16(
            a,
            b,
            [
                13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            ],
        ),
        14 => simd_shuffle16(
            a,
            b,
            [
                14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            ],
        ),
        15 => simd_shuffle16(
            a,
            b,
            [
                15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            ],
        ),
        _ => unreachable_unchecked(),
    }
}

/// Load multiple single-element structures to one, two, three, or four registers
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(ldr))]
// even gcc compiles this to ldr: https://clang.godbolt.org/z/1bvH2x
// #[cfg_attr(test, assert_instr(ld1))]
pub unsafe fn vld1q_s8(addr: *const i8) -> int8x16_t {
    ptr::read(addr as *const int8x16_t)
}

impl NeonInit for int8x16_t {
    type From = [i8; 16];
    fn new(input: Self::From) -> Self {
        unsafe { vld1q_s8(&input as *const i8) }
    }
}

/// Load multiple single-element structures to one, two, three, or four registers
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(ldr))]
// even gcc compiles this to ldr: https://clang.godbolt.org/z/1bvH2x
// #[cfg_attr(test, assert_instr(ld1))]
pub unsafe fn vld1q_u8(addr: *const u8) -> uint8x16_t {
    ptr::read(addr as *const uint8x16_t)
}

impl NeonInit for uint8x16_t {
    type From = [u8; 16];
    fn new(input: Self::From) -> Self {
        unsafe { vld1q_u8(&input as *const u8) }
    }
}

/// Compare bitwise test bits nonzero
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(test, assert_instr(cmtst))]
// even gcc compiles this to ldr: https://clang.godbolt.org/z/1bvH2x
// #[cfg_attr(test, assert_instr(ld1))]
pub unsafe fn vtstq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    vcgtq_u8(vandq_u8(a, b), vdupq_n_u8(0))
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "aarch64")]
    use crate::aarch64::*;
    use crate::arm::*;
    use crate::cmparm::cmp_arm;
    use simd_test_macro::simd_test;
    #[cfg(target_arch = "aarch64")]
    use std::arch::aarch64::*;
    #[cfg(target_arch = "arm")]
    use std::arch::arm::*;
    use std::mem::transmute;

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vtstq_u8() {
        unsafe {
            let a: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let b: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let e: [u8; 16] = [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ];
            let r = vtstq_u8(transmute(a), transmute(b));
            assert!(cmp_arm(r, transmute(e)));
            let a: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let b: [u8; 16] = [0, 0, 1, 4, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let e: [u8; 16] = [
                0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF,
            ];
            let r = vtstq_u8(transmute(a), transmute(b));
            assert!(cmp_arm(r, transmute(e)));
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vqmovn_u64() {
        unsafe {
            let a: [u64; 2] = [1, 2];
            let e: [u32; 2] = [1, 2];
            let r = vqmovn_u64(transmute(a));
            assert!(cmp_arm(r, transmute(e)));
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vld1q_s8() {
        unsafe {
            let a: [i8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let e = transmute(a);
            let r = vld1q_s8(transmute(&a));
            assert!(cmp_arm(r, e));
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vld1q_u8() {
        unsafe {
            let a: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let e = transmute(a);
            let r = vld1q_u8(transmute(&a));
            assert!(cmp_arm(r, e));
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vget_lane_u8() {
        unsafe {
            let v: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
            let r = vget_lane_u8(transmute(v), 1);
            assert_eq!(r, 2);
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vgetq_lane_u32() {
        unsafe {
            let v: [u32; 4] = [1, 2, 3, 4];
            let r = vgetq_lane_u32(transmute(v), 1);
            assert_eq!(r, 2);
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vget_lane_u64() {
        unsafe {
            let v: [u64; 1] = [1];
            let r = vget_lane_u64(transmute(v), 0);
            assert_eq!(r, 1);
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vgetq_lane_u16() {
        unsafe {
            let v: [u16; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
            let r = vgetq_lane_u16(transmute(v), 1);
            assert_eq!(r, 2);
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vextq_s8() {
        unsafe {
            let a: [i8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let b: [i8; 16] = [
                17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 31, 32,
            ];
            let e: [i8; 16] = [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
            let r = vextq_s8(transmute(a), transmute(b), 3);
            assert!(cmp_arm(r, transmute(e)));
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vextq_u8() {
        unsafe {
            let a: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let b: [u8; 16] = [
                17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 31, 32,
            ];
            let e: [u8; 16] = [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
            let r = vextq_s8(transmute(a), transmute(b), 3);
            assert!(cmp_arm(r, transmute(e)));
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vshrq_n_u8() {
        unsafe {
            let a: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let e: [u8; 16] = [0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4];
            let r = vshrq_n_u8(transmute(a), 2);
            assert!(cmp_arm(r, transmute(e)));
        }
    }

    //#[simd_test(enable = "neon")]
    #[test]
    fn test_vshlq_n_u8() {
        unsafe {
            let a: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let e: [u8; 16] = [4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64];
            let r = vshlq_n_u8(transmute(a), 2);
            assert!(cmp_arm(r, transmute(e)));
        }
    }
}

#[cfg(test)]
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
