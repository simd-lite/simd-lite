#[cfg(target_arch = "aarch64")]
use crate::aarch64::*;
use crate::arm::*;
use crate::simd_llvm::*;
#[cfg(test)]
use assert_instr_macro::assert_instr;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vand_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vandq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vand_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vandq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vand_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vandq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vand_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_and(a, b)
}


/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vand_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vandq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vand_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vand_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vandq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vand_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    simd_and(a, b)
}

/// Vector bitwise and
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vand))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(and))]
pub unsafe fn vandq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    simd_and(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorr_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorrq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorr_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorrq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorr_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorrq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorr_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorrq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorr_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorrq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorr_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorrq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorr_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorrq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorr_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    simd_or(a, b)
}

/// Vector bitwise or (immediate, inclusive)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(vorr))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(orr))]
pub unsafe fn vorrq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    simd_or(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veor_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veorq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veor_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veorq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veor_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veorq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veor_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veorq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veor_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veorq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veor_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veorq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veor_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veorq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veor_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    simd_xor(a, b)
}

/// Vector bitwise exclusive or (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(veor))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(eor))]
pub unsafe fn veorq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    simd_xor(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceq_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceqq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceq_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceqq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceq_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceqq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceq_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceqq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceq_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceqq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceq_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    simd_eq(a, b)
}

/// Compare bitwise Equal (vector)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmeq))]
pub unsafe fn vceqq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    simd_eq(a, b)
}

/// Floating-point compare equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmeq))]
pub unsafe fn vceq_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    simd_eq(a, b)
}

/// Floating-point compare equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmeq))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmeq))]
pub unsafe fn vceqq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    simd_eq(a, b)
}

/// Compare signed greater than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vcgt_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    simd_gt(a, b)
}

/// Compare signed greater than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vcgtq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    simd_gt(a, b)
}

/// Compare signed greater than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vcgt_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    simd_gt(a, b)
}

/// Compare signed greater than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vcgtq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    simd_gt(a, b)
}

/// Compare signed greater than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vcgt_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    simd_gt(a, b)
}

/// Compare signed greater than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vcgtq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    simd_gt(a, b)
}

/// Compare unsigned highe
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vcgt_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_gt(a, b)
}

/// Compare unsigned highe
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vcgt_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_gt(a, b)
}

/// Compare unsigned highe
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vcgtq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_gt(a, b)
}

/// Compare unsigned highe
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vcgt_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_gt(a, b)
}

/// Compare unsigned highe
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vcgtq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    simd_gt(a, b)
}

/// Floating-point compare greater than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmgt))]
pub unsafe fn vcgt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    simd_gt(a, b)
}

/// Floating-point compare greater than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmgt))]
pub unsafe fn vcgtq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    simd_gt(a, b)
}

/// Compare signed less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vclt_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    simd_lt(a, b)
}

/// Compare signed less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vcltq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    simd_lt(a, b)
}

/// Compare signed less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vclt_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    simd_lt(a, b)
}

/// Compare signed less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vcltq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    simd_lt(a, b)
}

/// Compare signed less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vclt_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    simd_lt(a, b)
}

/// Compare signed less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmgt))]
pub unsafe fn vcltq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    simd_lt(a, b)
}

/// Compare unsigned less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vclt_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_lt(a, b)
}

/// Compare unsigned less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vcltq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    simd_lt(a, b)
}

/// Compare unsigned less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vclt_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_lt(a, b)
}

/// Compare unsigned less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vcltq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_lt(a, b)
}

/// Compare unsigned less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vclt_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_lt(a, b)
}

/// Compare unsigned less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhi))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhi))]
pub unsafe fn vcltq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    simd_lt(a, b)
}

/// Floating-point compare less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmgt))]
pub unsafe fn vclt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    simd_lt(a, b)
}

/// Floating-point compare less than
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmgt))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmgt))]
pub unsafe fn vcltq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    simd_lt(a, b)
}

/// Compare signed less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcle_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    simd_le(a, b)
}

/// Compare signed less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcleq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    simd_le(a, b)
}

/// Compare signed less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcle_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    simd_le(a, b)
}

/// Compare signed less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcleq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    simd_le(a, b)
}

/// Compare signed less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcle_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    simd_le(a, b)
}

/// Compare signed less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcleq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    simd_le(a, b)
}

/// Compare unsigned less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcle_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_le(a, b)
}

/// Compare unsigned less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcleq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    simd_le(a, b)
}

/// Compare unsigned less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcle_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_le(a, b)
}

/// Compare unsigned less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcleq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_le(a, b)
}

/// Compare unsigned less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcle_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_le(a, b)
}

/// Compare unsigned less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcleq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    simd_le(a, b)
}

/// Floating-point compare less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmge))]
pub unsafe fn vcle_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    simd_le(a, b)
}

/// Floating-point compare less than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmge))]
pub unsafe fn vcleq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    simd_le(a, b)
}

/// Compare signed greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcge_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    simd_ge(a, b)
}

/// Compare signed greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcgeq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    simd_ge(a, b)
}

/// Compare signed greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcge_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    simd_ge(a, b)
}

/// Compare signed greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcgeq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    simd_ge(a, b)
}

/// Compare signed greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcge_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    simd_ge(a, b)
}

/// Compare signed greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmge))]
pub unsafe fn vcgeq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    simd_ge(a, b)
}

/// Compare unsigned greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcge_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_ge(a, b)
}

/// Compare unsigned greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcgeq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    simd_ge(a, b)
}

/// Compare unsigned greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcge_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_ge(a, b)
}

/// Compare unsigned greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcgeq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_ge(a, b)
}

/// Compare unsigned greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcge_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_ge(a, b)
}

/// Compare unsigned greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(cmhs))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(cmhs))]
pub unsafe fn vcgeq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    simd_ge(a, b)
}

/// Floating-point compare greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmge))]
pub unsafe fn vcge_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    simd_ge(a, b)
}

/// Floating-point compare greater than or equal
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(fcmge))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(fcmge))]
pub unsafe fn vcgeq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    simd_ge(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqsub))]
pub unsafe fn vqsub_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubu.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqsub.v8i8")]
        fn vqsub_u8_(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t;
    }
    vqsub_u8_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqsub))]
pub unsafe fn vqsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubu.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqsub.v16i8")]
        fn vqsubq_u8_(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
    }
    vqsubq_u8_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqsub))]
pub unsafe fn vqsub_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubu.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqsub.v4i16")]
        fn vqsub_u16_(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t;
    }
    vqsub_u16_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqsub))]
pub unsafe fn vqsubq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubu.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqsub.v8i16")]
        fn vqsubq_u16_(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t;
    }
    vqsubq_u16_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqsub))]
pub unsafe fn vqsub_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubu.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqsub.v2i32")]
        fn vqsub_u32_(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t;
    }
    vqsub_u32_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqsub))]
pub unsafe fn vqsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubu.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqsub.v4i32")]
        fn vqsubq_u32_(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
    }
    vqsubq_u32_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqsub))]
pub unsafe fn vqsub_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubs.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqsub.v8i8")]
        fn vqsub_s8_(a: int8x8_t, b: int8x8_t) -> int8x8_t;
    }
    vqsub_s8_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqsub))]
pub unsafe fn vqsubq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubs.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqsub.v16i8")]
        fn vqsubq_s8_(a: int8x16_t, b: int8x16_t) -> int8x16_t;
    }
    vqsubq_s8_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqsub))]
pub unsafe fn vqsub_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubs.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqsub.v4i16")]
        fn vqsub_s16_(a: int16x4_t, b: int16x4_t) -> int16x4_t;
    }
    vqsub_s16_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqsub))]
pub unsafe fn vqsubq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubs.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqsub.v8i16")]
        fn vqsubq_s16_(a: int16x8_t, b: int16x8_t) -> int16x8_t;
    }
    vqsubq_s16_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqsub))]
pub unsafe fn vqsub_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubs.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqsub.v2i32")]
        fn vqsub_s32_(a: int32x2_t, b: int32x2_t) -> int32x2_t;
    }
    vqsub_s32_(a, b)
}

/// Saturating subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqsub))]
pub unsafe fn vqsubq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqsubs.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqsub.v4i32")]
        fn vqsubq_s32_(a: int32x4_t, b: int32x4_t) -> int32x4_t;
    }
    vqsubq_s32_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhadd))]
pub unsafe fn vhadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhaddu.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhadd.v8i8")]
        fn vhadd_u8_(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t;
    }
    vhadd_u8_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhadd))]
pub unsafe fn vhaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhaddu.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhadd.v16i8")]
        fn vhaddq_u8_(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
    }
    vhaddq_u8_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhadd))]
pub unsafe fn vhadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhaddu.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhadd.v4i16")]
        fn vhadd_u16_(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t;
    }
    vhadd_u16_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhadd))]
pub unsafe fn vhaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhaddu.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhadd.v8i16")]
        fn vhaddq_u16_(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t;
    }
    vhaddq_u16_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhadd))]
pub unsafe fn vhadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhaddu.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhadd.v2i32")]
        fn vhadd_u32_(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t;
    }
    vhadd_u32_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhadd))]
pub unsafe fn vhaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhaddu.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhadd.v4i32")]
        fn vhaddq_u32_(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
    }
    vhaddq_u32_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shadd))]
pub unsafe fn vhadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhadds.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shadd.v8i8")]
        fn vhadd_s8_(a: int8x8_t, b: int8x8_t) -> int8x8_t;
    }
    vhadd_s8_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shadd))]
pub unsafe fn vhaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhadds.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shadd.v16i8")]
        fn vhaddq_s8_(a: int8x16_t, b: int8x16_t) -> int8x16_t;
    }
    vhaddq_s8_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shadd))]
pub unsafe fn vhadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhadds.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shadd.v4i16")]
        fn vhadd_s16_(a: int16x4_t, b: int16x4_t) -> int16x4_t;
    }
    vhadd_s16_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shadd))]
pub unsafe fn vhaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhadds.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shadd.v8i16")]
        fn vhaddq_s16_(a: int16x8_t, b: int16x8_t) -> int16x8_t;
    }
    vhaddq_s16_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shadd))]
pub unsafe fn vhadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhadds.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shadd.v2i32")]
        fn vhadd_s32_(a: int32x2_t, b: int32x2_t) -> int32x2_t;
    }
    vhadd_s32_(a, b)
}

/// Halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shadd))]
pub unsafe fn vhaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhadds.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shadd.v4i32")]
        fn vhaddq_s32_(a: int32x4_t, b: int32x4_t) -> int32x4_t;
    }
    vhaddq_s32_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(urhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(urhadd))]
