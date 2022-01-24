use std::marker::PhantomData;

#[derive(Clone)]
pub struct Lazy<Value, Thunk>(LazyEnum<Value, Thunk>);

impl<Value, Thunk> Lazy<Value, Thunk> {
    pub fn value(v: Value) -> Self {
        Self(LazyEnum::Value(v, PhantomData))
    }

    // TODO: Enable dead code warnings
    #[allow(dead_code)]
    pub fn thunk(t: Thunk) -> Self {
        Self(LazyEnum::Thunk(t))
    }

    pub fn as_ref(&self) -> Lazy<&Value, &Thunk> {
        match &self.0 {
            LazyEnum::Value(value, _) => Lazy(LazyEnum::Value(value, PhantomData)),
            LazyEnum::Thunk(thunk) => Lazy(LazyEnum::Thunk(thunk)),
        }
    }

    pub fn as_mut(&mut self) -> Lazy<&mut Value, &mut Thunk> {
        match &mut self.0 {
            LazyEnum::Value(value, _) => Lazy(LazyEnum::Value(value, PhantomData)),
            LazyEnum::Thunk(thunk) => Lazy(LazyEnum::Thunk(thunk)),
        }
    }
}

impl<Value, Thunk: Into<Value>> Lazy<Value, Thunk> {
    pub fn eval(self) -> Value {
        self.0.eval()
    }
}

#[derive(Clone)]
enum LazyEnum<Value, Thunk> {
    Value(Value, PhantomData<Thunk>),
    // TODO: Enable dead code warnings
    #[allow(dead_code)]
    Thunk(Thunk),
}

impl<Value, Thunk: Into<Value>> LazyEnum<Value, Thunk> {
    fn eval(self) -> Value {
        match self {
            LazyEnum::Value(v, _) => v,
            LazyEnum::Thunk(node) => node.into(),
        }
    }
}

pub fn map1<XValue, XThunk, Args, ValueResult, ThunkResult>(
    x: Lazy<XValue, XThunk>,
    args: Args,
    f_value: impl FnOnce(XValue, Args) -> ValueResult,
    f_thunk: impl FnOnce(XThunk, Args) -> ThunkResult,
) -> Lazy<ValueResult, ThunkResult> {
    Lazy(match x.0 {
        LazyEnum::Value(x, _) => LazyEnum::Value(f_value(x, args), PhantomData),
        LazyEnum::Thunk(x) => LazyEnum::Thunk(f_thunk(x, args)),
    })
}

pub fn call2<XValue, XThunk: Into<XValue>, YValue, YThunk: Into<YValue>>(
    x: Lazy<XValue, XThunk>,
    y: Lazy<YValue, YThunk>,
    f_value: impl FnOnce(XValue, YValue),
    f_thunk: impl FnOnce(XThunk, YThunk),
) {
    match (x.0, y.0) {
        (LazyEnum::Value(x, _), LazyEnum::Value(y, _)) => f_value(x, y),
        (LazyEnum::Thunk(x), LazyEnum::Thunk(y)) => f_thunk(x, y),
        (x, y) => f_value(x.eval(), y.eval()),
    }
}

pub fn call3<
    XValue,
    XThunk: Into<XValue>,
    YValue,
    YThunk: Into<YValue>,
    ZValue,
    ZThunk: Into<ZValue>,
>(
    x: Lazy<XValue, XThunk>,
    y: Lazy<YValue, YThunk>,
    z: Lazy<ZValue, ZThunk>,
    f_value: impl FnOnce(XValue, YValue, ZValue),
    f_thunk: impl FnOnce(XThunk, YThunk, ZThunk),
) {
    match (x.0, y.0, z.0) {
        (LazyEnum::Value(x, _), LazyEnum::Value(y, _), LazyEnum::Value(z, _)) => f_value(x, y, z),
        (LazyEnum::Thunk(x), LazyEnum::Thunk(y), LazyEnum::Thunk(z)) => f_thunk(x, y, z),
        (x, y, z) => f_value(x.eval(), y.eval(), z.eval()),
    }
}
