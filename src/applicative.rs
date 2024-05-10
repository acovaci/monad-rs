// class Functor f where
//fmap :: (a -> b) -> f a -> f b

// class Applicative f where
//  pure :: a -> f a
//  (<*>) :: f (a -> b) -> f a -> f b

trait ApplicativeSuper {
    type Item;

    fn new(x: Self::Item) -> Self;
}

trait Applicative<T, F, A>: ApplicativeSuper
where
    T: ApplicativeSuper,
    F: Fn(&Self::Item) -> T::Item,
    A: ApplicativeSuper<Item = F>,
{
    fn apply(&self, a: A) -> T;
}

#[derive(Debug, PartialEq)]
struct Identity<T>(T);

impl<T> ApplicativeSuper for Identity<T> {
    type Item = T;

    fn new(x: Self::Item) -> Self {
        Identity(x)
    }
}

impl<X, Y, F> Applicative<Identity<Y>, F, Identity<F>> for Identity<X>
where
    F: Fn(&X) -> Y,
{
    fn apply(&self, a: Identity<F>) -> Identity<Y> {
        let Identity(x) = self;
        let f = a.0;
        let y = f(x);
        Identity::new(y)
    }
}

#[derive(Debug, PartialEq)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> ApplicativeSuper for Maybe<T> {
    type Item = T;

    fn new(x: Self::Item) -> Self {
        Maybe::Just(x)
    }
}

impl<X, Y, F> Applicative<Maybe<Y>, F, Maybe<F>> for Maybe<X>
where
    F: Fn(&X) -> Y,
{
    fn apply(&self, a: Maybe<F>) -> Maybe<Y> {
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
