pub(crate) trait CMPArm {
    fn eq_arm(self, o: Self) -> bool;
}

pub(crate) fn cmp_arm<T: CMPArm>(l: T, r: T) -> bool {
    l.eq_arm(r)
}

macro_rules! impl_cmp_arm {
    ($in_t:ty, $out_t:ty, $len:expr) => {
        impl CMPArm for $in_t {
            fn eq_arm(self, o: Self) -> bool {
                use std::mem::transmute;
                unsafe {
                    let l: [$out_t; $len] = transmute(self);
                    let r: [$out_t; $len] = transmute(o);
                    l == r
                }
            }
        }
    };
}
mod arm {
    use super::*;
    use crate::arm::*;
    impl_cmp_arm!(int8x8_t, i8, 8);
    impl_cmp_arm!(int8x16_t, i8, 16);
    impl_cmp_arm!(uint8x8_t, u8, 8);
    impl_cmp_arm!(uint8x16_t, u8, 16);

    impl_cmp_arm!(int16x4_t, i16, 4);
    impl_cmp_arm!(int16x8_t, i16, 8);
    impl_cmp_arm!(uint16x4_t, u16, 4);
    impl_cmp_arm!(uint16x8_t, u16, 8);

    impl_cmp_arm!(int32x2_t, i32, 2);
    impl_cmp_arm!(int32x4_t, i32, 4);
    impl_cmp_arm!(uint32x2_t, u32, 2);
    impl_cmp_arm!(uint32x4_t, u32, 4);

    impl_cmp_arm!(int64x1_t, i64, 1);
    impl_cmp_arm!(uint64x1_t, u64, 1);
}

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    use super::*;
    use crate::aarch64::*;
    impl_cmp_arm!(float32x2_t, f32, 2);
    impl_cmp_arm!(float32x4_t, f32, 4);

    impl_cmp_arm!(float64x1_t, f64, 1);
    impl_cmp_arm!(float64x2_t, f64, 2);

    impl_cmp_arm!(int64x2_t, i64, 2);
    impl_cmp_arm!(uint64x2_t, u64, 2);
}
