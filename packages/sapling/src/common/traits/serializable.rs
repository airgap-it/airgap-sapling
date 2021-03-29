pub trait Serializable<T, Err> {
    fn deserialize(serialized: T) -> Result<Self, Err> where Self: Sized;
    fn serialize(&self) -> Result<T, Err>;
}