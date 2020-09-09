pub trait Serializable<Err> {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Err> where Self: Sized;
    fn to_bytes(&self) -> Result<Vec<u8>, Err>;
}

pub trait Serializer<T, R, E> {
    fn serialize(value: T) -> Result<R, E>;
    fn deserialize(raw: R) -> Result<T, E>
}