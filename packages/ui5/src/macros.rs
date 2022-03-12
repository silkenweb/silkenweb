macro_rules! attribute {
    ([$($index:tt),*] $name:ident : $t:ty) => {
        pub fn $name(self, value: impl ::silkenweb::attribute::AsAttribute<$t>) -> Self {
            Self(self.0.$name(value) $(, self.$index)*)
        }

        paste::paste!{
            pub fn [< $name _signal >](self, value: impl ::futures_signals::signal::Signal<Item = $t> + 'static) -> Self {
                Self(self.0.[< $name _signal >](value) $(, self.$index)*)
            }
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
