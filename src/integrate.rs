//!
//! Integration
//! 

pub trait Integrator {
    type Input;
    type Output;

    fn integrate<F>(from: Self::Input, to: Self::Input, func: F) -> Self::Output
        where
            F: Fn(Self::Input) -> Self::Output;
}
