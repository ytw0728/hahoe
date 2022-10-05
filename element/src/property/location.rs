pub trait location<T> {
    type Error;
    pub fn location_change(&self, Magnitude : T , dir : [T;3]) -> Result<_, Error>;
}