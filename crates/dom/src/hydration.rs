pub mod dom;

#[cfg(any(
    feature = "hydration",
    all(feature = "client-side-render", feature = "server-side-render")
))]
mod select_impl {
    use super::IsDry;

    pub struct Hydration<Wet, Dry>(HydrationEnum<Wet, Dry>);

    pub enum HydrationEnum<Wet, Dry> {
        Wet(Wet),
        Dry(Option<Dry>),
    }

    impl<Wet, Dry> Hydration<Wet, Dry> {
        pub fn new(_wet: impl FnOnce() -> Wet, dry: impl FnOnce() -> Dry) -> Self {
            Self(HydrationEnum::Dry(Some(dry())))
        }
    }

    impl<Wet, Dry: Into<Wet>> Hydration<Wet, Dry> {
        pub fn wet(&mut self) -> &mut Wet {
            self.wet_with(Dry::into)
        }

        pub fn wet_with(&mut self, f: impl FnOnce(Dry) -> Wet) -> &mut Wet {
            self.hydrate(f);

            match &mut self.0 {
                HydrationEnum::Wet(wet) => wet,
                HydrationEnum::Dry(_) => unreachable!(),
            }
        }

        pub fn map<Arg, R>(
            &mut self,
            arg: Arg,
            f_dry: impl FnOnce(&mut Dry, Arg) -> R,
            f_wet: impl FnOnce(&mut Wet, Arg) -> R,
        ) -> R {
            match &mut self.0 {
                HydrationEnum::Wet(wet) => f_wet(wet, arg),
                HydrationEnum::Dry(dry) => f_dry(dry.as_mut().unwrap(), arg),
            }
        }

        pub fn map1<T: IsDry>(
            &mut self,
            arg: T,
            f_dry: impl FnOnce(&mut Dry, T),
            f_wet: impl FnOnce(&mut Wet, T),
        ) {
            if all_dry([self, &arg]) {
                f_dry(self.dry(), arg);
            } else {
                f_wet(self.wet(), arg);
            }
        }

        pub fn map2<T: IsDry, U: IsDry>(
            &mut self,
            arg0: T,
            arg1: U,
            f_dry: impl FnOnce(&mut Dry, T, U),
            f_wet: impl FnOnce(&mut Wet, T, U),
        ) {
            if all_dry([self, &arg0, &arg1]) {
                f_dry(self.dry(), arg0, arg1);
            } else {
                f_wet(self.wet(), arg0, arg1);
            }
        }

        fn hydrate(&mut self, f: impl FnOnce(Dry) -> Wet) {
            let self_enum = &mut self.0;
            *self_enum = HydrationEnum::<Wet, Dry>::Wet(match self_enum {
                HydrationEnum::Wet(_) => return,
                HydrationEnum::Dry(dry) => f(dry.take().unwrap()),
            });
        }

        fn dry(&mut self) -> &mut Dry {
            match &mut self.0 {
                HydrationEnum::Wet(_) => panic!("Expected a dry item"),
                HydrationEnum::Dry(dry) => return dry.as_mut().unwrap(),
            }
        }
    }

    impl<Wet, Dry> IsDry for Hydration<Wet, Dry> {
        fn is_dry(&self) -> bool {
            match &self.0 {
                HydrationEnum::Wet(_) => false,
                HydrationEnum::Dry(_) => true,
            }
        }
    }

