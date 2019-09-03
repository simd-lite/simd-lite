use crate::aarch64::neon::test_support::*;
use crate::arm::neon::test_support::*;
use crate::cmparm::cmp_arm;
use crate::{aarch64::neon::*, aarch64::*};
use std::mem::transmute as __transmute;
macro_rules! transmute {
    ($v: expr) => {
        unsafe { __transmute(unsafe { $v }) }
    };
}

//    use stdarch_test::simd_test;

#[test] //FIXME: #[simd_test(enable = "neon")]
fn test_vpaddq_u8() {
    let a = int8x16_t::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let b = int8x16_t::new([
        17, 18, 19, 20, 20, 21, 22, 23, 24, 25, 26, 27, 29, 29, 30, 31,
    ]);
    let r = int8x16_t::new([1, 5, 9, 13, 17, 21, 25, 29, 35, 39, 41, 45, 49, 53, 58, 61]);
    let e: int8x16_t = unsafe { transmute!(vpaddq_u8(transmute!(a), transmute!(b))) };
    assert!(cmp_arm(r, e));
}

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
