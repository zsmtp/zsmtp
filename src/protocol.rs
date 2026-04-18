use crate::config::ProtocolConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolProfile {
    name: String,
    require_starttls: bool,
    zero_knowledge: bool,
}

impl ProtocolProfile {
    #[must_use]
    pub fn from_settings(config: &ProtocolConfig) -> Self {
        Self {
            name: config.profile.clone(),
            require_starttls: config.require_starttls,
            zero_knowledge: config.zero_knowledge,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn require_starttls(&self) -> bool {
        self.require_starttls
    }

    #[must_use]
    pub fn zero_knowledge(&self) -> bool {
        self.zero_knowledge
    }
}