pub unsafe fn vrhadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhaddu.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.urhadd.v8i8")]
        fn vrhadd_u8_(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t;
    }
    vrhadd_u8_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(urhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(urhadd))]
pub unsafe fn vrhaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhaddu.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.urhadd.v16i8")]
        fn vrhaddq_u8_(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
    }
    vrhaddq_u8_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(urhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(urhadd))]
pub unsafe fn vrhadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhaddu.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.urhadd.v4i16")]
        fn vrhadd_u16_(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t;
    }
    vrhadd_u16_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(urhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(urhadd))]
pub unsafe fn vrhaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhaddu.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.urhadd.v8i16")]
        fn vrhaddq_u16_(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t;
    }
    vrhaddq_u16_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(urhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(urhadd))]
pub unsafe fn vrhadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhaddu.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.urhadd.v2i32")]
        fn vrhadd_u32_(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t;
    }
    vrhadd_u32_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(urhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(urhadd))]
pub unsafe fn vrhaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhaddu.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.urhadd.v4i32")]
        fn vrhaddq_u32_(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
    }
    vrhaddq_u32_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(srhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(srhadd))]
pub unsafe fn vrhadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhadds.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.srhadd.v8i8")]
        fn vrhadd_s8_(a: int8x8_t, b: int8x8_t) -> int8x8_t;
    }
    vrhadd_s8_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(srhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(srhadd))]
pub unsafe fn vrhaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhadds.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.srhadd.v16i8")]
        fn vrhaddq_s8_(a: int8x16_t, b: int8x16_t) -> int8x16_t;
    }
    vrhaddq_s8_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(srhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(srhadd))]
pub unsafe fn vrhadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhadds.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.srhadd.v4i16")]
        fn vrhadd_s16_(a: int16x4_t, b: int16x4_t) -> int16x4_t;
    }
    vrhadd_s16_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(srhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(srhadd))]
pub unsafe fn vrhaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhadds.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.srhadd.v8i16")]
        fn vrhaddq_s16_(a: int16x8_t, b: int16x8_t) -> int16x8_t;
    }
    vrhaddq_s16_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(srhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(srhadd))]
pub unsafe fn vrhadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhadds.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.srhadd.v2i32")]
        fn vrhadd_s32_(a: int32x2_t, b: int32x2_t) -> int32x2_t;
    }
    vrhadd_s32_(a, b)
}

/// Rounding halving add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(srhadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(srhadd))]
pub unsafe fn vrhaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vrhadds.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.srhadd.v4i32")]
        fn vrhaddq_s32_(a: int32x4_t, b: int32x4_t) -> int32x4_t;
    }
    vrhaddq_s32_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqadd))]
pub unsafe fn vqadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqaddu.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqadd.v8i8")]
        fn vqadd_u8_(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t;
    }
    vqadd_u8_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqadd))]
pub unsafe fn vqaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqaddu.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqadd.v16i8")]
        fn vqaddq_u8_(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
    }
    vqaddq_u8_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqadd))]
pub unsafe fn vqadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqaddu.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqadd.v4i16")]
        fn vqadd_u16_(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t;
    }
    vqadd_u16_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqadd))]
pub unsafe fn vqaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqaddu.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqadd.v8i16")]
        fn vqaddq_u16_(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t;
    }
    vqaddq_u16_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqadd))]
pub unsafe fn vqadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqaddu.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqadd.v2i32")]
        fn vqadd_u32_(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t;
    }
    vqadd_u32_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uqadd))]
pub unsafe fn vqaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqaddu.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uqadd.v4i32")]
        fn vqaddq_u32_(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
    }
    vqaddq_u32_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqadd))]
pub unsafe fn vqadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqadds.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqadd.v8i8")]
        fn vqadd_s8_(a: int8x8_t, b: int8x8_t) -> int8x8_t;
    }
    vqadd_s8_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqadd))]
pub unsafe fn vqaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqadds.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqadd.v16i8")]
        fn vqaddq_s8_(a: int8x16_t, b: int8x16_t) -> int8x16_t;
    }
    vqaddq_s8_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqadd))]
pub unsafe fn vqadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqadds.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqadd.v4i16")]
        fn vqadd_s16_(a: int16x4_t, b: int16x4_t) -> int16x4_t;
    }
    vqadd_s16_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqadd))]
pub unsafe fn vqaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqadds.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqadd.v8i16")]
        fn vqaddq_s16_(a: int16x8_t, b: int16x8_t) -> int16x8_t;
    }
    vqaddq_s16_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqadd))]
pub unsafe fn vqadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqadds.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqadd.v2i32")]
        fn vqadd_s32_(a: int32x2_t, b: int32x2_t) -> int32x2_t;
    }
    vqadd_s32_(a, b)
}

/// Saturating add
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sqadd))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sqadd))]
pub unsafe fn vqaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vqadds.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqadd.v4i32")]
        fn vqaddq_s32_(a: int32x4_t, b: int32x4_t) -> int32x4_t;
    }
    vqaddq_s32_(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmul_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmulq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmul_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmulq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmul_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmulq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmul_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmulq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmul_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmulq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmul_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmulq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmul_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    simd_mul(a, b)
}

/// Multiply
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(mul))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(mul))]
pub unsafe fn vmulq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    simd_mul(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsub_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsubq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsub_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsubq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsub_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsubq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsub_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsub_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsubq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsub_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsub_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsubq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsub_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsubq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsub_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    simd_sub(a, b)
}

/// Subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(sub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(sub))]
pub unsafe fn vsubq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    simd_sub(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhsub))]
pub unsafe fn vhsub_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubu.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhsub.v8i8")]
        fn vhsub_u8_(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t;
    }
    vhsub_u8_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhsub))]
pub unsafe fn vhsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubu.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhsub.v16i8")]
        fn vhsubq_u8_(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
    }
    vhsubq_u8_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhsub))]
pub unsafe fn vhsub_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubu.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhsub.v4i16")]
        fn vhsub_u16_(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t;
    }
    vhsub_u16_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhsub))]
pub unsafe fn vhsubq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubu.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhsub.v8i16")]
        fn vhsubq_u16_(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t;
    }
    vhsubq_u16_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhsub))]
pub unsafe fn vhsub_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubu.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhsub.v2i32")]
        fn vhsub_u32_(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t;
    }
    vhsub_u32_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(uhsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(uhsub))]
pub unsafe fn vhsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubu.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.uhsub.v4i32")]
        fn vhsubq_u32_(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
    }
    vhsubq_u32_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shsub))]
pub unsafe fn vhsub_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubs.v8i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shsub.v8i8")]
        fn vhsub_s8_(a: int8x8_t, b: int8x8_t) -> int8x8_t;
    }
    vhsub_s8_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shsub))]
pub unsafe fn vhsubq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubs.v16i8")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shsub.v16i8")]
        fn vhsubq_s8_(a: int8x16_t, b: int8x16_t) -> int8x16_t;
    }
    vhsubq_s8_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shsub))]
pub unsafe fn vhsub_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubs.v4i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shsub.v4i16")]
        fn vhsub_s16_(a: int16x4_t, b: int16x4_t) -> int16x4_t;
    }
    vhsub_s16_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shsub))]
pub unsafe fn vhsubq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubs.v8i16")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shsub.v8i16")]
        fn vhsubq_s16_(a: int16x8_t, b: int16x8_t) -> int16x8_t;
    }
    vhsubq_s16_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shsub))]
pub unsafe fn vhsub_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubs.v2i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shsub.v2i32")]
        fn vhsub_s32_(a: int32x2_t, b: int32x2_t) -> int32x2_t;
    }
    vhsub_s32_(a, b)
}

/// Signed halving subtract
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
#[cfg_attr(all(test, target_arch = "arm"), assert_instr(shsub))]
#[cfg_attr(all(test, target_arch = "aarch64"), assert_instr(shsub))]
pub unsafe fn vhsubq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[cfg_attr(target_arch = "arm", link_name = "llvm.arm.neon.vhsubs.v4i32")]
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.shsub.v4i32")]
        fn vhsubq_s32_(a: int32x4_t, b: int32x4_t) -> int32x4_t;
    }
    vhsubq_s32_(a, b)
}

