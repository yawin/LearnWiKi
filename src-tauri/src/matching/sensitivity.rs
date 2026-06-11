//! Sensitivity tiers controlling both AI and keyword thresholds via one setting.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensitivityLevel {
    Loose,
    Balanced,
    Strict,
}

impl SensitivityLevel {
    pub fn ai_threshold(self) -> f64 {
        match self {
            SensitivityLevel::Loose => 0.5,
            SensitivityLevel::Balanced => 0.6,
            SensitivityLevel::Strict => 0.7,
        }
    }

    pub fn keyword_threshold(self) -> f64 {
        match self {
            SensitivityLevel::Loose => 0.2,
            SensitivityLevel::Balanced => 0.3,
            SensitivityLevel::Strict => 0.4,
        }
    }

    pub fn from_setting(value: Option<&str>) -> Self {
        match value {
            Some("loose") => SensitivityLevel::Loose,
            Some("strict") => SensitivityLevel::Strict,
            _ => SensitivityLevel::Balanced,
        }
    }
}

pub const SETTING_KEY: &str = "auto_link_sensitivity";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_thresholds_match_design() {
        assert_eq!(SensitivityLevel::Loose.ai_threshold(), 0.5);
        assert_eq!(SensitivityLevel::Balanced.ai_threshold(), 0.6);
        assert_eq!(SensitivityLevel::Strict.ai_threshold(), 0.7);
    }

    #[test]
    fn keyword_thresholds_match_design() {
        assert_eq!(SensitivityLevel::Loose.keyword_threshold(), 0.2);
        assert_eq!(SensitivityLevel::Balanced.keyword_threshold(), 0.3);
        assert_eq!(SensitivityLevel::Strict.keyword_threshold(), 0.4);
    }

    #[test]
    fn from_setting_defaults_to_balanced() {
        assert_eq!(SensitivityLevel::from_setting(None), SensitivityLevel::Balanced);
        assert_eq!(SensitivityLevel::from_setting(Some("")), SensitivityLevel::Balanced);
        assert_eq!(SensitivityLevel::from_setting(Some("nonsense")), SensitivityLevel::Balanced);
    }

    #[test]
    fn from_setting_parses_known_values() {
        assert_eq!(SensitivityLevel::from_setting(Some("loose")), SensitivityLevel::Loose);
        assert_eq!(SensitivityLevel::from_setting(Some("balanced")), SensitivityLevel::Balanced);
        assert_eq!(SensitivityLevel::from_setting(Some("strict")), SensitivityLevel::Strict);
    }
}
