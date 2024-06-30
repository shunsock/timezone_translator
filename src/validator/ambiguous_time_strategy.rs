#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum AmbiguousTimeStrategy {
    Earliest,
    Latest,
}