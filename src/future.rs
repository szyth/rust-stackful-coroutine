pub trait Future {
    type Output;
    fn poll(&mut self) -> PollState<Self::Output>;
}
pub enum PollState<Data> {
    Ready(Data),
    NotReady,
}
