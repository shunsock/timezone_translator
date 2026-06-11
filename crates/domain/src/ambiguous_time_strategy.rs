use std::str::FromStr;

/// Strategy to resolve an ambiguous local time.
///
/// When DST ends, clocks are set back and the same local time
/// occurs twice. This strategy decides which occurrence to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguousTimeStrategy {
    Earliest,
    Latest,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error(
    "Validation Error: Invalid ambiguous time strategy found. {0} (expected: earliest, latest)"
)]
pub struct AmbiguousTimeStrategyParseError(pub String);

impl FromStr for AmbiguousTimeStrategy {
    type Err = AmbiguousTimeStrategyParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "earliest" => Ok(AmbiguousTimeStrategy::Earliest),
            "latest" => Ok(AmbiguousTimeStrategy::Latest),
            _ => Err(AmbiguousTimeStrategyParseError(text.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_earliest() {
        // Arrange
        let input = "earliest";

        // Act
        let strategy: AmbiguousTimeStrategy = input.parse().unwrap();

        // Assert
        assert_eq!(strategy, AmbiguousTimeStrategy::Earliest);
    }

    #[test]
    fn parses_latest() {
        // Arrange
        let input = "latest";

        // Act
        let strategy: AmbiguousTimeStrategy = input.parse().unwrap();

        // Assert
        assert_eq!(strategy, AmbiguousTimeStrategy::Latest);
    }

    #[test]
    fn rejects_unknown_strategy() {
        // Arrange
        let input = "invalid";

        // Act
        let result: Result<AmbiguousTimeStrategy, AmbiguousTimeStrategyParseError> = input.parse();

        // Assert
        assert_eq!(
            result,
            Err(AmbiguousTimeStrategyParseError(input.to_string()))
        );
    }
}
