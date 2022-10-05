pub trait location<T> {
    type Error;
    pub fn location_change(&self, dir : [T;3]) -> Result<_, Error>;
}