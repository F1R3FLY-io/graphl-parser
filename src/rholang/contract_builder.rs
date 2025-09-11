use std::str::FromStr;

use super::channel::Channel;

const PLACEHOLDER: &str = "%REPLACE_ME%";

#[derive(Debug)]
pub struct ContractBuilder {
    pub contract_name: String,
    pub channels: Vec<Channel>,
}

impl ContractBuilder {
    pub fn new(contract_name: impl Into<String>, channels: Vec<Channel>) -> Self {
        // let mut channels = channels;
        // channels.push(Channel::new("contract_result"));

        ContractBuilder {
            contract_name: contract_name.into(),
            channels,
        }
    }

    pub fn render_rholang(&self) -> String {
        let call_stack = if self.channels.is_empty() {
            "contract_result!(Nil)".to_string()
        } else {
            let init_variables = self
                .channels
                .iter()
                .fold(PLACEHOLDER.to_string(), |state, ch| {
                    state.replace(
                        PLACEHOLDER,
                        &format!(
                            r#"new {ch_name}, {ch_name}_result in {{ {PLACEHOLDER} }}"#,
                            ch_name = ch.name
                        ),
                    )
                });

            let call_stack = self.channels
              .iter()
              .fold(PLACEHOLDER.to_string(), |state, (channel)| {
                  state.replace(
                      PLACEHOLDER,
                      &format!(
                          r#"{ch_name}!(*{ch_name}_result) | for ({ch_name}_result_value <- {ch_name}_result) {{ {PLACEHOLDER} }}"#,
                          ch_name = channel.name
                      ),
                  )
              }).replace(
                  PLACEHOLDER,
                  &format!(
                      "contract_result!(*{ch_name}_result_value)",
                      ch_name = self
                          .channels
                          .last().unwrap().name
                  ),
              );

            init_variables.replace(PLACEHOLDER, &call_stack)
        };

        format!(
            r#"contract {contract_name} (contract_result) = {{ {call_stack} }}"#,
            contract_name = self.contract_name,
            call_stack = call_stack
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::rholang::{channel::Channel, contract_builder::ContractBuilder};

    #[test]
    fn test_render() {
        let contract = ContractBuilder::new("test_contract", vec![]);

        assert_eq!(
            contract.render_rholang(),
            r#"contract test_contract (contract_result) = { contract_result!(Nil) }"#
        );
    }

    #[test]
    fn test_render_with_one_channel() {
        let channels = vec![Channel::new("a")];
        let contract = ContractBuilder::new("test_contract", channels);
        let chain = contract.render_rholang();

        assert_eq!(
            chain,
            r#"contract test_contract (contract_result) = { new a, a_result in { a!(*a_result) | for (a_result_value <- a_result) { contract_result!(*a_result_value) } } }"#
        );
    }

    #[test]
    fn test_render_with_two_channels() {
        let channels = vec![Channel::new("a"), Channel::new("b")];
        let contract = ContractBuilder::new("test_contract", channels);
        let chain = contract.render_rholang();

        assert_eq!(
            chain,
            r#"contract test_contract (contract_result) = { new a, a_result in { new b, b_result in { a!(*a_result) | for (a_result_value <- a_result) { b!(*a_result_value) | for (b_result_value <- b_result) { contract_result!(*b_result_value) } } } } }"#
        );
    }
}
