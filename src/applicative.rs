// class Functor f where
//fmap :: (a -> b) -> f a -> f b

// class Applicative f where
//  pure :: a -> f a
//  (<*>) :: f (a -> b) -> f a -> f b

trait Applicative {
    type Kind<T>: Applicative;
    type Item;
    type Result<U>: Applicative<Kind<U> = Self::Kind<U>, Item = U>;

    fn new(x: Self::Item) -> Self;
    fn apply<U, F>(&self, a: Self::Result<F>) -> Self::Result<U>
    where
        F: Fn(&Self::Item) -> U;
}

#[derive(Debug, PartialEq)]
struct Identity<T>(T);

impl<T> Applicative for Identity<T> {
    type Kind<U> = Identity<U>;
    type Item = T;
    type Result<U> = Identity<U>;

    fn new(x: Self::Item) -> Self {
        Identity(x)
    }

    fn apply<U, F>(&self, a: Self::Result<F>) -> Self::Result<U>
    where
        F: Fn(&Self::Item) -> U,
    {
        let Identity(x) = self;
        let Identity(f) = a;
        let y = f(x);
        Identity(y)
    }
}

#[derive(Debug, PartialEq)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Applicative for Maybe<T> {
    type Kind<U> = Maybe<U>;
    type Item = T;
    type Result<U> = Maybe<U>;

    fn new(x: Self::Item) -> Self {
        Maybe::Just(x)
    }

    fn apply<U, F>(&self, a: Self::Result<F>) -> Self::Result<U>
    where
        F: Fn(&Self::Item) -> U,
    {
        match self {
            Maybe::Nothing => Maybe::Nothing,
            Maybe::Just(x) => match a {
                Maybe::Nothing => Maybe::Nothing,
                Maybe::Just(f) => Maybe::Just(f(x)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let x = Identity(1);
        let f = |x: &i32| x + 1;
        let a = Identity(f);
        let y = x.apply(a);
        assert_eq!(y, Identity(2));
    }

    #[test]
    fn test_identity_different_types() {
        let x = Identity(1);
        let f = Identity(|x: &i32| x.to_string());
        let y = x.apply(f);
        assert_eq!(y, Identity("1".to_string()));
    }

    #[test]
    fn test_maybe_just() {
        let x = Maybe::Just(1);
        let f = |x: &i32| x + 1;
        let a = Maybe::Just(f);
        let y = x.apply(a);
        assert_eq!(y, Maybe::Just(2));
    }

    #[test]
    fn test_maybe_nothing() {
        let x = Maybe::Nothing;
        let f = |x: &i32| x + 1;
        let a = Maybe::Just(f);
        let y = x.apply(a);
        assert_eq!(y, Maybe::Nothing);
    }

    #[test]
    fn test_maybe_just_different_types() {
        let x = Maybe::Just(1);
        let f = Maybe::Just(|x: &i32| x.to_string());
        let y = x.apply(f);
        assert_eq!(y, Maybe::Just("1".to_string()));
    }

    #[test]
    fn test_maybe_nothing_different_types() {
        let x = Maybe::Nothing;
        let f = Maybe::Just(|x: &i32| x.to_string());
        let y = x.apply(f);
        assert_eq!(y, Maybe::Nothing);
    }
}
