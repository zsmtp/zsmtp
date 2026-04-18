use crate::config::ServerConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Server {
    config: ServerConfig,
}

impl Server {
    #[must_use]
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    #[must_use]
    pub fn bind_address(&self) -> &str {
        &self.config.bind_address
    }

    #[must_use]
    pub fn hostname(&self) -> &str {
        &self.config.hostname
    }

    #[must_use]
    pub fn status_message(&self) -> &'static str {
        "listening disabled; server protocol skeleton only"
    }
}
