// class Functor f where
//fmap :: (a -> b) -> f a -> f b

// class Applicative f where
//  pure :: a -> f a
//  (<*>) :: f (a -> b) -> f a -> f b

// class Monad m where
//  return :: a -> m a
//  (>>=) :: m a -> (a -> m b) -> m b

trait MonadSuper {
    type Item;

    fn new(x: Self::Item) -> Self;
}

trait Monad<M>: MonadSuper
where
    M: MonadSuper,
{
    fn bind(&self, f: fn(&<Self as MonadSuper>::Item) -> M) -> M;
}

#[derive(Debug, PartialEq)]
struct Identity<T>(T);

impl<T> MonadSuper for Identity<T> {
    type Item = T;

    fn new(x: Self::Item) -> Self {
        Identity(x)
    }
}

impl<X, Y> Monad<Identity<Y>> for Identity<X> {
    fn bind(&self, f: fn(&X) -> Identity<Y>) -> Identity<Y> {
        let Identity(x) = self;
        f(x)
    }
}

#[derive(Debug, PartialEq)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> MonadSuper for Maybe<T> {
    type Item = T;

    fn new(x: Self::Item) -> Self {
        Maybe::Just(x)
    }
}

impl<X, Y> Monad<Maybe<Y>> for Maybe<X> {
    fn bind(&self, f: fn(&X) -> Maybe<Y>) -> Maybe<Y> {
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
