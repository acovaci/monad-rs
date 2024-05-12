// class Functor f where
// fmap :: (a -> b) -> f a -> f b

pub trait Functor<T> {
    type Kind<U>: Functor<U>;

    fn new(value: T) -> Self;
    fn map<'a, U>(&'a self, f: fn(&'a T) -> U) -> <Self as Functor<T>>::Kind<U>;
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

        impl<L, R: Clone> Functor<L> for Either<L, R> {
            type Kind<U> = Either<U, R>;

            fn new(value: L) -> Self {
                Either::Left(value)
            }

            fn map<'a, U>(&'a self, f: fn(&'a L) -> U) -> Either<U, R> {
                match self {
                    Either::Left(value) => Either::Left(f(value)),
                    Either::Right(value) => Either::Right(value.clone()),
                }
            }
        }

        #[test]
        fn test_left_same_type() {
            let either = Either::<_, ()>::Left(42);
            let result = either.map(|x| x + 1);
            assert_eq!(result, Either::Left(43i32));
        }

        #[test]
        fn test_right_same_type() {
            let either = Either::Right(42);
            let result = either.map(|x| x + 1);
            assert_eq!(result, Either::Right(42));
        }

        #[test]
        fn test_left_different_type() {
            let either = Either::<_, ()>::Left(42);
            let result = either.map(|x| x.to_string());
            assert_eq!(result, Either::Left("42".to_string()));
        }

        #[test]
        fn test_right_different_type() {
            let either = Either::<i32, _>::Right(42);
            let result = either.map(|x| x.to_string());
            assert_eq!(result, Either::Right(42));
        }
    }
}
