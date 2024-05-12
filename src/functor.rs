// class Functor f where
//fmap :: (a -> b) -> f a -> f b

pub trait MultiFunctor<T, U> {
    type Generic<V>;
    // where
    //     Self: MultiFunctor<T, U, Generic<T> = U>;
    type Kind<V>: MultiFunctor<
        V,
        Self::Generic<V>,
        Generic<V> = Self::Generic<V>,
        Kind<V> = Self::Kind<V>,
    >;

    fn multi_new(value: T) -> Self;
    fn multi_map<'a, V>(&'a self, f: fn(&'a T) -> V) -> Self::Kind<V>;
}

pub trait Functor<T>: MultiFunctor<T, T> {
    type Kind<U>: Functor<U>;

    fn new(value: T) -> Self;
    fn map<'a, U>(&'a self, f: fn(&'a T) -> U) -> <Self as Functor<T>>::Kind<U>;
}

impl<T, U> MultiFunctor<U, U> for T
where
    T: Functor<U>,
{
    type Generic<V> = ((V,), V);
    type Kind<V> = <T as MultiFunctor<U, U>>::Kind<V>;

    fn multi_new(value: U) -> Self {
        Functor::<U>::new(value)
    }

    fn multi_map<'a, V>(&'a self, f: fn(&'a U) -> V) -> Self::Kind<V> {
        MultiFunctor::<U, U>::multi_map::<V>(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod identity {
        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        struct Identity<T> {
            value: T,
        }

        impl<T> Functor<T> for Identity<T> {
            type Kind<U> = Identity<U>;

            fn new(value: T) -> Self {
                Identity { value }
            }

            fn map<'a, U>(&'a self, f: fn(&'a T) -> U) -> Identity<U> {
                Identity::new(f(&self.value))
            }
        }

        #[test]
        fn test_same_type() {
            let identity = Identity::new(42);
            let result = identity.map(|x| x + 1);
            assert_eq!(result, Identity::new(43i32));
        }

        #[test]
        fn test_different_type() {
            let identity = Identity::new(42);
            let result = identity.map(|x| x.to_string());
            assert_eq!(result, Identity::new("42".to_string()));
        }
    }

    mod maybe {
        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        enum Maybe<T> {
            Just(T),
            Nothing,
        }

        impl<T> Functor<T> for Maybe<T> {
            type Kind<U> = Maybe<U>;

            fn new(value: T) -> Self {
                Maybe::Just(value)
            }

            fn map<'a, U>(&'a self, f: fn(&'a T) -> U) -> Maybe<U> {
                match self {
                    Maybe::Just(value) => Maybe::Just(f(value)),
                    Maybe::Nothing => Maybe::Nothing,
                }
            }
        }

        #[test]
        fn test_just_same_type() {
            let maybe = Maybe::Just(42);
            let result = maybe.map(|x| x + 1);
            assert_eq!(result, Maybe::Just(43i32));
        }

        #[test]
        fn test_nothing_same_type() {
            let maybe = Maybe::Nothing;
            let result = maybe.map(|x: &i32| x + 1);
            assert_eq!(result, Maybe::Nothing);
        }

        #[test]
        fn test_just_different_type() {
            let maybe = Maybe::Just(42);
            let result = maybe.map(|x| x.to_string());
            assert_eq!(result, Maybe::Just("42".to_string()));
        }

        #[test]
        fn test_nothing_different_type() {
            let maybe = Maybe::Nothing;
            let result = maybe.map(|x: &i32| x.to_string());
            assert_eq!(result, Maybe::Nothing);
        }
    }

    mod either {
        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        enum Either<L, R> {
            Left(L),
            Right(R),
        }

        impl<L, R> MultiFunctor<L, ((L, R), L)> for Either<L, R>
        where
            R: Clone,
        {
            type Generic<V> = ((V, R), V);
            type Kind<V> = Either<V, R>;

            fn multi_new(value: L) -> Self {
                Either::Left(value)
            }

            fn multi_map<'a, V>(&'a self, f: fn(&'a L) -> V) -> Self::Kind<V> {
                match self {
                    Either::Left(value) => Either::Left(f(value)),
                    Either::Right(value) => Either::Right(value.clone()), // or should we deref? or should Kind<V> = Either<V, &R>?
                }
            }
        }

        impl<L, R> MultiFunctor<R, ((L, R), L, R)> for Either<L, R>
        where
            L: Clone,
        {
            type Generic<V> = ((L, V), L, V);
            type Kind<V> = Either<L, V>;

            fn multi_new(value: R) -> Self {
                Either::Right(value)
            }

            fn multi_map<'a, V>(&'a self, f: fn(&'a R) -> V) -> Self::Kind<V> {
                match self {
                    Either::Left(value) => Either::Left(value.clone()), // or should we deref? or should Kind<V> = Either<&L, V>?
                    Either::Right(value) => Either::Right(f(value)),
                }
            }
        }

        #[test]
        fn test_left_functor_left_same_type() {
            let either = Either::<i32, i32>::Left(42);
            let result = MultiFunctor::<i32, ((i32, i32), i32)>::multi_map(&either, |x| x + 1);
            assert_eq!(result, Either::Left(43i32));
        }
    }
}
