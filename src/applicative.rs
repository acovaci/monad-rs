// class Applicative f where
//  pure :: a -> f a
//  (<*>) :: f (a -> b) -> f a -> f b

pub trait Applicative<T> {
    type Kind<U>: Applicative<U>;

    fn new(value: T) -> Self;
    fn apply<'a, U>(&'a self, f: Self::Kind<impl Fn(&'a T) -> U>) -> Self::Kind<U>
    where
        T: 'a;
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

        impl<T> Applicative<T> for Identity<T> {
            type Kind<U> = Identity<U>;

            fn new(value: T) -> Self {
                Identity { value }
            }

            fn apply<'a, U>(&'a self, f: Identity<impl Fn(&'a T) -> U>) -> Self::Kind<U> {
                let Identity { value: func } = f;
                Identity {
                    value: func(&self.value),
                }
            }
        }

        #[test]
        fn test_same_type() {
            let x = Identity::new(1);
            let f = Identity::new(|x| x + 1);
            let y = x.apply(f);
            assert_eq!(y, Identity::new(2));
        }

        #[test]
        fn test_different_type() {
            let x = Identity::new(1);
            let f = Identity::new(|x: &i32| x.to_string());
            let y = x.apply(f);
            assert_eq!(y, Identity::new("1".to_string()));
        }
    }

    mod maybe {
        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        enum Maybe<T> {
            Just(T),
            Nothing,
        }

        impl<T> Applicative<T> for Maybe<T> {
            type Kind<U> = Maybe<U>;

            fn new(value: T) -> Self {
                Maybe::Just(value)
            }

            fn apply<'a, U>(&'a self, f: Self::Kind<impl Fn(&'a T) -> U>) -> Self::Kind<U> {
                match self {
                    Maybe::Just(value) => match f {
                        Maybe::Just(func) => Maybe::Just(func(value)),
                        Maybe::Nothing => Maybe::Nothing,
                    },
                    Maybe::Nothing => Maybe::Nothing,
                }
            }
        }

        #[test]
        fn test_just_same_type() {
            let x = Maybe::Just(1);
            let f = Maybe::Just(|x| x + 1);
            let y = x.apply(f);
            assert_eq!(y, Maybe::Just(2));
        }

        #[test]
        fn test_nothing_same_type() {
            let x = Maybe::Nothing;
            let f = Maybe::Just(|x| x + 1);
            let y = x.apply(f);
            assert_eq!(y, Maybe::Nothing);
        }

        #[test]
        fn test_just_different_type() {
            let x = Maybe::Just(1);
            let f = Maybe::Just(|x: &i32| x.to_string());
            let y = x.apply(f);
            assert_eq!(y, Maybe::Just("1".to_string()));
        }

        #[test]
        fn test_nothing_different_type() {
            let x = Maybe::Nothing;
            let f = Maybe::Just(|x: &i32| x.to_string());
            let y = x.apply(f);
            assert_eq!(y, Maybe::Nothing);
        }
    }

    mod either {
        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        enum Either<L, R> {
            Left(L),
            Right(R),
        }

        impl<T, U> Applicative<T> for Either<T, U>
        where
            U: Clone,
        {
            type Kind<V> = Either<V, U>;

            fn new(value: T) -> Self {
                Either::Left(value)
            }

            fn apply<'a, V>(&'a self, f: Self::Kind<impl Fn(&'a T) -> V>) -> Self::Kind<V> {
                match self {
                    Either::Left(value) => match f {
                        Either::Left(func) => Either::Left(func(value)),
                        Either::Right(value) => Either::Right(value.clone()),
                    },
                    Either::Right(value) => Either::Right(value.clone()),
                }
            }
        }

        #[test]
        fn test_left_left_same_type() {
            let x = Either::<_, ()>::Left(1);
            let f = Either::Left(|x| x + 1);
            let y = x.apply(f);
            assert_eq!(y, Either::Left(2));
        }

        #[test]
        fn test_left_right_same_type() {
            let x = Either::Left(1);
            let f = Either::<fn(&i32) -> i32, _>::Right(2);
            let y = x.apply(f);
            assert_eq!(y, Either::Right(2));
        }

        #[test]
        fn test_right_left_same_type() {
            let x = Either::Right(1);
            let f = Either::Left(|x| x + 1);
            let y = x.apply(f);
            assert_eq!(y, Either::Right(1));
        }

        #[test]
        fn test_right_right_same_type() {
            let x = Either::<(), _>::Right(1);
            let f = Either::<fn(&()) -> (), _>::Right(2);
            let y = x.apply(f);
            assert_eq!(y, Either::Right(1));
        }

        #[test]
        fn test_left_left_different_type() {
            let x = Either::<_, ()>::Left(1);
            let f = Either::Left(|x: &i32| x.to_string());
            let y = x.apply(f);
            assert_eq!(y, Either::Left("1".to_string()));
        }

        #[test]
        fn test_left_right_different_type() {
            let x = Either::Left(1);
            let f = Either::<fn(&i32) -> &str, _>::Right("2");
            let y = x.apply(f);
            assert_eq!(y, Either::Right("2"));
        }

        #[test]
        fn test_right_left_different_type() {
            let x = Either::Right(1);
            let f = Either::Left(|x: &i32| x.to_string());
            let y = x.apply(f);
            assert_eq!(y, Either::Right(1));
        }

        #[test]
        fn test_right_right_different_type() {
            let x = Either::<(), _>::Right(1);
            let f = Either::<fn(&()) -> (), _>::Right(2);
            let y = x.apply(f);
            assert_eq!(y, Either::Right(1));
        }
    }
}
