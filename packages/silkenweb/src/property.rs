// TODO: Module docs

// TODO: Docs
pub trait AsProperty {
    type Type<'a>
    where
        Self: 'a;

    fn as_property(&self) -> Self::Type<'_>;
}

impl<'a> AsProperty for &'a str {
    type Type<'b> = &'b str where 'a: 'b;

    fn as_property(&self) -> Self::Type<'_> {
        self
    }
}

impl AsProperty for String {
    type Type<'a> = &'a str;

    fn as_property(&self) -> Self::Type<'_> {
        self.as_ref()
    }
}

macro_rules! as_property{
    ($($typ:ty),*) => {
        $(
            impl AsProperty for $typ {
                type Type<'a> = Self;

                fn as_property(&self) -> Self::Type<'_> {
                    *self
                }
            }
        )*
    }
}

as_property!(bool);
as_property!(i8, i16, i32, i64);
as_property!(u8, u16, u32, u64);
as_property!(f32, f64);
as_property!(usize);
