// class Functor f where
//fmap :: (a -> b) -> f a -> f b

pub trait FunctorSuper {
    type Item;

    fn new(x: Self::Item) -> Self;
}

pub trait Functor<T: FunctorSuper>: FunctorSuper {
    fn apply(&self, f: fn(&<Self as FunctorSuper>::Item) -> T::Item) -> T;
}

#[derive(Debug, PartialEq)]
struct Identity<T>(T);

impl<T> FunctorSuper for Identity<T> {
    type Item = T;

    fn new(x: Self::Item) -> Self {
        Identity(x)
    }
}

impl<X, Y> Functor<Identity<Y>> for Identity<X> {
    fn apply(&self, f: fn(&X) -> Y) -> Identity<Y> {
        let Identity(x) = self;
        let y = f(x);
        Identity::new(y)
    }
}

#[derive(Debug, PartialEq)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> FunctorSuper for Maybe<T> {
    type Item = T;

    fn new(x: Self::Item) -> Self {
        Maybe::Just(x)
    }
}

impl<X, Y> Functor<Maybe<Y>> for Maybe<X> {
    fn apply(&self, f: fn(&X) -> Y) -> Maybe<Y> {
        match self {
            Maybe::Nothing => Maybe::Nothing,
            Maybe::Just(x) => Maybe::Just(f(x)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_same_type() {
        let map = |x: &i32| x + 1;
        let x = Identity(1);
        let y = x.apply(map);
        assert_eq!(y, Identity(2));
    }

    #[test]
    fn test_identity_different_type() {
        let map = |x: &i32| x.to_string();
        let x = Identity(1);
        let y = x.apply(map);
        assert_eq!(y, Identity("1".to_string()));
    }

    #[test]
    fn test_maybe_nothing() {
        let map = |x: &i32| x + 1;
        let x = Maybe::Nothing;
        let y = x.apply(map);
        assert_eq!(y, Maybe::Nothing);
    }

    #[test]
    fn test_maybe_just() {
        let map = |x: &i32| x + 1;
        let x = Maybe::Just(1);
        let y = x.apply(map);
        assert_eq!(y, Maybe::Just(2));
    }

    #[test]
    fn test_maybe_nothing_different_type() {
        let map = |x: &i32| x.to_string();
        let x = Maybe::Nothing;
        let y = x.apply(map);
        assert_eq!(y, Maybe::Nothing);
    }

    #[test]
    fn test_maybe_just_different_type() {
        let map = |x: &i32| x.to_string();
        let x = Maybe::Just(1);
        let y = x.apply(map);
        assert_eq!(y, Maybe::Just("1".to_string()));
    }
}
