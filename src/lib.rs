#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::marker::PhantomData;

// TODO: This should not be private
pub struct FnCompose<Args, F1: FnOnce<Args>, F2: FnOnce<(F1::Output,)>> {
    first_closure: F1,
    second_closure: F2,
    _args: PhantomData<Args>,
}

impl<Args, F1: FnOnce<Args>, F2: FnOnce<(F1::Output,)>> FnOnce<Args> for FnCompose<Args, F1, F2> {
    type Output = F2::Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        (self.second_closure)(self.first_closure.call_once(args))
    }
}

// All closures are Sized but the Fn* traits don't depend on it
pub trait FnOnceThen<Args>: FnOnce<Args> + Sized {
    // TODO: This should return a generic type with the help of GAT
    fn then_once<F>(self, f: F) -> FnCompose<Args, Self, F>
    where
        F: FnOnce<(Self::Output,)>,
    {
        FnCompose {
            first_closure: self,
            second_closure: f,
            _args: PhantomData,
        }
    }
}

impl<Args, T: FnOnce<Args>> FnOnceThen<Args> for T {}

#[cfg(test)]
mod tests {
    use crate::FnOnceThen;

    #[test]
    fn fn_once_compose_sum_and_tostring() {
        let sum2 = |a| a + 2;
        // FIXME: For some reason the compiler can't infer the input type of the closure
        let sum2_then_tostring = sum2.then_once(|x: i32| x.to_string());
        assert_eq!(sum2(3).to_string(), sum2_then_tostring(3));
    }
}
