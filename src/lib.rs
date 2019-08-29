
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod arm;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
