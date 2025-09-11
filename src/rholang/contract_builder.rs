//! Rholang contract builder module for generating contract code.

use super::channel::Channel;

/// Placeholder string used during template rendering.
const PLACEHOLDER: &str = "%REPLACE_ME%";

/// A builder for generating Rholang contract code.
///
/// `ContractBuilder` allows you to construct a Rholang contract with a given name
/// and a sequence of channels that will be chained together in the contract execution.
#[derive(Debug)]
pub struct ContractBuilder {
    /// The name of the contract to be generated.
    pub contract_name: String,
    /// The channels that will be used in the contract execution chain.
    pub channels: Vec<Channel>,
}

impl ContractBuilder {
    /// Creates a new `ContractBuilder` with the specified contract name and channels.
    ///
    /// # Arguments
    ///
    /// * `contract_name` - A string-like type that will be converted into the contract name
    /// * `channels` - A vector of `Channel` objects that define the execution chain
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::rholang::{channel::Channel, contract_builder::ContractBuilder};
    ///
    /// let channels = vec![Channel::new("a"), Channel::new("b")];
    /// let contract = ContractBuilder::new("my_contract", channels);
    /// ```
    pub fn new(contract_name: impl Into<String>, channels: Vec<Channel>) -> Self {
        ContractBuilder {
            contract_name: contract_name.into(),
            channels,
        }
    }

    /// Renders the contract as Rholang code.
    ///
    /// This method generates a complete Rholang contract string based on the configured
    /// contract name and channels. If no channels are provided, the contract will simply
    /// return `Nil`. Otherwise, it creates a chain of channel calls where each channel's
    /// result is passed to the next channel in the sequence.
    ///
    /// # Returns
    ///
    /// A `String` containing the complete Rholang contract code.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::rholang::{channel::Channel, contract_builder::ContractBuilder};
    ///
    /// // Empty contract
    /// let contract = ContractBuilder::new("test_contract", vec![]);
    /// assert_eq!(
    ///     contract.render_rholang(),
    ///     r#"contract test_contract (contract_result) = { contract_result!(Nil) }"#
    /// );
    ///
    /// // Contract with one channel
    /// let channels = vec![Channel::new("a")];
    /// let contract = ContractBuilder::new("test_contract", channels);
    /// let result = contract.render_rholang();
    /// // Result will be a contract that calls channel 'a' and returns its result
    /// ```
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
            .enumerate()
              .fold(PLACEHOLDER.to_string(), |state, (index,channel)| {
                let prev_channel = if index > 0 {
                  format!("{}_result_value", self.channels.get(index-1).unwrap_or(channel).name.clone())
                }else{
                    format!("{}_result", channel.name.clone())
                };

                state.replace(
                      PLACEHOLDER,
                      &format!(
                          r#"{ch_name}!(*{prev_channel}) | for ({ch_name}_result_value <- {ch_name}_result) {{ {PLACEHOLDER} }}"#,
                          ch_name = channel.name,
                          prev_channel = prev_channel
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
