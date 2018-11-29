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

///Result extensions
pub trait ResultExt<T, E> {
    ///Returns `Ok` variant assuming that `Err` is unreachable
    fn unreach_err(self) -> T;
    ///Returns `Err` variant assuming that `Ok` is unreachable
    fn unreach_ok(self) -> E;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    #[inline]
    fn unreach_err(self) -> T {
        match self {
            Ok(res) => res,
            Err(_) => unreach!()
        }
    }

    #[inline]
    fn unreach_ok(self) -> E {
        match self {
            Err(res) => res,
            Ok(_) => unreach!()
        }
    }
}
