use std::fmt;
struct Profile {
    pub first_name: String,
}

#[derive(Default)]
struct ProfileBuilder {
    #[allow(dead_code)]
    pub first_name: Option<String>,
}

impl Profile {
    #[allow(dead_code)]
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::default()
    }
}

impl ProfileBuilder {
    #[allow(dead_code)]
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(String::from(first_name));
        self
    }

    #[allow(dead_code)]
    pub fn build(self) -> Result<Profile, &'static str> {
        Ok(Profile {
            first_name: self.first_name.ok_or("_")?,
        })
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.first_name)
    }
}

#[cfg(test)]
mod tests {
    use super::Profile;

    #[test]
    fn test_agent_name() {
        let agent = Profile::builder().first_name("Jason").build();

        assert!(agent.is_ok())
    }

    #[test]
    fn test_agent_name_to_string() {
        let agent = Profile::builder().first_name("Jason").build().unwrap();

        assert_eq!(agent.to_string(), "Jason")
    }
}
