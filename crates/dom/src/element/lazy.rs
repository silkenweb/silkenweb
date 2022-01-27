use std::marker::PhantomData;

pub enum Lazy<Value, Thunk> {
    Value(Value, PhantomData<Thunk>),
    // TODO: feature to disable this at compile time
    #[allow(dead_code)]
    Thunk(Option<Thunk>),
}

impl<Value, Thunk> Lazy<Value, Thunk> {
    #[allow(dead_code)]
    pub fn new_value(x: Value) -> Self {
        Self::Value(x, PhantomData)
    }

    pub fn new_thunk(x: Thunk) -> Self {
        Self::Thunk(Some(x))
    }
}

impl<Value, Thunk: Into<Value>> Lazy<Value, Thunk> {
    pub fn value(&mut self) -> &mut Value {
        self.value_with(Thunk::into)
    }

    pub fn value_with(&mut self, f: impl FnOnce(Thunk) -> Value) -> &mut Value {
        *self = Self::Value(
            match self {
                Lazy::Value(value, _) => return value,
                Lazy::Thunk(thunk) => f(thunk.take().unwrap()),
            },
            PhantomData,
        );

        match self {
            Lazy::Value(value, _) => value,
            Lazy::Thunk(_) => unreachable!(),
        }
    }

    pub fn map<Arg, R>(
        &mut self,
        arg: Arg,
        f_virt: impl FnOnce(&mut Thunk, Arg) -> R,
        f_real: impl FnOnce(&mut Value, Arg) -> R,
    ) -> R {
        match self {
            Lazy::Value(value, _) => f_real(value, arg),
            Lazy::Thunk(thunk) => f_virt(thunk.as_mut().unwrap(), arg),
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

    fn thunk(&mut self) -> &mut Thunk {
        match self {
            Lazy::Value(_, _) => panic!("Expected a thunk"),
            Lazy::Thunk(thunk) => return thunk.as_mut().unwrap(),
        }
    }
}

impl<Value, Thunk> IsThunk for Lazy<Value, Thunk> {
    fn is_thunk(&self) -> bool {
        match self {
            Lazy::Value(_, _) => false,
            Lazy::Thunk(_) => true,
        }
    }
}

// TODO: Typically, we'd check if `is_thunk`, `evaluate` if needed and pass the
// arg on to a function. Each of these will borrow for Rc types. Can we find a
// way around this? Maybe a `Borrowed` type on the `DomNode` trait?
pub trait IsThunk {
    fn is_thunk(&self) -> bool;
}

pub fn all_thunks<const COUNT: usize>(args: [&dyn IsThunk; COUNT]) -> bool {
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
