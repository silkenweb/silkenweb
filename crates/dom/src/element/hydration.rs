#[cfg(any(
    feature = "hydration",
    all(feature = "client-side-render", feature = "server-side-render")
))]
mod select_impl {
    use super::IsThunk;

    pub struct Hydration<Value, Thunk>(HydrationEnum<Value, Thunk>);

    pub enum HydrationEnum<Value, Thunk> {
        Value(Value),
        Thunk(Option<Thunk>),
    }

    impl<Value, Thunk> Hydration<Value, Thunk> {
        pub fn new(_value: impl FnOnce() -> Value, thunk: impl FnOnce() -> Thunk) -> Self {
            Self(HydrationEnum::Thunk(Some(thunk())))
        }
    }

    impl<Value, Thunk: Into<Value>> Hydration<Value, Thunk> {
        pub fn value(&mut self) -> &mut Value {
            self.value_with(Thunk::into)
        }

        pub fn value_with(&mut self, f: impl FnOnce(Thunk) -> Value) -> &mut Value {
            self.set_value(f);

            match &mut self.0 {
                HydrationEnum::Value(value) => value,
                HydrationEnum::Thunk(_) => unreachable!(),
            }
        }

        pub fn map<Arg, R>(
            &mut self,
            arg: Arg,
            f_virt: impl FnOnce(&mut Thunk, Arg) -> R,
            f_real: impl FnOnce(&mut Value, Arg) -> R,
        ) -> R {
            match &mut self.0 {
                HydrationEnum::Value(value) => f_real(value, arg),
                HydrationEnum::Thunk(thunk) => f_virt(thunk.as_mut().unwrap(), arg),
            }
        }

        pub fn map1<T: IsThunk>(
            &mut self,
            arg: T,
            f_virt: impl FnOnce(&mut Thunk, T),
            f_real: impl FnOnce(&mut Value, T),
        ) {
            if all_thunks([self, &arg]) {
                f_virt(self.thunk(), arg);
            } else {
                f_real(self.value(), arg);
            }
        }

        pub fn map2<T: IsThunk, U: IsThunk>(
            &mut self,
            arg0: T,
            arg1: U,
            f_virt: impl FnOnce(&mut Thunk, T, U),
            f_real: impl FnOnce(&mut Value, T, U),
        ) {
            if all_thunks([self, &arg0, &arg1]) {
                f_virt(self.thunk(), arg0, arg1);
            } else {
                f_real(self.value(), arg0, arg1);
            }
        }

        fn set_value(&mut self, f: impl FnOnce(Thunk) -> Value) {
            let self_enum = &mut self.0;
            *self_enum = HydrationEnum::<Value, Thunk>::Value(match self_enum {
                HydrationEnum::Value(_) => return,
                HydrationEnum::Thunk(thunk) => f(thunk.take().unwrap()),
            });
        }

        fn thunk(&mut self) -> &mut Thunk {
            match &mut self.0 {
                HydrationEnum::Value(_) => panic!("Expected a thunk"),
                HydrationEnum::Thunk(thunk) => return thunk.as_mut().unwrap(),
            }
        }
    }

    impl<Value, Thunk> IsThunk for Hydration<Value, Thunk> {
        fn is_thunk(&self) -> bool {
            match &self.0 {
                HydrationEnum::Value(_) => false,
                HydrationEnum::Thunk(_) => true,
            }
        }
    }

    fn all_thunks<const COUNT: usize>(args: [&dyn IsThunk; COUNT]) -> bool {
        args.into_iter().all(IsThunk::is_thunk)
    }
}

#[cfg(any(
    all(
        feature = "client-side-render",
        not(any(feature = "server-side-render", feature = "hydration"))
    ),
    not(any(
        feature = "client-side-render",
        feature = "server-side-render",
        feature = "hydration",
    ))
))]
mod select_impl {
    use std::marker::PhantomData;

    use super::IsThunk;

    pub struct Hydration<Value, Thunk> {
        value: Value,
        phantom: PhantomData<Thunk>,
    }

    impl<Value, Thunk> Hydration<Value, Thunk> {
        pub fn new(value: impl FnOnce() -> Value, _thunk: impl FnOnce() -> Thunk) -> Self {
            Self {
                value: value(),
                phantom: PhantomData,
            }
        }
    }

