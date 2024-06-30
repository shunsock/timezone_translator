#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AmbiguousTimeStrategy {
    Earliest,
    Latest,
}