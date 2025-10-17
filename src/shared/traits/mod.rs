pub trait AggregateRoot {
    type Event: Clone + Send + Sync + 'static;
    fn apply_event(&mut self, event: &Self::Event);
    fn uncommitted_events(&self) -> &[Self::Event];
    fn take_uncommitted(&mut self) -> Vec<Self::Event>;
}

