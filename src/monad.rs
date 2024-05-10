// class Functor f where
//fmap :: (a -> b) -> f a -> f b

// class Applicative f where
//  pure :: a -> f a
//  (<*>) :: f (a -> b) -> f a -> f b

// class Monad m where
//  return :: a -> m a
//  (>>=) :: m a -> (a -> m b) -> m b

trait Monad {
    type Kind<T>: Monad;
    type Item;
    type Result<U>: Monad<Kind<U> = Self::Kind<U>, Item = U>;

    fn new(x: Self::Item) -> Self;
    fn bind<U>(&self, f: fn(&Self::Item) -> Self::Result<U>) -> Self::Result<U>;
}

#[derive(Debug, PartialEq)]
struct Identity<T>(T);

impl<T> Monad for Identity<T> {
    type Kind<U> = Identity<U>;
    type Item = T;
    type Result<U> = Identity<U>;

    fn new(x: Self::Item) -> Self {
        Identity(x)
    }

    fn bind<U>(&self, f: fn(&Self::Item) -> Self::Result<U>) -> Self::Result<U> {
        let Identity(x) = self;
        f(x)
    }
}

#[derive(Debug, PartialEq)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Monad for Maybe<T> {
    type Kind<U> = Maybe<U>;
    type Item = T;
    type Result<U> = Maybe<U>;

    fn new(x: Self::Item) -> Self {
        Maybe::Just(x)
    }

    fn bind<U>(&self, f: fn(&Self::Item) -> Self::Result<U>) -> Self::Result<U> {
        match self {
            Maybe::Nothing => Maybe::Nothing,
            Maybe::Just(x) => f(x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let x = Identity::new(1);
        let f = |x: &i32| Identity::new(x + 1);
        let y = x.bind(f);
        assert_eq!(y, Identity::new(2));
    }

    #[test]
    fn test_identity_different_type() {
        let x = Identity::new(1);
        let f = |x: &i32| Identity::new(x.to_string());
        let y = x.bind(f);
        assert_eq!(y, Identity::new("1".to_string()));
    }

    #[test]
    fn test_maybe() {
        let x = Maybe::Just(1);
        let f = |x: &i32| Maybe::Just(x + 1);
        let y = x.bind(f);
        assert_eq!(y, Maybe::Just(2));
    }

    #[test]
    fn test_maybe_nothing() {
        let x = Maybe::Nothing;
        let f = |x: &i32| Maybe::Just(x + 1);
        let y = x.bind(f);
        assert_eq!(y, Maybe::Nothing);
    }

    #[test]
    fn test_maybe_different_type() {
        let x = Maybe::Just(1);
        let f = |x: &i32| Maybe::Just(x.to_string());
        let y = x.bind(f);
        assert_eq!(y, Maybe::Just("1".to_string()));
    }

    #[test]
    fn test_maybe_nothing_different_type() {
        let x = Maybe::Nothing;
        let f = |x: &i32| Maybe::Just(x.to_string());
        let y = x.bind(f);
        assert_eq!(y, Maybe::Nothing);
    }
}
