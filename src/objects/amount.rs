#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Amount<T>
{
    pub(crate) rate_per_period: usize,
    pub(crate) kind: T,
}

impl<T> Amount<T>
{
    pub(crate) const fn new(count: usize, kind: T) -> Self {
        Self {
            rate_per_period: count,
            kind,
        }
    }
}
