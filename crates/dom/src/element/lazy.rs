use std::marker::PhantomData;

pub struct Lazy<Value, Thunk>(LazyEnum<Value, Thunk>);

enum LazyEnum<Value, Thunk> {
    Value(Value, PhantomData<Thunk>),
    // TODO: feature to disable this at compile time
    Thunk(Option<Thunk>),
}

impl<Value, Thunk> Lazy<Value, Thunk> {
    pub fn new(_value: impl FnOnce() -> Value, thunk: impl FnOnce() -> Thunk) -> Self {
        Self(LazyEnum::Thunk(Some(thunk())))
    }
}

impl<Value, Thunk: Into<Value>> Lazy<Value, Thunk> {
    pub fn value(&mut self) -> &mut Value {
        self.value_with(Thunk::into)
    }

    pub fn value_with(&mut self, f: impl FnOnce(Thunk) -> Value) -> &mut Value {
        self.set_value(f);

        match &mut self.0 {
            LazyEnum::Value(value, _) => value,
            LazyEnum::Thunk(_) => unreachable!(),
        }
    }

    pub fn map<Arg, R>(
        &mut self,
        arg: Arg,
        f_virt: impl FnOnce(&mut Thunk, Arg) -> R,
        f_real: impl FnOnce(&mut Value, Arg) -> R,
    ) -> R {
        match &mut self.0 {
            LazyEnum::Value(value, _) => f_real(value, arg),
            LazyEnum::Thunk(thunk) => f_virt(thunk.as_mut().unwrap(), arg),
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
        let lazy_enum = &mut self.0;
        *lazy_enum = LazyEnum::<Value, Thunk>::Value(
            match lazy_enum {
                LazyEnum::Value(_, _) => return,
                LazyEnum::Thunk(thunk) => f(thunk.take().unwrap()),
            },
            PhantomData,
        );
    }

    fn thunk(&mut self) -> &mut Thunk {
        match &mut self.0 {
            LazyEnum::Value(_, _) => panic!("Expected a thunk"),
            LazyEnum::Thunk(thunk) => return thunk.as_mut().unwrap(),
        }
    }
}

impl<Value, Thunk> IsThunk for Lazy<Value, Thunk> {
    fn is_thunk(&self) -> bool {
        match &self.0 {
            LazyEnum::Value(_, _) => false,
            LazyEnum::Thunk(_) => true,
        }
    }
}

// TODO: Typically, we'd check if `is_thunk`, `evaluate` if needed and pass the
// arg on to a function. Each of these will borrow for Rc types. Can we find a
// way around this? Maybe a `Borrowed` type on the `DomNode` trait?
pub trait IsThunk {
    fn is_thunk(&self) -> bool;
}

fn all_thunks<const COUNT: usize>(args: [&dyn IsThunk; COUNT]) -> bool {
    args.into_iter().all(IsThunk::is_thunk)
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
