// alias for Result<T,E>
// to avoid convert from orphan rule
pub struct CR<T, E>(pub Result<T, E>);
