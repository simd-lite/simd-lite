#![unstable(feature = "stdsimd", issue = "27731")]
#![feature(
    const_fn,
    const_fn_union,
    custom_inner_attributes,
    link_llvm_intrinsics,
    platform_intrinsics,
    repr_simd,
    simd_ffi,
    asm,
    proc_macro_hygiene,
    stmt_expr_attributes,
    core_intrinsics,
    no_core,
    rustc_attrs,
    stdsimd,
    staged_api,
    doc_cfg,
    mmx_target_feature,
    tbm_target_feature,
    sse4a_target_feature,
    arm_target_feature,
    aarch64_target_feature,
    cmpxchg16b_target_feature,
    avx512_target_feature,
    mips_target_feature,
    powerpc_target_feature,
    wasm_target_feature,
    abi_unadjusted,
    adx_target_feature,
    rtm_target_feature,
    f16c_target_feature,
    external_doc
)]

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod arm;

mod simd;
mod simd_llvm;

pub trait NeonInit {
    type From;
    fn new(input: Self::From) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