    impl<Value, Thunk: Into<Value>> Hydration<Value, Thunk> {
        pub fn value(&mut self) -> &mut Value {
            &mut self.value
        }

        pub fn value_with(&mut self, _f: impl FnOnce(Thunk) -> Value) -> &mut Value {
            self.value()
        }

        pub fn map<Arg, R>(
            &mut self,
            arg: Arg,
            _f_virt: impl FnOnce(&mut Thunk, Arg) -> R,
            f_real: impl FnOnce(&mut Value, Arg) -> R,
        ) -> R {
            f_real(&mut self.value, arg)
        }

        pub fn map1<T: IsThunk>(
            &mut self,
            arg: T,
            _f_virt: impl FnOnce(&mut Thunk, T),
            f_real: impl FnOnce(&mut Value, T),
        ) {
            f_real(&mut self.value, arg)
        }

        pub fn map2<T: IsThunk, U: IsThunk>(
            &mut self,
            arg0: T,
            arg1: U,
            _f_virt: impl FnOnce(&mut Thunk, T, U),
            f_real: impl FnOnce(&mut Value, T, U),
        ) {
            f_real(&mut self.value, arg0, arg1)
        }
    }

    impl<Value, Thunk> IsThunk for Hydration<Value, Thunk> {
        fn is_thunk(&self) -> bool {
            false
        }
    }
}

#[cfg(all(
    feature = "server-side-render",
    not(any(feature = "client-side-render", feature = "hydration")),
))]
mod select_impl {
    use std::marker::PhantomData;

    use super::IsThunk;

    pub struct Hydration<Value, Thunk> {
        thunk: Thunk,
        value: PhantomData<Value>,
    }

    impl<Value, Thunk> Hydration<Value, Thunk> {
        pub fn new(_value: impl FnOnce() -> Value, thunk: impl FnOnce() -> Thunk) -> Self {
            Self {
                thunk: thunk(),
                value: PhantomData,
            }
        }
    }

    impl<Value, Thunk: Into<Value>> Hydration<Value, Thunk> {
        pub fn value(&mut self) -> &mut Value {
            self.value_with(Thunk::into)
        }

        pub fn value_with(&mut self, _f: impl FnOnce(Thunk) -> Value) -> &mut Value {
            panic!("Build is configured for thunks only")
        }

        pub fn map<Arg, R>(
            &mut self,
            arg: Arg,
            f_virt: impl FnOnce(&mut Thunk, Arg) -> R,
            _f_real: impl FnOnce(&mut Value, Arg) -> R,
        ) -> R {
            f_virt(&mut self.thunk, arg)
        }

        pub fn map1<T: IsThunk>(
            &mut self,
            arg: T,
            f_virt: impl FnOnce(&mut Thunk, T),
            _f_real: impl FnOnce(&mut Value, T),
        ) {
            f_virt(&mut self.thunk, arg)
        }

        pub fn map2<T: IsThunk, U: IsThunk>(
            &mut self,
            arg0: T,
            arg1: U,
            f_virt: impl FnOnce(&mut Thunk, T, U),
            _f_real: impl FnOnce(&mut Value, T, U),
        ) {
            f_virt(&mut self.thunk, arg0, arg1)
        }
    }

    impl<Value, Thunk> IsThunk for Hydration<Value, Thunk> {
        fn is_thunk(&self) -> bool {
            true
        }
    }
}

pub use select_impl::Hydration;

// TODO: Typically, we'd check if `is_thunk`, `evaluate` if needed and pass the
// arg on to a function. Each of these will borrow for Rc types. Can we find a
// way around this? Maybe a `Borrowed` type on the `DomNode` trait?
pub trait IsThunk {
    fn is_thunk(&self) -> bool;
}

impl<'a, T: IsThunk> IsThunk for &'a T {
    fn is_thunk(&self) -> bool {
        T::is_thunk(self)
    }
}

impl<'a, T: IsThunk> IsThunk for &'a mut T {
    fn is_thunk(&self) -> bool {
        T::is_thunk(self)
    }
}

impl<T: IsThunk> IsThunk for Option<T> {
    fn is_thunk(&self) -> bool {
        if let Some(x) = self {
            x.is_thunk()
        } else {
            true
        }
    }
}