    fn all_dry<const COUNT: usize>(args: [&dyn IsDry; COUNT]) -> bool {
        args.into_iter().all(IsDry::is_dry)
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

    use super::IsDry;

    pub struct Hydration<Wet, Dry> {
        wet: Wet,
        phantom: PhantomData<Dry>,
    }

    impl<Wet, Dry> Hydration<Wet, Dry> {
        pub fn new(wet: impl FnOnce() -> Wet, _dry: impl FnOnce() -> Dry) -> Self {
            Self {
                wet: wet(),
                phantom: PhantomData,
            }
        }
    }

    impl<Wet, Dry: Into<Wet>> Hydration<Wet, Dry> {
        pub fn wet(&mut self) -> &mut Wet {
            &mut self.wet
        }

        pub fn wet_with(&mut self, _f: impl FnOnce(Dry) -> Wet) -> &mut Wet {
            self.wet()
        }

        pub fn map<Arg, R>(
            &mut self,
            arg: Arg,
            _f_dry: impl FnOnce(&mut Dry, Arg) -> R,
            f_wet: impl FnOnce(&mut Wet, Arg) -> R,
        ) -> R {
            f_wet(&mut self.wet, arg)
        }

        pub fn map1<T: IsDry>(
            &mut self,
            arg: T,
            _f_dry: impl FnOnce(&mut Dry, T),
            f_wet: impl FnOnce(&mut Wet, T),
        ) {
            f_wet(&mut self.wet, arg)
        }

        pub fn map2<T: IsDry, U: IsDry>(
            &mut self,
            arg0: T,
            arg1: U,
            _f_dry: impl FnOnce(&mut Dry, T, U),
            f_wet: impl FnOnce(&mut Wet, T, U),
        ) {
            f_wet(&mut self.wet, arg0, arg1)
        }
    }

    impl<Wet, Dry> IsDry for Hydration<Wet, Dry> {
        fn is_dry(&self) -> bool {
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

    use super::IsDry;

    pub struct Hydration<Wet, Dry> {
        dry: Dry,
        wet: PhantomData<Wet>,
    }

    impl<Wet, Dry> Hydration<Wet, Dry> {
        pub fn new(_wet: impl FnOnce() -> Wet, dry: impl FnOnce() -> Dry) -> Self {
            Self {
                dry: dry(),
                wet: PhantomData,
            }
        }
    }

    impl<Wet, Dry: Into<Wet>> Hydration<Wet, Dry> {
        pub fn wet(&mut self) -> &mut Wet {
            self.wet_with(Dry::into)
        }

        pub fn wet_with(&mut self, _f: impl FnOnce(Dry) -> Wet) -> &mut Wet {
            panic!("Build is configured for dry items only")
        }

        pub fn map<Arg, R>(
            &mut self,
            arg: Arg,
            f_dry: impl FnOnce(&mut Dry, Arg) -> R,
            _f_wet: impl FnOnce(&mut Wet, Arg) -> R,
        ) -> R {
            f_dry(&mut self.dry, arg)
        }

        pub fn map1<T: IsDry>(
            &mut self,
            arg: T,
            f_dry: impl FnOnce(&mut Dry, T),
            _f_wet: impl FnOnce(&mut Wet, T),
        ) {
            f_dry(&mut self.dry, arg)
        }

        pub fn map2<T: IsDry, U: IsDry>(
            &mut self,
            arg0: T,
            arg1: U,
            f_dry: impl FnOnce(&mut Dry, T, U),
            _f_wet: impl FnOnce(&mut Wet, T, U),
        ) {
            f_dry(&mut self.dry, arg0, arg1)
        }
    }

    impl<Wet, Dry> IsDry for Hydration<Wet, Dry> {
        fn is_dry(&self) -> bool {
            true
        }
    }
}

pub use select_impl::Hydration;

// TODO: Typically, we'd check if `is_dry`, `evaluate` if needed and pass the
// arg on to a function. Each of these will borrow for Rc types. Can we find a
// way around this? Maybe a `Borrowed` type on the `DomNode` trait?
pub trait IsDry {
    fn is_dry(&self) -> bool;
}

impl<'a, T: IsDry> IsDry for &'a T {
    fn is_dry(&self) -> bool {
        T::is_dry(self)
    }
}

impl<'a, T: IsDry> IsDry for &'a mut T {
    fn is_dry(&self) -> bool {
        T::is_dry(self)
    }
}

impl<T: IsDry> IsDry for Option<T> {
    fn is_dry(&self) -> bool {
        if let Some(x) = self {
            x.is_dry()
        } else {
            true
        }
    }
}
