#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! unreach {
    () => ({
        unsafe {
            ::std::hint::unreachable_unchecked();
        }
    })
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! unreach {
    () => ({
        unreachable!()
    })
}

pub trait AssignOptionIf where Self: Sized {
    #[inline(always)]
    fn assign_if(&mut self, new_value: Option<Self>) {
        if let Some(new_value) = new_value {
            *self = new_value;
        }
    }
}

impl<T: Sized> AssignOptionIf for T {}
