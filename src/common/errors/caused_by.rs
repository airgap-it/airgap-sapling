pub trait CausedBy<T> {
    fn caused_by(cause: T) -> Self;
}