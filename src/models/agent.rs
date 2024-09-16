struct Agent { 
    pub first_name: String
}

#[derive(Default)]
struct AgentBuilder {
   pub first_name: Option<String>
}

impl Agent {
    pub fn builder() -> AgentBuilder { 
        AgentBuilder::default()
    }
}

impl AgentBuilder {
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(String::from(first_name));
        self
    } 

    pub fn build(self) -> Result<Agent, &'static str> {
        Ok(Agent {
            first_name: self.first_name.ok_or("_")?
        })
    }
}


#[cfg(test)]
mod tests {
    use super::Agent;

    #[test]
    fn test_agent_name() {
        let agent = Agent::builder().first_name("Jason").build();
        
        assert!(agent.is_ok())
    }

}