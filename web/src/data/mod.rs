use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub long_description: String,
    pub tech_stack: Vec<String>,
    pub role: String,
    pub thumbnail: String,
    pub screenshots: Vec<String>,
    pub live_demo: String,
    pub source_code: String,
    pub featured: bool,
}

pub fn load_projects() -> Vec<Project> {
    let data = include_str!("../data/projects.json");
    serde_json::from_str(data).expect("Failed to parse projects.json")
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experience {
    pub company: String,
    pub role: String,
    pub date_range: String,
    pub location: String,
    pub achievements: Vec<String>,
}

pub fn load_experience() -> Vec<Experience> {
    let data = include_str!("../data/experience.json");
    serde_json::from_str(data).expect("Failed to parse experience.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimum number of achievement bullets expected per experience entry.
    /// Derived from story spec: each entry should have 2-3 achievement bullets.
    const MIN_ACHIEVEMENTS_PER_ENTRY: usize = 2;

    #[test]
    fn test_load_experience_returns_entries() {
        let experiences = load_experience();
        assert!(
            !experiences.is_empty(),
            "Should have at least 1 experience entry"
        );
    }

    #[test]
    fn test_load_experience_most_recent_first() {
        let experiences = load_experience();
        assert!(
            experiences.len() >= 2,
            "Should have at least 2 entries for ordering test"
        );
        // First entry should be the most recent — check it contains "present"
        // rather than hardcoding a year, which would break on data updates
        assert!(
            experiences[0].date_range.to_lowercase().contains("present"),
            "First entry should be most recent (contain 'present'), got: {}",
            experiences[0].date_range
        );
    }

    #[test]
    fn test_experience_fields_populated() {
        let experiences = load_experience();
        for exp in &experiences {
            assert!(!exp.company.is_empty(), "Company should not be empty");
            assert!(!exp.role.is_empty(), "Role should not be empty");
            assert!(!exp.date_range.is_empty(), "Date range should not be empty");
            assert!(!exp.location.is_empty(), "Location should not be empty");
            assert!(
                !exp.achievements.is_empty(),
                "Achievements should not be empty"
            );
        }
    }

    #[test]
    fn test_experience_achievements_count() {
        let experiences = load_experience();
        for exp in &experiences {
            assert!(
                exp.achievements.len() >= MIN_ACHIEVEMENTS_PER_ENTRY,
                "Each entry should have at least {} achievements, got {} for {}",
                MIN_ACHIEVEMENTS_PER_ENTRY,
                exp.achievements.len(),
                exp.company
            );
        }
    }
}
