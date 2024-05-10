// class Functor f where
//fmap :: (a -> b) -> f a -> f b

trait FunctorSuper {}

trait Functor {
    type Kind<T>: Functor;
    type Item;
    type Result<U>: Functor<Kind<U> = Self::Kind<U>, Item = U>;

    fn new(x: Self::Item) -> Self;
    fn map<U>(&self, m: fn(&Self::Item) -> U) -> Self::Result<U>;
}

#[derive(Debug, PartialEq)]
struct Identity<T>(T);

impl<T> Functor for Identity<T> {
    type Kind<U> = Identity<U>;
    type Item = T;
    type Result<U> = Identity<U>;

    fn new(x: Self::Item) -> Self {
        Identity(x)
    }

    fn map<V>(&self, m: fn(&Self::Item) -> V) -> Self::Result<V> {
        let Identity(x) = self;
        let y = m(x);
        Identity(y)
    }
}

#[derive(Debug, PartialEq)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Functor for Maybe<T> {
    type Kind<U> = Maybe<U>;
    type Item = T;
    type Result<U> = Maybe<U>;

    fn new(x: Self::Item) -> Self {
        Maybe::Just(x)
    }

    fn map<V>(&self, m: fn(&Self::Item) -> V) -> Self::Result<V> {
        match self {
            Maybe::Nothing => Maybe::Nothing,
            Maybe::Just(x) => Maybe::Just(m(x)),
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
        let y = x.map(map);
        assert_eq!(y, Identity(2));
    }

    #[test]
    fn test_identity_different_type() {
        let map = |x: &i32| x.to_string();
        let x = Identity(1);
        let y = x.map(map);
        assert_eq!(y, Identity("1".to_string()));
    }

    #[test]
    fn test_maybe_nothing() {
        let map = |x: &i32| x + 1;
        let x = Maybe::Nothing;
        let y = x.map(map);
        assert_eq!(y, Maybe::Nothing);
    }

    #[test]
    fn test_maybe_just() {
        let map = |x: &i32| x + 1;
        let x = Maybe::Just(1);
        let y = x.map(map);
        assert_eq!(y, Maybe::Just(2));
    }

    #[test]
    fn test_maybe_nothing_different_type() {
        let map = |x: &i32| x.to_string();
        let x = Maybe::Nothing;
        let y = x.map(map);
        assert_eq!(y, Maybe::Nothing);
    }

    #[test]
    fn test_maybe_just_different_type() {
        let map = |x: &i32| x.to_string();
        let x = Maybe::Just(1);
        let y = x.map(map);
        assert_eq!(y, Maybe::Just("1".to_string()));
    }
}
