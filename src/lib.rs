pub trait KtStd {
    fn then<R>(self, f: impl FnOnce(Self) -> R) -> R where Self: Sized {
        f(self)
    }
}

impl<T> KtStd for T {}

