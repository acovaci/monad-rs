// class Functor f where
//fmap :: (a -> b) -> f a -> f b

// class Applicative f where
//  pure :: a -> f a
//  (<*>) :: f (a -> b) -> f a -> f b

// class Monad m where
//  return :: a -> m a
//  (>>=) :: m a -> (a -> m b) -> m b

pub trait Monad<T> {
    type Kind<U>: Monad<U>;

    fn new(value: T) -> Self;
    fn bind<'a, U>(&'a self, f: impl Fn(&'a T) -> Self::Kind<U>) -> Self::Kind<U>
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

        impl<T> Monad<T> for Identity<T> {
            type Kind<U> = Identity<U>;

            fn new(value: T) -> Self {
                Identity { value }
            }

            fn bind<'a, U>(&'a self, f: impl Fn(&'a T) -> Self::Kind<U>) -> Self::Kind<U> {
                f(&self.value)
            }
        }

        #[test]
        fn test_same_type() {
            let identity = Identity::new(1);
            let identity2 = identity.bind(|x| Identity::new(x + 1));
            assert_eq!(identity2, Identity::new(2));
        }

        #[test]
        fn test_different_type() {
            let identity = Identity::new(1);
            let identity2 = identity.bind(|x: &i32| Identity::new(x.to_string()));
            assert_eq!(identity2, Identity::new("1".to_string()));
        }
    }

    mod maybe {
        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        enum Maybe<T> {
            Just(T),
            Nothing,
        }

        impl<T> Monad<T> for Maybe<T> {
            type Kind<U> = Maybe<U>;

            fn new(value: T) -> Self {
                Maybe::Just(value)
            }

            fn bind<'a, U>(&'a self, f: impl Fn(&'a T) -> Self::Kind<U>) -> Self::Kind<U> {
                match self {
                    Maybe::Just(value) => f(value),
                    Maybe::Nothing => Maybe::Nothing,
                }
            }
        }

        #[test]
        fn test_just_just_same_type() {
            let maybe = Maybe::Just(1);
            let maybe2 = maybe.bind(|x| Maybe::Just(x + 1));
            assert_eq!(maybe2, Maybe::Just(2));
        }

        #[test]
        fn test_nothing_just_same_type() {
            let maybe = Maybe::Nothing;
            let maybe2 = maybe.bind(|x: &i32| Maybe::Just(x + 1));
            assert_eq!(maybe2, Maybe::Nothing);
        }

        #[test]
        fn test_just_nothing_same_type() {
            let maybe = Maybe::Just(1);
            let maybe2 = maybe.bind(|_: &i32| Maybe::<i32>::Nothing);
            assert_eq!(maybe2, Maybe::Nothing);
        }

        #[test]
        fn test_nothing_nothing_same_type() {
            let maybe = Maybe::Nothing;
            let maybe2 = maybe.bind(|_: &i32| Maybe::Just(1));
            assert_eq!(maybe2, Maybe::Nothing);
        }
    }

    mod either {
        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        enum Either<L, R> {
            Left(L),
            Right(R),
        }

        impl<T, U> Monad<T> for Either<T, U>
        where
            U: Clone,
        {
            type Kind<V> = Either<V, U>;

            fn new(value: T) -> Self {
                Either::Left(value)
            }

            fn bind<'a, V>(&'a self, f: impl Fn(&'a T) -> Self::Kind<V>) -> Self::Kind<V> {
                match self {
                    Either::Left(value) => f(value),
                    Either::Right(value) => Either::Right(value.clone()),
                }
            }
        }

        #[test]
        fn test_left_monad_left_left_same_type() {
            let either = Either::<_, i32>::Left(1);
            let either2 = either.bind(|x| Either::<_, i32>::Left(x + 1));
            assert_eq!(either2, Either::Left(2));
        }

        #[test]
        fn test_left_monad_left_right_same_type() {
            let either = Either::<_, i32>::Left(1);
            let either2 = either.bind(|x| Either::Right(x + 1));
            assert_eq!(either2, Either::Left(2));
        }

        #[test]
        fn test_left_monad_right_left_same_type() {
            let either = Either::Right(1);
            let either2 = either.bind(|x| Either::Left(x + 1));
            assert_eq!(either2, Either::Right(1));
        }

        #[test]
        fn test_left_monad_right_right_same_type() {
            let either = Either::Right(1);
            let either2 = either.bind(|x| Either::<i32, _>::Right(x + 1));
            assert_eq!(either2, Either::Right(1));
        }

        #[test]
        fn test_left_monad_left_left_different_type() {
            let either = Either::<_, i32>::Left(1);
            let either2 = either.bind(|x: &i32| Either::Left(x.to_string()));
            assert_eq!(either2, Either::Left("1".to_string()));
        }

        #[test]
        fn test_left_monad_left_right_different_type() {
            let either = Either::Left(1);
            let either2 = either.bind(|x: &i32| Either::Right(x.to_string()));
            assert_eq!(either2, Either::Left("1"));
        }

        #[test]
        fn test_left_monad_right_left_different_type() {
            let either = Either::Right(1);
            let either2 = either.bind(|x: &i32| Either::Left(x.to_string()));
            assert_eq!(either2, Either::Right(1));
        }

        #[test]
        fn test_left_monad_right_right_different_type() {
            let either = Either::Right(1);
            let either2 = either.bind(|_: &i32| Either::<i32, _>::Right(2));
            assert_eq!(either2, Either::Right(1));
        }
    }
}
