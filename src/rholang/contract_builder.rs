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
        ContractBuilder {
            contract_name: contract_name.into(),
            channels,
        }
    }

    pub fn render_rholang(&self) -> String {
        let call_stack = if self.channels.is_empty() {
            "contract_result!(Nil)".to_string()
        } else {
            let init_stack = self
                .channels
                .iter()
                .scan(PLACEHOLDER.to_string(), |state, ch| {
                    *state = state.replace(
                        PLACEHOLDER,
                        &format!(
                            r#"new {ch_name}, {ch_name}_result in {{ {PLACEHOLDER} }}"#,
                            ch_name = ch.name
                        ),
                    );

                    Some(state.to_owned())
                })
                .collect::<String>();

            let last_channel = self
                .channels
                .last()
                .expect("Something goes wrong, because here have to be last channel");

            self.channels
                .iter()
                .scan(init_stack, |state, chanel| {
                    *state = state.replace(
                        PLACEHOLDER,
                        &format!(
                            r#"{ch_name}!(*{ch_name}_result) | for ({ch_name}_result_value <- {ch_name}_result) {{ {PLACEHOLDER} }}"#,
                            ch_name = chanel.name
                        ),
                    );

                    Some(state.to_owned())
                })
                .collect::<String>()
                .replace(
                    PLACEHOLDER,
                    &format!(
                        "contract_result!(*{last_channel_name}_result_value)",
                        last_channel_name = last_channel.name
                    ),
                )
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
    fn test_render_with_channels() {
        let channels = vec![Channel::new("a")];
        let contract = ContractBuilder::new("test_contract", channels);
        let chain = contract.render_rholang();

        assert_eq!(
            chain,
            r#"contract test_contract (contract_result) = { new a, a_result in { a!(*a_result) | for (a_result_value <- a_result) { contract_result!(*a_result_value) } } }"#
        );
    }
}
