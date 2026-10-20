pub trait EntityProperty {
    type Value: Default;

    const OFFSET: usize;
}
