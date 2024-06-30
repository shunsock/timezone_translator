#[derive(Debug, Clone)]
pub(crate) enum AmbiguousTimeStrategy {
    Earliest,
    Latest,
}