#[cfg(test)]
#[allow(overflowing_literals)]
mod test {
    #[cfg(target_arch = "aarch64")]
    use crate::aarch64::*;
    use crate::arm::cmp_arm;
    use crate::arm::*;
    use simd_test_macro::simd_test;
    #[cfg(target_arch = "aarch64")]
    use std::arch::aarch64::*;
    #[cfg(target_arch = "arm")]
    use std::arch::arm::*;
    use std::mem::transmute;

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vand_s8() {
        unsafe {
            let a: int8x8_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let b: int8x8_t = transmute([
                0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8,
            ]);
            let e: int8x8_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let r: int8x8_t = transmute(vand_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int8x8_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let b: int8x8_t = transmute([
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
            ]);
            let e: int8x8_t = transmute([
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
            ]);
            let r: int8x8_t = transmute(vand_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vandq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0x0Ci8, 0x0Di8, 0x0Ei8, 0x00i8,
            ]);
            let b: int8x16_t = transmute([
                0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8,
                0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8, 0x0Fi8,
            ]);
            let e: int8x16_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0x0Ci8, 0x0Di8, 0x0Ei8, 0x00i8,
            ]);
            let r: int8x16_t = transmute(vandq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int8x16_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0x0Ci8, 0x0Di8, 0x0Ei8, 0x00i8,
            ]);
            let b: int8x16_t = transmute([
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
            ]);
            let e: int8x16_t = transmute([
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
            ]);
            let r: int8x16_t = transmute(vandq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vand_s16() {
        unsafe {
            let a: int16x4_t = transmute([0x00i16, 0x01i16, 0x02i16, 0x03i16]);
            let b: int16x4_t = transmute([0x0Fi16, 0x0Fi16, 0x0Fi16, 0x0Fi16]);
            let e: int16x4_t = transmute([0x00i16, 0x01i16, 0x02i16, 0x03i16]);
            let r: int16x4_t = transmute(vand_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int16x4_t = transmute([0x00i16, 0x01i16, 0x02i16, 0x03i16]);
            let b: int16x4_t = transmute([0x00i16, 0x00i16, 0x00i16, 0x00i16]);
            let e: int16x4_t = transmute([0x00i16, 0x00i16, 0x00i16, 0x00i16]);
            let r: int16x4_t = transmute(vand_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vandq_s16() {
        unsafe {
            let a: int16x8_t = transmute([
                0x00i16, 0x01i16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let b: int16x8_t = transmute([
                0x0Fi16, 0x0Fi16, 0x0Fi16, 0x0Fi16, 0x0Fi16, 0x0Fi16, 0x0Fi16, 0x0Fi16,
            ]);
            let e: int16x8_t = transmute([
                0x00i16, 0x01i16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let r: int16x8_t = transmute(vandq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int16x8_t = transmute([
                0x00i16, 0x01i16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let b: int16x8_t = transmute([
                0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16,
            ]);
            let e: int16x8_t = transmute([
                0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16,
            ]);
            let r: int16x8_t = transmute(vandq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vand_s32() {
        unsafe {
            let a: int32x2_t = transmute([0x00i32, 0x01i32]);
            let b: int32x2_t = transmute([0x0Fi32, 0x0Fi32]);
            let e: int32x2_t = transmute([0x00i32, 0x01i32]);
            let r: int32x2_t = transmute(vand_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int32x2_t = transmute([0x00i32, 0x01i32]);
            let b: int32x2_t = transmute([0x00i32, 0x00i32]);
            let e: int32x2_t = transmute([0x00i32, 0x00i32]);
            let r: int32x2_t = transmute(vand_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vandq_s32() {
        unsafe {
            let a: int32x4_t = transmute([0x00i32, 0x01i32, 0x02i32, 0x03i32]);
            let b: int32x4_t = transmute([0x0Fi32, 0x0Fi32, 0x0Fi32, 0x0Fi32]);
            let e: int32x4_t = transmute([0x00i32, 0x01i32, 0x02i32, 0x03i32]);
            let r: int32x4_t = transmute(vandq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int32x4_t = transmute([0x00i32, 0x01i32, 0x02i32, 0x03i32]);
            let b: int32x4_t = transmute([0x00i32, 0x00i32, 0x00i32, 0x00i32]);
            let e: int32x4_t = transmute([0x00i32, 0x00i32, 0x00i32, 0x00i32]);
            let r: int32x4_t = transmute(vandq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vand_u8() {
        unsafe {
            let a: uint8x8_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let b: uint8x8_t = transmute([
                0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8,
            ]);
            let e: uint8x8_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let r: uint8x8_t = transmute(vand_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint8x8_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let b: uint8x8_t = transmute([
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            ]);
            let e: uint8x8_t = transmute([
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            ]);
            let r: uint8x8_t = transmute(vand_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vandq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0x00u8,
            ]);
            let b: uint8x16_t = transmute([
                0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8,
                0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8, 0x0Fu8,
            ]);
            let e: uint8x16_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0x00u8,
            ]);
            let r: uint8x16_t = transmute(vandq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint8x16_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0x00u8,
            ]);
            let b: uint8x16_t = transmute([
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            ]);
            let e: uint8x16_t = transmute([
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            ]);
            let r: uint8x16_t = transmute(vandq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vand_u16() {
        unsafe {
            let a: uint16x4_t = transmute([0x00u16, 0x01u16, 0x02u16, 0x03u16]);
            let b: uint16x4_t = transmute([0x0Fu16, 0x0Fu16, 0x0Fu16, 0x0Fu16]);
            let e: uint16x4_t = transmute([0x00u16, 0x01u16, 0x02u16, 0x03u16]);
            let r: uint16x4_t = transmute(vand_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint16x4_t = transmute([0x00u16, 0x01u16, 0x02u16, 0x03u16]);
            let b: uint16x4_t = transmute([0x00u16, 0x00u16, 0x00u16, 0x00u16]);
            let e: uint16x4_t = transmute([0x00u16, 0x00u16, 0x00u16, 0x00u16]);
            let r: uint16x4_t = transmute(vand_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vandq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([
                0x00u16, 0x01u16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let b: uint16x8_t = transmute([
                0x0Fu16, 0x0Fu16, 0x0Fu16, 0x0Fu16, 0x0Fu16, 0x0Fu16, 0x0Fu16, 0x0Fu16,
            ]);
            let e: uint16x8_t = transmute([
                0x00u16, 0x01u16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let r: uint16x8_t = transmute(vandq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint16x8_t = transmute([
                0x00u16, 0x01u16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let b: uint16x8_t = transmute([
                0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16,
            ]);
            let e: uint16x8_t = transmute([
                0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16,
            ]);
            let r: uint16x8_t = transmute(vandq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vand_u32() {
        unsafe {
            let a: uint32x2_t = transmute([0x00u32, 0x01u32]);
            let b: uint32x2_t = transmute([0x0Fu32, 0x0Fu32]);
            let e: uint32x2_t = transmute([0x00u32, 0x01u32]);
            let r: uint32x2_t = transmute(vand_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint32x2_t = transmute([0x00u32, 0x01u32]);
            let b: uint32x2_t = transmute([0x00u32, 0x00u32]);
            let e: uint32x2_t = transmute([0x00u32, 0x00u32]);
            let r: uint32x2_t = transmute(vand_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vandq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([0x00u32, 0x01u32, 0x02u32, 0x03u32]);
            let b: uint32x4_t = transmute([0x0Fu32, 0x0Fu32, 0x0Fu32, 0x0Fu32]);
            let e: uint32x4_t = transmute([0x00u32, 0x01u32, 0x02u32, 0x03u32]);
            let r: uint32x4_t = transmute(vandq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint32x4_t = transmute([0x00u32, 0x01u32, 0x02u32, 0x03u32]);
            let b: uint32x4_t = transmute([0x00u32, 0x00u32, 0x00u32, 0x00u32]);
            let e: uint32x4_t = transmute([0x00u32, 0x00u32, 0x00u32, 0x00u32]);
            let r: uint32x4_t = transmute(vandq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vand_s64() {
        unsafe {
            let a: int64x1_t = transmute([0x00i64]);
            let b: int64x1_t = transmute([0x0Fi64]);
            let e: int64x1_t = transmute([0x00i64]);
            let r: int64x1_t = transmute(vand_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int64x1_t = transmute([0x00i64]);
            let b: int64x1_t = transmute([0x00i64]);
            let e: int64x1_t = transmute([0x00i64]);
            let r: int64x1_t = transmute(vand_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vandq_s64() {
        unsafe {
            let a: int64x2_t = transmute([0x00i64, 0x01i64]);
            let b: int64x2_t = transmute([0x0Fi64, 0x0Fi64]);
            let e: int64x2_t = transmute([0x00i64, 0x01i64]);
            let r: int64x2_t = transmute(vandq_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int64x2_t = transmute([0x00i64, 0x01i64]);
            let b: int64x2_t = transmute([0x00i64, 0x00i64]);
            let e: int64x2_t = transmute([0x00i64, 0x00i64]);
            let r: int64x2_t = transmute(vandq_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vand_u64() {
        unsafe {
            let a: uint64x1_t = transmute([0x00u64]);
            let b: uint64x1_t = transmute([0x0Fu64]);
            let e: uint64x1_t = transmute([0x00u64]);
            let r: uint64x1_t = transmute(vand_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint64x1_t = transmute([0x00u64]);
            let b: uint64x1_t = transmute([0x00u64]);
            let e: uint64x1_t = transmute([0x00u64]);
            let r: uint64x1_t = transmute(vand_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vandq_u64() {
        unsafe {
            let a: uint64x2_t = transmute([0x00u64, 0x01u64]);
            let b: uint64x2_t = transmute([0x0Fu64, 0x0Fu64]);
            let e: uint64x2_t = transmute([0x00u64, 0x01u64]);
            let r: uint64x2_t = transmute(vandq_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint64x2_t = transmute([0x00u64, 0x01u64]);
            let b: uint64x2_t = transmute([0x00u64, 0x00u64]);
            let e: uint64x2_t = transmute([0x00u64, 0x00u64]);
            let r: uint64x2_t = transmute(vandq_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorr_s8() {
        unsafe {
            let a: int8x8_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let b: int8x8_t = transmute([
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
            ]);
            let e: int8x8_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let r: int8x8_t = transmute(vorr_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorrq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0x0Ci8, 0x0Di8, 0x0Ei8, 0x0Fi8,
            ]);
            let b: int8x16_t = transmute([
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
            ]);
            let e: int8x16_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0x0Ci8, 0x0Di8, 0x0Ei8, 0x0Fi8,
            ]);
            let r: int8x16_t = transmute(vorrq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorr_s16() {
        unsafe {
            let a: int16x4_t = transmute([0x00i16, 0x01i16, 0x02i16, 0x03i16]);
            let b: int16x4_t = transmute([0x00i16, 0x00i16, 0x00i16, 0x00i16]);
            let e: int16x4_t = transmute([0x00i16, 0x01i16, 0x02i16, 0x03i16]);
            let r: int16x4_t = transmute(vorr_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorrq_s16() {
        unsafe {
            let a: int16x8_t = transmute([
                0x00i16, 0x01i16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let b: int16x8_t = transmute([
                0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16,
            ]);
            let e: int16x8_t = transmute([
                0x00i16, 0x01i16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let r: int16x8_t = transmute(vorrq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorr_s32() {
        unsafe {
            let a: int32x2_t = transmute([0x00i32, 0x01i32]);
            let b: int32x2_t = transmute([0x00i32, 0x00i32]);
            let e: int32x2_t = transmute([0x00i32, 0x01i32]);
            let r: int32x2_t = transmute(vorr_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorrq_s32() {
        unsafe {
            let a: int32x4_t = transmute([0x00i32, 0x01i32, 0x02i32, 0x03i32]);
            let b: int32x4_t = transmute([0x00i32, 0x00i32, 0x00i32, 0x00i32]);
            let e: int32x4_t = transmute([0x00i32, 0x01i32, 0x02i32, 0x03i32]);
            let r: int32x4_t = transmute(vorrq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorr_u8() {
        unsafe {
            let a: uint8x8_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let b: uint8x8_t = transmute([
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            ]);
            let e: uint8x8_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let r: uint8x8_t = transmute(vorr_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorrq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0x0Fu8,
            ]);
            let b: uint8x16_t = transmute([
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            ]);
            let e: uint8x16_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0x0Fu8,
            ]);
            let r: uint8x16_t = transmute(vorrq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorr_u16() {
        unsafe {
            let a: uint16x4_t = transmute([0x00u16, 0x01u16, 0x02u16, 0x03u16]);
            let b: uint16x4_t = transmute([0x00u16, 0x00u16, 0x00u16, 0x00u16]);
            let e: uint16x4_t = transmute([0x00u16, 0x01u16, 0x02u16, 0x03u16]);
            let r: uint16x4_t = transmute(vorr_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorrq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([
                0x00u16, 0x01u16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let b: uint16x8_t = transmute([
                0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16,
            ]);
            let e: uint16x8_t = transmute([
                0x00u16, 0x01u16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let r: uint16x8_t = transmute(vorrq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorr_u32() {
        unsafe {
            let a: uint32x2_t = transmute([0x00u32, 0x01u32]);
            let b: uint32x2_t = transmute([0x00u32, 0x00u32]);
            let e: uint32x2_t = transmute([0x00u32, 0x01u32]);
            let r: uint32x2_t = transmute(vorr_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorrq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([0x00u32, 0x01u32, 0x02u32, 0x03u32]);
            let b: uint32x4_t = transmute([0x00u32, 0x00u32, 0x00u32, 0x00u32]);
            let e: uint32x4_t = transmute([0x00u32, 0x01u32, 0x02u32, 0x03u32]);
            let r: uint32x4_t = transmute(vorrq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorr_s64() {
        unsafe {
            let a: int64x1_t = transmute([0x00i64]);
            let b: int64x1_t = transmute([0x00i64]);
            let e: int64x1_t = transmute([0x00i64]);
            let r: int64x1_t = transmute(vorr_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorrq_s64() {
        unsafe {
            let a: int64x2_t = transmute([0x00i64, 0x01i64]);
            let b: int64x2_t = transmute([0x00i64, 0x00i64]);
            let e: int64x2_t = transmute([0x00i64, 0x01i64]);
            let r: int64x2_t = transmute(vorrq_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorr_u64() {
        unsafe {
            let a: uint64x1_t = transmute([0x00u64]);
            let b: uint64x1_t = transmute([0x00u64]);
            let e: uint64x1_t = transmute([0x00u64]);
            let r: uint64x1_t = transmute(vorr_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vorrq_u64() {
        unsafe {
            let a: uint64x2_t = transmute([0x00u64, 0x01u64]);
            let b: uint64x2_t = transmute([0x00u64, 0x00u64]);
            let e: uint64x2_t = transmute([0x00u64, 0x01u64]);
            let r: uint64x2_t = transmute(vorrq_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veor_s8() {
        unsafe {
            let a: int8x8_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let b: int8x8_t = transmute([
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
            ]);
            let e: int8x8_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let r: int8x8_t = transmute(veor_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veorq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0x0Ci8, 0x0Di8, 0x0Ei8, 0x0Fi8,
            ]);
            let b: int8x16_t = transmute([
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
                0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8, 0x00i8,
            ]);
            let e: int8x16_t = transmute([
                0x00i8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0x0Ci8, 0x0Di8, 0x0Ei8, 0x0Fi8,
            ]);
            let r: int8x16_t = transmute(veorq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veor_s16() {
        unsafe {
            let a: int16x4_t = transmute([0x00i16, 0x01i16, 0x02i16, 0x03i16]);
            let b: int16x4_t = transmute([0x00i16, 0x00i16, 0x00i16, 0x00i16]);
            let e: int16x4_t = transmute([0x00i16, 0x01i16, 0x02i16, 0x03i16]);
            let r: int16x4_t = transmute(veor_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veorq_s16() {
        unsafe {
            let a: int16x8_t = transmute([
                0x00i16, 0x01i16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let b: int16x8_t = transmute([
                0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16, 0x00i16,
            ]);
            let e: int16x8_t = transmute([
                0x00i16, 0x01i16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let r: int16x8_t = transmute(veorq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veor_s32() {
        unsafe {
            let a: int32x2_t = transmute([0x00i32, 0x01i32]);
            let b: int32x2_t = transmute([0x00i32, 0x00i32]);
            let e: int32x2_t = transmute([0x00i32, 0x01i32]);
            let r: int32x2_t = transmute(veor_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veorq_s32() {
        unsafe {
            let a: int32x4_t = transmute([0x00i32, 0x01i32, 0x02i32, 0x03i32]);
            let b: int32x4_t = transmute([0x00i32, 0x00i32, 0x00i32, 0x00i32]);
            let e: int32x4_t = transmute([0x00i32, 0x01i32, 0x02i32, 0x03i32]);
            let r: int32x4_t = transmute(veorq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veor_u8() {
        unsafe {
            let a: uint8x8_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let b: uint8x8_t = transmute([
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            ]);
            let e: uint8x8_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let r: uint8x8_t = transmute(veor_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veorq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0x0Fu8,
            ]);
            let b: uint8x16_t = transmute([
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            ]);
            let e: uint8x16_t = transmute([
                0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0x0Fu8,
            ]);
            let r: uint8x16_t = transmute(veorq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veor_u16() {
        unsafe {
            let a: uint16x4_t = transmute([0x00u16, 0x01u16, 0x02u16, 0x03u16]);
            let b: uint16x4_t = transmute([0x00u16, 0x00u16, 0x00u16, 0x00u16]);
            let e: uint16x4_t = transmute([0x00u16, 0x01u16, 0x02u16, 0x03u16]);
            let r: uint16x4_t = transmute(veor_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veorq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([
                0x00u16, 0x01u16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let b: uint16x8_t = transmute([
                0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16, 0x00u16,
            ]);
            let e: uint16x8_t = transmute([
                0x00u16, 0x01u16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let r: uint16x8_t = transmute(veorq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veor_u32() {
        unsafe {
            let a: uint32x2_t = transmute([0x00u32, 0x01u32]);
            let b: uint32x2_t = transmute([0x00u32, 0x00u32]);
            let e: uint32x2_t = transmute([0x00u32, 0x01u32]);
            let r: uint32x2_t = transmute(veor_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veorq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([0x00u32, 0x01u32, 0x02u32, 0x03u32]);
            let b: uint32x4_t = transmute([0x00u32, 0x00u32, 0x00u32, 0x00u32]);
            let e: uint32x4_t = transmute([0x00u32, 0x01u32, 0x02u32, 0x03u32]);
            let r: uint32x4_t = transmute(veorq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veor_s64() {
        unsafe {
            let a: int64x1_t = transmute([0x00i64]);
            let b: int64x1_t = transmute([0x00i64]);
            let e: int64x1_t = transmute([0x00i64]);
            let r: int64x1_t = transmute(veor_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veorq_s64() {
        unsafe {
            let a: int64x2_t = transmute([0x00i64, 0x01i64]);
            let b: int64x2_t = transmute([0x00i64, 0x00i64]);
            let e: int64x2_t = transmute([0x00i64, 0x01i64]);
            let r: int64x2_t = transmute(veorq_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veor_u64() {
        unsafe {
            let a: uint64x1_t = transmute([0x00u64]);
            let b: uint64x1_t = transmute([0x00u64]);
            let e: uint64x1_t = transmute([0x00u64]);
            let r: uint64x1_t = transmute(veor_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_veorq_u64() {
        unsafe {
            let a: uint64x2_t = transmute([0x00u64, 0x01u64]);
            let b: uint64x2_t = transmute([0x00u64, 0x00u64]);
            let e: uint64x2_t = transmute([0x00u64, 0x01u64]);
            let r: uint64x2_t = transmute(veorq_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceq_u8() {
        unsafe {
            let a: uint8x8_t = transmute([
                0xFFu8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let b: uint8x8_t = transmute([
                0xFFu8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vceq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
            ]);
            let b: uint8x8_t =
                transmute([0xFFu8, 0u8, 0x02u8, 0x04u8, 0x04u8, 0x00u8, 0x06u8, 0x08u8]);
            let e: uint8x8_t = transmute([0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8]);
            let r: uint8x8_t = transmute(vceq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceqq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                0xFFu8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0u8,
            ]);
            let b: uint8x16_t = transmute([
                0xFFu8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0u8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vceqq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8,
                0x0Au8, 0x0Bu8, 0xCCu8, 0x0Du8, 0xEEu8, 0u8,
            ]);
            let b: uint8x16_t = transmute([
                0xFFu8, 0u8, 0x02u8, 0x04u8, 0x04u8, 0x00u8, 0x06u8, 0x08u8, 0x08u8, 0x00u8,
                0x0Au8, 0x0Au8, 0xCCu8, 0xD0u8, 0xEEu8, 0xFFu8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8,
                0xFFu8, 0u8, 0xFFu8, 0u8,
            ]);
            let r: uint8x16_t = transmute(vceqq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceq_u16() {
        unsafe {
            let a: uint16x4_t = transmute([0xFF_FFu16, 0x01u16, 0x02u16, 0x03u16]);
            let b: uint16x4_t = transmute([0xFF_FFu16, 0x01u16, 0x02u16, 0x03u16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vceq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0x02u16, 0x03u16]);
            let b: uint16x4_t = transmute([0xFF_FFu16, 0u16, 0x02u16, 0x04u16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0u16, 0xFF_FFu16, 0u16]);
            let r: uint16x4_t = transmute(vceq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceqq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([
                0xFF_FFu16, 0x01u16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let b: uint16x8_t = transmute([
                0xFF_FFu16, 0x01u16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vceqq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0x02u16, 0x03u16, 0x04u16, 0x05u16, 0x06u16, 0x07u16,
            ]);
            let b: uint16x8_t = transmute([
                0xFF_FFu16, 0u16, 0x02u16, 0x04u16, 0x04u16, 0x00u16, 0x06u16, 0x08u16,
            ]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0u16, 0xFF_FFu16, 0u16, 0xFF_FFu16, 0u16, 0xFF_FFu16, 0u16,
            ]);
            let r: uint16x8_t = transmute(vceqq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceq_u32() {
        unsafe {
            let a: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0x01u32]);
            let b: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0x01u32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vceq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let b: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0u32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0u32]);
            let r: uint32x2_t = transmute(vceq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceqq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([0xFF_FF_FF_FFu32, 0x01u32, 0x02u32, 0x03u32]);
            let b: uint32x4_t = transmute([0xFF_FF_FF_FFu32, 0x01u32, 0x02u32, 0x03u32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vceqq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: uint32x4_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32, 0x02u32, 0x03u32]);
            let b: uint32x4_t = transmute([0xFF_FF_FF_FFu32, 0u32, 0x02u32, 0x04u32]);
            let e: uint32x4_t = transmute([0xFF_FF_FF_FFu32, 0u32, 0xFF_FF_FF_FFu32, 0u32]);
            let r: uint32x4_t = transmute(vceqq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceq_s8() {
        unsafe {
            let a: int8x8_t = transmute([
                0x7Fi8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let b: int8x8_t = transmute([
                0x7Fi8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vceq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int8x8_t = transmute([
                0x7Fi8, 0x7Fi8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8,
            ]);
            let b: int8x8_t = transmute([
                0x7Fi8, -128i8, 0x02i8, 0x04i8, 0x04i8, 0x00i8, 0x06i8, 0x08i8,
            ]);
            let e: uint8x8_t = transmute([0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8]);
            let r: uint8x8_t = transmute(vceq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceqq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                0x7Fi8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0x0Ci8, 0x0Di8, 0x0Ei8, -128i8,
            ]);
            let b: int8x16_t = transmute([
                0x7Fi8, 0x01i8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0x0Ci8, 0x0Di8, 0x0Ei8, -128i8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vceqq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int8x16_t = transmute([
                0x7Fi8, 0x7Fi8, 0x02i8, 0x03i8, 0x04i8, 0x05i8, 0x06i8, 0x07i8, 0x08i8, 0x09i8,
                0x0Ai8, 0x0Bi8, 0xCCi8, 0x0Di8, 0xEEi8, -128i8,
            ]);
            let b: int8x16_t = transmute([
                0x7Fi8, -128i8, 0x02i8, 0x04i8, 0x04i8, 0x00i8, 0x06i8, 0x08i8, 0x08i8, 0x00i8,
                0x0Ai8, 0x0Ai8, 0xCCi8, 0xD0i8, 0xEEi8, 0x7Fi8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8, 0xFFu8, 0u8,
                0xFFu8, 0u8, 0xFFu8, 0u8,
            ]);
            let r: uint8x16_t = transmute(vceqq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceq_s16() {
        unsafe {
            let a: int16x4_t = transmute([0x7F_FFi16, 0x01i16, 0x02i16, 0x03i16]);
            let b: int16x4_t = transmute([0x7F_FFi16, 0x01i16, 0x02i16, 0x03i16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vceq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int16x4_t = transmute([0x7F_FFi16, 0x7F_FFi16, 0x02i16, 0x03i16]);
            let b: int16x4_t = transmute([0x7F_FFi16, -32768i16, 0x02i16, 0x04i16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0u16, 0xFF_FFu16, 0u16]);
            let r: uint16x4_t = transmute(vceq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceqq_s16() {
        unsafe {
            let a: int16x8_t = transmute([
                0x7F_FFi16, 0x01i16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let b: int16x8_t = transmute([
                0x7F_FFi16, 0x01i16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vceqq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int16x8_t = transmute([
                0x7F_FFi16, 0x7F_FFi16, 0x02i16, 0x03i16, 0x04i16, 0x05i16, 0x06i16, 0x07i16,
            ]);
            let b: int16x8_t = transmute([
                0x7F_FFi16, -32768i16, 0x02i16, 0x04i16, 0x04i16, 0x00i16, 0x06i16, 0x08i16,
            ]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0u16, 0xFF_FFu16, 0u16, 0xFF_FFu16, 0u16, 0xFF_FFu16, 0u16,
            ]);
            let r: uint16x8_t = transmute(vceqq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceq_s32() {
        unsafe {
            let a: int32x2_t = transmute([0x7F_FF_FF_FFi32, 0x01i32]);
            let b: int32x2_t = transmute([0x7F_FF_FF_FFi32, 0x01i32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vceq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int32x2_t = transmute([0x7F_FF_FF_FFi32, 0x7F_FF_FF_FFi32]);
            let b: int32x2_t = transmute([0x7F_FF_FF_FFi32, -2147483648i32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0u32]);
            let r: uint32x2_t = transmute(vceq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceqq_s32() {
        unsafe {
            let a: int32x4_t = transmute([0x7F_FF_FF_FFi32, 0x01i32, 0x02i32, 0x03i32]);
            let b: int32x4_t = transmute([0x7F_FF_FF_FFi32, 0x01i32, 0x02i32, 0x03i32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vceqq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));

            let a: int32x4_t = transmute([0x7F_FF_FF_FFi32, 0x7F_FF_FF_FFi32, 0x02i32, 0x03i32]);
            let b: int32x4_t = transmute([0x7F_FF_FF_FFi32, -2147483648i32, 0x02i32, 0x04i32]);
            let e: uint32x4_t = transmute([0xFF_FF_FF_FFu32, 0u32, 0xFF_FF_FF_FFu32, 0u32]);
            let r: uint32x4_t = transmute(vceqq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceq_f32() {
        unsafe {
            let a: float32x2_t = transmute([1.2f32, 3.4f32]);
            let b: float32x2_t = transmute([1.2f32, 3.4f32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vceq_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vceqq_f32() {
        unsafe {
            let a: float32x4_t = transmute([1.2f32, 3.4f32, 5.6f32, 7.8f32]);
            let b: float32x4_t = transmute([1.2f32, 3.4f32, 5.6f32, 7.8f32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vceqq_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgt_s8() {
        unsafe {
            let a: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let b: int8x8_t = transmute([0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vcgt_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgtq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let b: int8x16_t = transmute([
                0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8,
                15i8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vcgtq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgt_s16() {
        unsafe {
            let a: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let b: int16x4_t = transmute([0i16, 1i16, 2i16, 3i16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vcgt_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgtq_s16() {
        unsafe {
            let a: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let b: int16x8_t = transmute([0i16, 1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vcgtq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgt_s32() {
        unsafe {
            let a: int32x2_t = transmute([1i32, 2i32]);
            let b: int32x2_t = transmute([0i32, 1i32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vcgt_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgtq_s32() {
        unsafe {
            let a: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let b: int32x4_t = transmute([0i32, 1i32, 2i32, 3i32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcgtq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgt_u8() {
        unsafe {
            let a: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let b: uint8x8_t = transmute([0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vcgt_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgtq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let b: uint8x16_t = transmute([
                0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                15u8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vcgtq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgt_u16() {
        unsafe {
            let a: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let b: uint16x4_t = transmute([0u16, 1u16, 2u16, 3u16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vcgt_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgtq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let b: uint16x8_t = transmute([0u16, 1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vcgtq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgt_u32() {
        unsafe {
            let a: uint32x2_t = transmute([1u32, 2u32]);
            let b: uint32x2_t = transmute([0u32, 1u32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vcgt_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgtq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let b: uint32x4_t = transmute([0u32, 1u32, 2u32, 3u32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcgtq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgt_f32() {
        unsafe {
            let a: float32x2_t = transmute([1.2f32, 2.3f32]);
            let b: float32x2_t = transmute([0.1f32, 1.2f32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vcgt_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgtq_f32() {
        unsafe {
            let a: float32x4_t = transmute([1.2f32, 2.3f32, 3.4f32, 4.5f32]);
            let b: float32x4_t = transmute([0.1f32, 1.2f32, 2.3f32, 3.4f32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcgtq_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vclt_s8() {
        unsafe {
            let a: int8x8_t = transmute([0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8]);
            let b: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vclt_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcltq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8,
                15i8,
            ]);
            let b: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vcltq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vclt_s16() {
        unsafe {
            let a: int16x4_t = transmute([0i16, 1i16, 2i16, 3i16]);
            let b: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vclt_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcltq_s16() {
        unsafe {
            let a: int16x8_t = transmute([0i16, 1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16]);
            let b: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vcltq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vclt_s32() {
        unsafe {
            let a: int32x2_t = transmute([0i32, 1i32]);
            let b: int32x2_t = transmute([1i32, 2i32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vclt_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcltq_s32() {
        unsafe {
            let a: int32x4_t = transmute([0i32, 1i32, 2i32, 3i32]);
            let b: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcltq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vclt_u8() {
        unsafe {
            let a: uint8x8_t = transmute([0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8]);
            let b: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vclt_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcltq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                15u8,
            ]);
            let b: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vcltq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vclt_u16() {
        unsafe {
            let a: uint16x4_t = transmute([0u16, 1u16, 2u16, 3u16]);
            let b: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vclt_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcltq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([0u16, 1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16]);
            let b: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vcltq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vclt_u32() {
        unsafe {
            let a: uint32x2_t = transmute([0u32, 1u32]);
            let b: uint32x2_t = transmute([1u32, 2u32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vclt_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcltq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([0u32, 1u32, 2u32, 3u32]);
            let b: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcltq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vclt_f32() {
        unsafe {
            let a: float32x2_t = transmute([0.1f32, 1.2f32]);
            let b: float32x2_t = transmute([1.2f32, 2.3f32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vclt_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcltq_f32() {
        unsafe {
            let a: float32x4_t = transmute([0.1f32, 1.2f32, 2.3f32, 3.4f32]);
            let b: float32x4_t = transmute([1.2f32, 2.3f32, 3.4f32, 4.5f32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcltq_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcle_s8() {
        unsafe {
            let a: int8x8_t = transmute([0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8]);
            let b: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vcle_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcleq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8,
                15i8,
            ]);
            let b: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vcleq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcle_s16() {
        unsafe {
            let a: int16x4_t = transmute([0i16, 1i16, 2i16, 3i16]);
            let b: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vcle_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcleq_s16() {
        unsafe {
            let a: int16x8_t = transmute([0i16, 1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16]);
            let b: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vcleq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcle_s32() {
        unsafe {
            let a: int32x2_t = transmute([0i32, 1i32]);
            let b: int32x2_t = transmute([1i32, 2i32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vcle_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcleq_s32() {
        unsafe {
            let a: int32x4_t = transmute([0i32, 1i32, 2i32, 3i32]);
            let b: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcleq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcle_u8() {
        unsafe {
            let a: uint8x8_t = transmute([0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8]);
            let b: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vcle_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcleq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                15u8,
            ]);
            let b: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vcleq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcle_u16() {
        unsafe {
            let a: uint16x4_t = transmute([0u16, 1u16, 2u16, 3u16]);
            let b: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vcle_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcleq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([0u16, 1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16]);
            let b: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vcleq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcle_u32() {
        unsafe {
            let a: uint32x2_t = transmute([0u32, 1u32]);
            let b: uint32x2_t = transmute([1u32, 2u32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vcle_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcleq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([0u32, 1u32, 2u32, 3u32]);
            let b: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcleq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcle_f32() {
        unsafe {
            let a: float32x2_t = transmute([0.1f32, 1.2f32]);
            let b: float32x2_t = transmute([1.2f32, 2.3f32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vcle_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcleq_f32() {
        unsafe {
            let a: float32x4_t = transmute([0.1f32, 1.2f32, 2.3f32, 3.4f32]);
            let b: float32x4_t = transmute([1.2f32, 2.3f32, 3.4f32, 4.5f32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcleq_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcge_s8() {
        unsafe {
            let a: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let b: int8x8_t = transmute([0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vcge_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgeq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let b: int8x16_t = transmute([
                0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8,
                15i8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vcgeq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcge_s16() {
        unsafe {
            let a: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let b: int16x4_t = transmute([0i16, 1i16, 2i16, 3i16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vcge_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgeq_s16() {
        unsafe {
            let a: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let b: int16x8_t = transmute([0i16, 1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vcgeq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcge_s32() {
        unsafe {
            let a: int32x2_t = transmute([1i32, 2i32]);
            let b: int32x2_t = transmute([0i32, 1i32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vcge_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgeq_s32() {
        unsafe {
            let a: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let b: int32x4_t = transmute([0i32, 1i32, 2i32, 3i32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcgeq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcge_u8() {
        unsafe {
            let a: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let b: uint8x8_t = transmute([0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8]);
            let e: uint8x8_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x8_t = transmute(vcge_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgeq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let b: uint8x16_t = transmute([
                0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8,
                15u8,
            ]);
            let e: uint8x16_t = transmute([
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
                0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            ]);
            let r: uint8x16_t = transmute(vcgeq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcge_u16() {
        unsafe {
            let a: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let b: uint16x4_t = transmute([0u16, 1u16, 2u16, 3u16]);
            let e: uint16x4_t = transmute([0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16]);
            let r: uint16x4_t = transmute(vcge_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgeq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let b: uint16x8_t = transmute([0u16, 1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16]);
            let e: uint16x8_t = transmute([
                0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16, 0xFF_FFu16,
                0xFF_FFu16,
            ]);
            let r: uint16x8_t = transmute(vcgeq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcge_u32() {
        unsafe {
            let a: uint32x2_t = transmute([1u32, 2u32]);
            let b: uint32x2_t = transmute([0u32, 1u32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vcge_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgeq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let b: uint32x4_t = transmute([0u32, 1u32, 2u32, 3u32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcgeq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcge_f32() {
        unsafe {
            let a: float32x2_t = transmute([1.2f32, 2.3f32]);
            let b: float32x2_t = transmute([0.1f32, 1.2f32]);
            let e: uint32x2_t = transmute([0xFF_FF_FF_FFu32, 0xFF_FF_FF_FFu32]);
            let r: uint32x2_t = transmute(vcge_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vcgeq_f32() {
        unsafe {
            let a: float32x4_t = transmute([1.2f32, 2.3f32, 3.4f32, 4.5f32]);
            let b: float32x4_t = transmute([0.1f32, 1.2f32, 2.3f32, 3.4f32]);
            let e: uint32x4_t = transmute([
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
                0xFF_FF_FF_FFu32,
            ]);
            let r: uint32x4_t = transmute(vcgeq_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsub_u8() {
        unsafe {
            let a: uint8x8_t = transmute([42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8]);
            let b: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let e: uint8x8_t = transmute([41u8, 40u8, 39u8, 38u8, 37u8, 36u8, 35u8, 34u8]);
            let r: uint8x8_t = transmute(vqsub_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsubq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8,
                42u8, 42u8,
            ]);
            let b: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let e: uint8x16_t = transmute([
                41u8, 40u8, 39u8, 38u8, 37u8, 36u8, 35u8, 34u8, 33u8, 32u8, 31u8, 30u8, 29u8, 28u8,
                27u8, 26u8,
            ]);
            let r: uint8x16_t = transmute(vqsubq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsub_u16() {
        unsafe {
            let a: uint16x4_t = transmute([42u16, 42u16, 42u16, 42u16]);
            let b: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let e: uint16x4_t = transmute([41u16, 40u16, 39u16, 38u16]);
            let r: uint16x4_t = transmute(vqsub_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsubq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([42u16, 42u16, 42u16, 42u16, 42u16, 42u16, 42u16, 42u16]);
            let b: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let e: uint16x8_t = transmute([41u16, 40u16, 39u16, 38u16, 37u16, 36u16, 35u16, 34u16]);
            let r: uint16x8_t = transmute(vqsubq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsub_u32() {
        unsafe {
            let a: uint32x2_t = transmute([42u32, 42u32]);
            let b: uint32x2_t = transmute([1u32, 2u32]);
            let e: uint32x2_t = transmute([41u32, 40u32]);
            let r: uint32x2_t = transmute(vqsub_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsubq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([42u32, 42u32, 42u32, 42u32]);
            let b: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let e: uint32x4_t = transmute([41u32, 40u32, 39u32, 38u32]);
            let r: uint32x4_t = transmute(vqsubq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsub_s8() {
        unsafe {
            let a: int8x8_t = transmute([42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8]);
            let b: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let e: int8x8_t = transmute([41i8, 40i8, 39i8, 38i8, 37i8, 36i8, 35i8, 34i8]);
            let r: int8x8_t = transmute(vqsub_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsubq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8,
                42i8, 42i8,
            ]);
            let b: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let e: int8x16_t = transmute([
                41i8, 40i8, 39i8, 38i8, 37i8, 36i8, 35i8, 34i8, 33i8, 32i8, 31i8, 30i8, 29i8, 28i8,
                27i8, 26i8,
            ]);
            let r: int8x16_t = transmute(vqsubq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsub_s16() {
        unsafe {
            let a: int16x4_t = transmute([42i16, 42i16, 42i16, 42i16]);
            let b: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let e: int16x4_t = transmute([41i16, 40i16, 39i16, 38i16]);
            let r: int16x4_t = transmute(vqsub_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsubq_s16() {
        unsafe {
            let a: int16x8_t = transmute([42i16, 42i16, 42i16, 42i16, 42i16, 42i16, 42i16, 42i16]);
            let b: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let e: int16x8_t = transmute([41i16, 40i16, 39i16, 38i16, 37i16, 36i16, 35i16, 34i16]);
            let r: int16x8_t = transmute(vqsubq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsub_s32() {
        unsafe {
            let a: int32x2_t = transmute([42i32, 42i32]);
            let b: int32x2_t = transmute([1i32, 2i32]);
            let e: int32x2_t = transmute([41i32, 40i32]);
            let r: int32x2_t = transmute(vqsub_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqsubq_s32() {
        unsafe {
            let a: int32x4_t = transmute([42i32, 42i32, 42i32, 42i32]);
            let b: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let e: int32x4_t = transmute([41i32, 40i32, 39i32, 38i32]);
            let r: int32x4_t = transmute(vqsubq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhadd_u8() {
        unsafe {
            let a: uint8x8_t = transmute([42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8]);
            let b: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let e: uint8x8_t = transmute([21u8, 22u8, 22u8, 23u8, 23u8, 24u8, 24u8, 25u8]);
            let r: uint8x8_t = transmute(vhadd_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhaddq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8,
                42u8, 42u8,
            ]);
            let b: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let e: uint8x16_t = transmute([
                21u8, 22u8, 22u8, 23u8, 23u8, 24u8, 24u8, 25u8, 25u8, 26u8, 26u8, 27u8, 27u8, 28u8,
                28u8, 29u8,
            ]);
            let r: uint8x16_t = transmute(vhaddq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhadd_u16() {
        unsafe {
            let a: uint16x4_t = transmute([42u16, 42u16, 42u16, 42u16]);
            let b: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let e: uint16x4_t = transmute([21u16, 22u16, 22u16, 23u16]);
            let r: uint16x4_t = transmute(vhadd_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhaddq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([42u16, 42u16, 42u16, 42u16, 42u16, 42u16, 42u16, 42u16]);
            let b: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let e: uint16x8_t = transmute([21u16, 22u16, 22u16, 23u16, 23u16, 24u16, 24u16, 25u16]);
            let r: uint16x8_t = transmute(vhaddq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhadd_u32() {
        unsafe {
            let a: uint32x2_t = transmute([42u32, 42u32]);
            let b: uint32x2_t = transmute([1u32, 2u32]);
            let e: uint32x2_t = transmute([21u32, 22u32]);
            let r: uint32x2_t = transmute(vhadd_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhaddq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([42u32, 42u32, 42u32, 42u32]);
            let b: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let e: uint32x4_t = transmute([21u32, 22u32, 22u32, 23u32]);
            let r: uint32x4_t = transmute(vhaddq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhadd_s8() {
        unsafe {
            let a: int8x8_t = transmute([42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8]);
            let b: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let e: int8x8_t = transmute([21i8, 22i8, 22i8, 23i8, 23i8, 24i8, 24i8, 25i8]);
            let r: int8x8_t = transmute(vhadd_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhaddq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8,
                42i8, 42i8,
            ]);
            let b: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let e: int8x16_t = transmute([
                21i8, 22i8, 22i8, 23i8, 23i8, 24i8, 24i8, 25i8, 25i8, 26i8, 26i8, 27i8, 27i8, 28i8,
                28i8, 29i8,
            ]);
            let r: int8x16_t = transmute(vhaddq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhadd_s16() {
        unsafe {
            let a: int16x4_t = transmute([42i16, 42i16, 42i16, 42i16]);
            let b: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let e: int16x4_t = transmute([21i16, 22i16, 22i16, 23i16]);
            let r: int16x4_t = transmute(vhadd_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhaddq_s16() {
        unsafe {
            let a: int16x8_t = transmute([42i16, 42i16, 42i16, 42i16, 42i16, 42i16, 42i16, 42i16]);
            let b: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let e: int16x8_t = transmute([21i16, 22i16, 22i16, 23i16, 23i16, 24i16, 24i16, 25i16]);
            let r: int16x8_t = transmute(vhaddq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhadd_s32() {
        unsafe {
            let a: int32x2_t = transmute([42i32, 42i32]);
            let b: int32x2_t = transmute([1i32, 2i32]);
            let e: int32x2_t = transmute([21i32, 22i32]);
            let r: int32x2_t = transmute(vhadd_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhaddq_s32() {
        unsafe {
            let a: int32x4_t = transmute([42i32, 42i32, 42i32, 42i32]);
            let b: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let e: int32x4_t = transmute([21i32, 22i32, 22i32, 23i32]);
            let r: int32x4_t = transmute(vhaddq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhadd_u8() {
        unsafe {
            let a: uint8x8_t = transmute([42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8]);
            let b: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let e: uint8x8_t = transmute([22u8, 22u8, 23u8, 23u8, 24u8, 24u8, 25u8, 25u8]);
            let r: uint8x8_t = transmute(vrhadd_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhaddq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8,
                42u8, 42u8,
            ]);
            let b: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let e: uint8x16_t = transmute([
                22u8, 22u8, 23u8, 23u8, 24u8, 24u8, 25u8, 25u8, 26u8, 26u8, 27u8, 27u8, 28u8, 28u8,
                29u8, 29u8,
            ]);
            let r: uint8x16_t = transmute(vrhaddq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhadd_u16() {
        unsafe {
            let a: uint16x4_t = transmute([42u16, 42u16, 42u16, 42u16]);
            let b: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let e: uint16x4_t = transmute([22u16, 22u16, 23u16, 23u16]);
            let r: uint16x4_t = transmute(vrhadd_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhaddq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([42u16, 42u16, 42u16, 42u16, 42u16, 42u16, 42u16, 42u16]);
            let b: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let e: uint16x8_t = transmute([22u16, 22u16, 23u16, 23u16, 24u16, 24u16, 25u16, 25u16]);
            let r: uint16x8_t = transmute(vrhaddq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhadd_u32() {
        unsafe {
            let a: uint32x2_t = transmute([42u32, 42u32]);
            let b: uint32x2_t = transmute([1u32, 2u32]);
            let e: uint32x2_t = transmute([22u32, 22u32]);
            let r: uint32x2_t = transmute(vrhadd_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhaddq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([42u32, 42u32, 42u32, 42u32]);
            let b: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let e: uint32x4_t = transmute([22u32, 22u32, 23u32, 23u32]);
            let r: uint32x4_t = transmute(vrhaddq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhadd_s8() {
        unsafe {
            let a: int8x8_t = transmute([42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8]);
            let b: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let e: int8x8_t = transmute([22i8, 22i8, 23i8, 23i8, 24i8, 24i8, 25i8, 25i8]);
            let r: int8x8_t = transmute(vrhadd_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhaddq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8,
                42i8, 42i8,
            ]);
            let b: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let e: int8x16_t = transmute([
                22i8, 22i8, 23i8, 23i8, 24i8, 24i8, 25i8, 25i8, 26i8, 26i8, 27i8, 27i8, 28i8, 28i8,
                29i8, 29i8,
            ]);
            let r: int8x16_t = transmute(vrhaddq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhadd_s16() {
        unsafe {
            let a: int16x4_t = transmute([42i16, 42i16, 42i16, 42i16]);
            let b: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let e: int16x4_t = transmute([22i16, 22i16, 23i16, 23i16]);
            let r: int16x4_t = transmute(vrhadd_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhaddq_s16() {
        unsafe {
            let a: int16x8_t = transmute([42i16, 42i16, 42i16, 42i16, 42i16, 42i16, 42i16, 42i16]);
            let b: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let e: int16x8_t = transmute([22i16, 22i16, 23i16, 23i16, 24i16, 24i16, 25i16, 25i16]);
            let r: int16x8_t = transmute(vrhaddq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhadd_s32() {
        unsafe {
            let a: int32x2_t = transmute([42i32, 42i32]);
            let b: int32x2_t = transmute([1i32, 2i32]);
            let e: int32x2_t = transmute([22i32, 22i32]);
            let r: int32x2_t = transmute(vrhadd_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vrhaddq_s32() {
        unsafe {
            let a: int32x4_t = transmute([42i32, 42i32, 42i32, 42i32]);
            let b: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let e: int32x4_t = transmute([22i32, 22i32, 23i32, 23i32]);
            let r: int32x4_t = transmute(vrhaddq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqadd_u8() {
        unsafe {
            let a: uint8x8_t = transmute([42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8]);
            let b: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let e: uint8x8_t = transmute([43u8, 44u8, 45u8, 46u8, 47u8, 48u8, 49u8, 50u8]);
            let r: uint8x8_t = transmute(vqadd_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqaddq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8, 42u8,
                42u8, 42u8,
            ]);
            let b: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let e: uint8x16_t = transmute([
                43u8, 44u8, 45u8, 46u8, 47u8, 48u8, 49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8,
                57u8, 58u8,
            ]);
            let r: uint8x16_t = transmute(vqaddq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqadd_u16() {
        unsafe {
            let a: uint16x4_t = transmute([42u16, 42u16, 42u16, 42u16]);
            let b: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let e: uint16x4_t = transmute([43u16, 44u16, 45u16, 46u16]);
            let r: uint16x4_t = transmute(vqadd_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqaddq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([42u16, 42u16, 42u16, 42u16, 42u16, 42u16, 42u16, 42u16]);
            let b: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let e: uint16x8_t = transmute([43u16, 44u16, 45u16, 46u16, 47u16, 48u16, 49u16, 50u16]);
            let r: uint16x8_t = transmute(vqaddq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqadd_u32() {
        unsafe {
            let a: uint32x2_t = transmute([42u32, 42u32]);
            let b: uint32x2_t = transmute([1u32, 2u32]);
            let e: uint32x2_t = transmute([43u32, 44u32]);
            let r: uint32x2_t = transmute(vqadd_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqaddq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([42u32, 42u32, 42u32, 42u32]);
            let b: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let e: uint32x4_t = transmute([43u32, 44u32, 45u32, 46u32]);
            let r: uint32x4_t = transmute(vqaddq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqadd_s8() {
        unsafe {
            let a: int8x8_t = transmute([42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8]);
            let b: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let e: int8x8_t = transmute([43i8, 44i8, 45i8, 46i8, 47i8, 48i8, 49i8, 50i8]);
            let r: int8x8_t = transmute(vqadd_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqaddq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8, 42i8,
                42i8, 42i8,
            ]);
            let b: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let e: int8x16_t = transmute([
                43i8, 44i8, 45i8, 46i8, 47i8, 48i8, 49i8, 50i8, 51i8, 52i8, 53i8, 54i8, 55i8, 56i8,
                57i8, 58i8,
            ]);
            let r: int8x16_t = transmute(vqaddq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqadd_s16() {
        unsafe {
            let a: int16x4_t = transmute([42i16, 42i16, 42i16, 42i16]);
            let b: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let e: int16x4_t = transmute([43i16, 44i16, 45i16, 46i16]);
            let r: int16x4_t = transmute(vqadd_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqaddq_s16() {
        unsafe {
            let a: int16x8_t = transmute([42i16, 42i16, 42i16, 42i16, 42i16, 42i16, 42i16, 42i16]);
            let b: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let e: int16x8_t = transmute([43i16, 44i16, 45i16, 46i16, 47i16, 48i16, 49i16, 50i16]);
            let r: int16x8_t = transmute(vqaddq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqadd_s32() {
        unsafe {
            let a: int32x2_t = transmute([42i32, 42i32]);
            let b: int32x2_t = transmute([1i32, 2i32]);
            let e: int32x2_t = transmute([43i32, 44i32]);
            let r: int32x2_t = transmute(vqadd_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vqaddq_s32() {
        unsafe {
            let a: int32x4_t = transmute([42i32, 42i32, 42i32, 42i32]);
            let b: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let e: int32x4_t = transmute([43i32, 44i32, 45i32, 46i32]);
            let r: int32x4_t = transmute(vqaddq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmul_s8() {
        unsafe {
            let a: int8x8_t = transmute([1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8]);
            let b: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let e: int8x8_t = transmute([1i8, 4i8, 3i8, 8i8, 5i8, 12i8, 7i8, 16i8]);
            let r: int8x8_t = transmute(vmul_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmulq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8,
            ]);
            let b: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let e: int8x16_t = transmute([
                1i8, 4i8, 3i8, 8i8, 5i8, 12i8, 7i8, 16i8, 9i8, 20i8, 11i8, 24i8, 13i8, 28i8, 15i8,
                32i8,
            ]);
            let r: int8x16_t = transmute(vmulq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmul_s16() {
        unsafe {
            let a: int16x4_t = transmute([1i16, 2i16, 1i16, 2i16]);
            let b: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let e: int16x4_t = transmute([1i16, 4i16, 3i16, 8i16]);
            let r: int16x4_t = transmute(vmul_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmulq_s16() {
        unsafe {
            let a: int16x8_t = transmute([1i16, 2i16, 1i16, 2i16, 1i16, 2i16, 1i16, 2i16]);
            let b: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let e: int16x8_t = transmute([1i16, 4i16, 3i16, 8i16, 5i16, 12i16, 7i16, 16i16]);
            let r: int16x8_t = transmute(vmulq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmul_s32() {
        unsafe {
            let a: int32x2_t = transmute([1i32, 2i32]);
            let b: int32x2_t = transmute([1i32, 2i32]);
            let e: int32x2_t = transmute([1i32, 4i32]);
            let r: int32x2_t = transmute(vmul_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmulq_s32() {
        unsafe {
            let a: int32x4_t = transmute([1i32, 2i32, 1i32, 2i32]);
            let b: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let e: int32x4_t = transmute([1i32, 4i32, 3i32, 8i32]);
            let r: int32x4_t = transmute(vmulq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmul_u8() {
        unsafe {
            let a: uint8x8_t = transmute([1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8]);
            let b: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let e: uint8x8_t = transmute([1u8, 4u8, 3u8, 8u8, 5u8, 12u8, 7u8, 16u8]);
            let r: uint8x8_t = transmute(vmul_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmulq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8,
            ]);
            let b: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let e: uint8x16_t = transmute([
                1u8, 4u8, 3u8, 8u8, 5u8, 12u8, 7u8, 16u8, 9u8, 20u8, 11u8, 24u8, 13u8, 28u8, 15u8,
                32u8,
            ]);
            let r: uint8x16_t = transmute(vmulq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmul_u16() {
        unsafe {
            let a: uint16x4_t = transmute([1u16, 2u16, 1u16, 2u16]);
            let b: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let e: uint16x4_t = transmute([1u16, 4u16, 3u16, 8u16]);
            let r: uint16x4_t = transmute(vmul_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmulq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([1u16, 2u16, 1u16, 2u16, 1u16, 2u16, 1u16, 2u16]);
            let b: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let e: uint16x8_t = transmute([1u16, 4u16, 3u16, 8u16, 5u16, 12u16, 7u16, 16u16]);
            let r: uint16x8_t = transmute(vmulq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmul_u32() {
        unsafe {
            let a: uint32x2_t = transmute([1u32, 2u32]);
            let b: uint32x2_t = transmute([1u32, 2u32]);
            let e: uint32x2_t = transmute([1u32, 4u32]);
            let r: uint32x2_t = transmute(vmul_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmulq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([1u32, 2u32, 1u32, 2u32]);
            let b: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let e: uint32x4_t = transmute([1u32, 4u32, 3u32, 8u32]);
            let r: uint32x4_t = transmute(vmulq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmul_f32() {
        unsafe {
            let a: float32x2_t = transmute([1.0f32, 2.0f32]);
            let b: float32x2_t = transmute([2.0f32, 3.0f32]);
            let e: float32x2_t = transmute([2.0f32, 6.0f32]);
            let r: float32x2_t = transmute(vmul_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vmulq_f32() {
        unsafe {
            let a: float32x4_t = transmute([1.0f32, 2.0f32, 1.0f32, 2.0f32]);
            let b: float32x4_t = transmute([2.0f32, 3.0f32, 4.0f32, 5.0f32]);
            let e: float32x4_t = transmute([2.0f32, 6.0f32, 4.0f32, 10.0f32]);
            let r: float32x4_t = transmute(vmulq_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsub_s8() {
        unsafe {
            let a: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let b: int8x8_t = transmute([1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8]);
            let e: int8x8_t = transmute([0i8, 0i8, 2i8, 2i8, 4i8, 4i8, 6i8, 6i8]);
            let r: int8x8_t = transmute(vsub_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsubq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let b: int8x16_t = transmute([
                1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8,
            ]);
            let e: int8x16_t = transmute([
                0i8, 0i8, 2i8, 2i8, 4i8, 4i8, 6i8, 6i8, 8i8, 8i8, 10i8, 10i8, 12i8, 12i8, 14i8,
                14i8,
            ]);
            let r: int8x16_t = transmute(vsubq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsub_s16() {
        unsafe {
            let a: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let b: int16x4_t = transmute([1i16, 2i16, 1i16, 2i16]);
            let e: int16x4_t = transmute([0i16, 0i16, 2i16, 2i16]);
            let r: int16x4_t = transmute(vsub_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsubq_s16() {
        unsafe {
            let a: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let b: int16x8_t = transmute([1i16, 2i16, 1i16, 2i16, 1i16, 2i16, 1i16, 2i16]);
            let e: int16x8_t = transmute([0i16, 0i16, 2i16, 2i16, 4i16, 4i16, 6i16, 6i16]);
            let r: int16x8_t = transmute(vsubq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsub_s32() {
        unsafe {
            let a: int32x2_t = transmute([1i32, 2i32]);
            let b: int32x2_t = transmute([1i32, 2i32]);
            let e: int32x2_t = transmute([0i32, 0i32]);
            let r: int32x2_t = transmute(vsub_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsubq_s32() {
        unsafe {
            let a: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let b: int32x4_t = transmute([1i32, 2i32, 1i32, 2i32]);
            let e: int32x4_t = transmute([0i32, 0i32, 2i32, 2i32]);
            let r: int32x4_t = transmute(vsubq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsub_u8() {
        unsafe {
            let a: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let b: uint8x8_t = transmute([1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8]);
            let e: uint8x8_t = transmute([0u8, 0u8, 2u8, 2u8, 4u8, 4u8, 6u8, 6u8]);
            let r: uint8x8_t = transmute(vsub_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsubq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let b: uint8x16_t = transmute([
                1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8,
            ]);
            let e: uint8x16_t = transmute([
                0u8, 0u8, 2u8, 2u8, 4u8, 4u8, 6u8, 6u8, 8u8, 8u8, 10u8, 10u8, 12u8, 12u8, 14u8,
                14u8,
            ]);
            let r: uint8x16_t = transmute(vsubq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsub_u16() {
        unsafe {
            let a: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let b: uint16x4_t = transmute([1u16, 2u16, 1u16, 2u16]);
            let e: uint16x4_t = transmute([0u16, 0u16, 2u16, 2u16]);
            let r: uint16x4_t = transmute(vsub_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsubq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let b: uint16x8_t = transmute([1u16, 2u16, 1u16, 2u16, 1u16, 2u16, 1u16, 2u16]);
            let e: uint16x8_t = transmute([0u16, 0u16, 2u16, 2u16, 4u16, 4u16, 6u16, 6u16]);
            let r: uint16x8_t = transmute(vsubq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsub_u32() {
        unsafe {
            let a: uint32x2_t = transmute([1u32, 2u32]);
            let b: uint32x2_t = transmute([1u32, 2u32]);
            let e: uint32x2_t = transmute([0u32, 0u32]);
            let r: uint32x2_t = transmute(vsub_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsubq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let b: uint32x4_t = transmute([1u32, 2u32, 1u32, 2u32]);
            let e: uint32x4_t = transmute([0u32, 0u32, 2u32, 2u32]);
            let r: uint32x4_t = transmute(vsubq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsub_s64() {
        unsafe {
            let a: int64x1_t = transmute([1i64]);
            let b: int64x1_t = transmute([1i64]);
            let e: int64x1_t = transmute([0i64]);
            let r: int64x1_t = transmute(vsub_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsubq_s64() {
        unsafe {
            let a: int64x2_t = transmute([1i64, 2i64]);
            let b: int64x2_t = transmute([1i64, 2i64]);
            let e: int64x2_t = transmute([0i64, 0i64]);
            let r: int64x2_t = transmute(vsubq_s64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsub_u64() {
        unsafe {
            let a: uint64x1_t = transmute([1u64]);
            let b: uint64x1_t = transmute([1u64]);
            let e: uint64x1_t = transmute([0u64]);
            let r: uint64x1_t = transmute(vsub_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsubq_u64() {
        unsafe {
            let a: uint64x2_t = transmute([1u64, 2u64]);
            let b: uint64x2_t = transmute([1u64, 2u64]);
            let e: uint64x2_t = transmute([0u64, 0u64]);
            let r: uint64x2_t = transmute(vsubq_u64(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsub_f32() {
        unsafe {
            let a: float32x2_t = transmute([1.0f32, 4.0f32]);
            let b: float32x2_t = transmute([1.0f32, 2.0f32]);
            let e: float32x2_t = transmute([0.0f32, 2.0f32]);
            let r: float32x2_t = transmute(vsub_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vsubq_f32() {
        unsafe {
            let a: float32x4_t = transmute([1.0f32, 4.0f32, 3.0f32, 8.0f32]);
            let b: float32x4_t = transmute([1.0f32, 2.0f32, 3.0f32, 4.0f32]);
            let e: float32x4_t = transmute([0.0f32, 2.0f32, 0.0f32, 4.0f32]);
            let r: float32x4_t = transmute(vsubq_f32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsub_u8() {
        unsafe {
            let a: uint8x8_t = transmute([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
            let b: uint8x8_t = transmute([1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8]);
            let e: uint8x8_t = transmute([0u8, 0u8, 1u8, 1u8, 2u8, 2u8, 3u8, 3u8]);
            let r: uint8x8_t = transmute(vhsub_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsubq_u8() {
        unsafe {
            let a: uint8x16_t = transmute([
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8,
            ]);
            let b: uint8x16_t = transmute([
                1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8, 1u8, 2u8,
            ]);
            let e: uint8x16_t = transmute([
                0u8, 0u8, 1u8, 1u8, 2u8, 2u8, 3u8, 3u8, 4u8, 4u8, 5u8, 5u8, 6u8, 6u8, 7u8, 7u8,
            ]);
            let r: uint8x16_t = transmute(vhsubq_u8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsub_u16() {
        unsafe {
            let a: uint16x4_t = transmute([1u16, 2u16, 3u16, 4u16]);
            let b: uint16x4_t = transmute([1u16, 2u16, 1u16, 2u16]);
            let e: uint16x4_t = transmute([0u16, 0u16, 1u16, 1u16]);
            let r: uint16x4_t = transmute(vhsub_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsubq_u16() {
        unsafe {
            let a: uint16x8_t = transmute([1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);
            let b: uint16x8_t = transmute([1u16, 2u16, 1u16, 2u16, 1u16, 2u16, 1u16, 2u16]);
            let e: uint16x8_t = transmute([0u16, 0u16, 1u16, 1u16, 2u16, 2u16, 3u16, 3u16]);
            let r: uint16x8_t = transmute(vhsubq_u16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsub_u32() {
        unsafe {
            let a: uint32x2_t = transmute([1u32, 2u32]);
            let b: uint32x2_t = transmute([1u32, 2u32]);
            let e: uint32x2_t = transmute([0u32, 0u32]);
            let r: uint32x2_t = transmute(vhsub_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsubq_u32() {
        unsafe {
            let a: uint32x4_t = transmute([1u32, 2u32, 3u32, 4u32]);
            let b: uint32x4_t = transmute([1u32, 2u32, 1u32, 2u32]);
            let e: uint32x4_t = transmute([0u32, 0u32, 1u32, 1u32]);
            let r: uint32x4_t = transmute(vhsubq_u32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsub_s8() {
        unsafe {
            let a: int8x8_t = transmute([1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8]);
            let b: int8x8_t = transmute([1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8]);
            let e: int8x8_t = transmute([0i8, 0i8, 1i8, 1i8, 2i8, 2i8, 3i8, 3i8]);
            let r: int8x8_t = transmute(vhsub_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsubq_s8() {
        unsafe {
            let a: int8x16_t = transmute([
                1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8,
                16i8,
            ]);
            let b: int8x16_t = transmute([
                1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8, 1i8, 2i8,
            ]);
            let e: int8x16_t = transmute([
                0i8, 0i8, 1i8, 1i8, 2i8, 2i8, 3i8, 3i8, 4i8, 4i8, 5i8, 5i8, 6i8, 6i8, 7i8, 7i8,
            ]);
            let r: int8x16_t = transmute(vhsubq_s8(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsub_s16() {
        unsafe {
            let a: int16x4_t = transmute([1i16, 2i16, 3i16, 4i16]);
            let b: int16x4_t = transmute([1i16, 2i16, 1i16, 2i16]);
            let e: int16x4_t = transmute([0i16, 0i16, 1i16, 1i16]);
            let r: int16x4_t = transmute(vhsub_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsubq_s16() {
        unsafe {
            let a: int16x8_t = transmute([1i16, 2i16, 3i16, 4i16, 5i16, 6i16, 7i16, 8i16]);
            let b: int16x8_t = transmute([1i16, 2i16, 1i16, 2i16, 1i16, 2i16, 1i16, 2i16]);
            let e: int16x8_t = transmute([0i16, 0i16, 1i16, 1i16, 2i16, 2i16, 3i16, 3i16]);
            let r: int16x8_t = transmute(vhsubq_s16(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsub_s32() {
        unsafe {
            let a: int32x2_t = transmute([1i32, 2i32]);
            let b: int32x2_t = transmute([1i32, 2i32]);
            let e: int32x2_t = transmute([0i32, 0i32]);
            let r: int32x2_t = transmute(vhsub_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }

    // FIXME: #[simd_test(enable = "neon")]
    #[test]
    fn test_vhsubq_s32() {
        unsafe {
            let a: int32x4_t = transmute([1i32, 2i32, 3i32, 4i32]);
            let b: int32x4_t = transmute([1i32, 2i32, 1i32, 2i32]);
            let e: int32x4_t = transmute([0i32, 0i32, 1i32, 1i32]);
            let r: int32x4_t = transmute(vhsubq_s32(transmute(a), transmute(b)));
            assert!(cmp_arm(r, e));
        }
    }
}
