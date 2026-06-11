/// Strategy to resolve an ambiguous local time.
///
/// When DST ends, clocks are set back and the same local time
/// occurs twice. This strategy decides which occurrence to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguousTimeStrategy {
    Earliest,
    Latest,
}
