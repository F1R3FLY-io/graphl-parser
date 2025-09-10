use std::{fmt::Display, rc::Rc};

#[derive(Debug)]
pub struct Channel {
    name: String,
    prev: Option<Rc<Channel>>,
    next: Option<Rc<Channel>>,
    arguments: Vec<String>,
}

impl Channel {
    pub fn new(name: &str) -> Self {
        Channel {
            name: name.to_string(),
            prev: None,
            next: None,
            arguments: Vec::new(),
        }
    }
    pub fn link(&mut self, next: Rc<Channel>) {
        self.next = Some(next);
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{channel_name}!(*{channel_name}_result) |
for(){{

}}
"#,
            channel_name = self.name
        )
    }
}

#[derive(Debug)]
pub struct ContractBuilder {
    pub contract_name: String,
    pub channels: Vec<Channel>,
}

impl Display for ContractBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
contract {contract_name} (contract_result) = {{
  new {channel_names} in {{
    {calls}
  }}
}}"#,
            contract_name = self.contract_name,
            channel_names = self
                .channels
                .iter()
                .take(self.channels.len() - 1)
                .flat_map(|c| vec![c.name.clone(), format!("{}_result", c.name.clone())])
                .chain(self.channels.last().iter().map(|c| c.name.clone()))
                .collect::<Vec<String>>()
                .join(", "),
            calls = self
                .channels
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("| ")
        )
    }
}

impl ContractBuilder {
    pub fn new(contract_name: impl Into<String>) -> Self {
        ContractBuilder {
            contract_name: contract_name.into(),
            channels: vec![],
        }
    }

    pub fn add_channel(&mut self, channel: Channel) {
        self.channels.push(channel);
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::rholang::{Channel, ContractBuilder};

    #[test]
    fn test_new() {
        let contract = ContractBuilder::new("test_contract");

        assert_eq!(contract.contract_name, "test_contract");
    }

    #[test]
    fn test_render() {
        let contract = ContractBuilder::new("test_contract");

        assert_eq!(contract.to_string(), "contract test_contract(return) = {}");
    }

    #[test]
    fn test_add_channel() {
        let mut contract = ContractBuilder::new("test_contract");

        contract.add_channel(Channel::new("a"));

        assert_eq!(
            contract.to_string(),
            "contract test_contract(return) = { new a in {} }"
        );
    }

    #[test]
    fn test_add_2_channels() {
        let mut contract = ContractBuilder::new("test_contract");

        contract.add_channel(Channel::new("a"));
        contract.add_channel(Channel::new("b"));

        assert_eq!(
            contract.to_string(),
            "contract test_contract(return) = { new a, a_return, b in {} }"
        );
    }

    #[test]
    fn test_channel_call() {
        let mut channel_a = Channel::new("a");
        let channel_b = Rc::new(Channel::new("b"));
        channel_a.link(Rc::clone(&channel_b));

        assert_eq!(
            channel_a.to_string(),
            r#"
a!(*a_result) |
for(a_result_value <- a_result){
  b!(*a_result_value)
}"#
            .trim()
            .to_owned()
        );
    }

    #[test]
    fn test_add_linked_channels() {
        let mut contract = ContractBuilder::new("test_contract");

        let mut channel_a = Channel::new("a");
        let channel_b = Rc::new(Channel::new("b"));
        channel_a.link(Rc::clone(&channel_b));

        contract.add_channel(channel_a);

        assert_eq!(
            contract.to_string(),
            "contract test_contract(contract_result) = {
              new a, a_result, b in {
                a!(*a_result) |
                for (a_value <- a_result){
                  b!(*contract_result)
                }
              }
            }"
        );
    }
}
