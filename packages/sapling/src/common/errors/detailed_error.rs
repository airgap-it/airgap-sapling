pub trait DetailedError {
    fn details(&self) -> String;
}