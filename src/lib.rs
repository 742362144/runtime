#![feature(generators, generator_trait)]
#![feature(llvm_asm)]

pub mod task;
pub mod executor;
pub mod policy;
pub mod invoke;
pub mod cycles;
pub mod ext;
mod sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
