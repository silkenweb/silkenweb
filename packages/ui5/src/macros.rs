macro_rules! attribute {
    ([$($index:tt),*] $name:ident : $t:ty) => {
        pub fn $name(self, value: impl ::silkenweb::node::element::SignalOrValue<'static, Item = $t>) -> Self {
            Self(self.0.$name(value) $(, self.$index)*)
        }
    };
}

macro_rules! attributes0{
    ($($name:ident : $t:ty),* $(,)?) => {
        $($crate::macros::attribute!{[] $name: $t})*
    }
}

macro_rules! attributes1{
    ($index:tt, $($name:ident : $t:ty),* $(,)?) => {
        $($crate::macros::attribute!{[$index] $name: $t})*
    }
}

pub(crate) use attribute;
pub(crate) use attributes0;
pub(crate) use attributes1;
