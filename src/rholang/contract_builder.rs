//! Rholang contract builder module for generating contract code.

use super::channel::Channel;

/// A builder for generating Rholang contract code.
///
/// `ContractBuilder` allows you to construct a Rholang contract with a given name
/// and a sequence of channels that will be chained together in the contract execution.
#[derive(Debug, Default)]
pub struct ContractBuilder {
    /// The name of the contract to be generated.
    pub contract_name: String,
    /// The channels that will be used in the contract execution chain.
    pub channels: Vec<Channel>,
    /// The arguments that will be passed to the contract.
    pub arguments: Vec<String>,
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
            ..Default::default()
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
        if self.channels.is_empty() {
            return format!(
                r#"contract {contract_name} ({contract_arguments}) = {{ contract_result!(Nil) }}"#,
                contract_name = self.contract_name,
                contract_arguments = self
                    .arguments
                    .iter()
                    .map(|arg| arg.as_str())
                    .chain(vec!["contract_result"])
                    .collect::<Vec<&str>>()
                    .join(", ")
            );
        }

        let mut result = String::new();

        // Build the initialization part
        for channel in &self.channels {
            result.push_str(&format!(
                "new {}, {}_result in {{ ",
                channel.name, channel.name
            ));
        }

        // Build the call chain
        let is_last = |index: usize| index == self.channels.len() - 1;
        for (index, channel) in self.channels.iter().enumerate() {
            match index {
                0 => {
                    let arguments = if self.arguments.len() > 0 {
                        format!("*{}, ", self.arguments.join(","))
                    } else {
                        "".to_string()
                    };

                    result.push_str(&format!(
                  "{channel_name}!({arguments}*{channel_name}_result) | for ({channel_name}_result_value <- {channel_name}_result) {{ ",
                  channel_name = channel.name,
                  arguments = arguments
              ));
                }
                _ => {
                    let prev_channel = &self.channels[index - 1];
                    result.push_str(&format!(
                      "{channel_name}!(*{prev_channel_name}_result_value, *{channel_name}_result) | for ({channel_name}_result_value <- {channel_name}_result) {{ ",
                      channel_name = channel.name,
                      prev_channel_name = prev_channel.name,
                  ));
                }
            };
        }

        // Add the final result
        let last_channel = &self.channels.last().unwrap().name;
        result.push_str(&format!("contract_result!(*{}_result_value)", last_channel));

        // Close all braces
        for _ in 0..(self.channels.len() * 2) {
            result.push_str(" }");
        }

        format!(
            r#"contract {contract_name} ({contract_arguments}) = {{ {result} }}"#,
            contract_name = self.contract_name,
            contract_arguments = self
                .arguments
                .iter()
                .map(|arg| arg.as_str())
                .chain(vec!["contract_result"])
                .collect::<Vec<&str>>()
                .join(", "),
            result = result
        )
    }

    fn add_argument(&mut self, name: impl Into<String>) {
        self.arguments.push(name.into());
    }
}

#[cfg(test)]
mod tests {

    use crate::rholang::{channel::Channel, contract_builder::ContractBuilder};

    #[test]
    fn test_render() {
        let builder = ContractBuilder::new("test_contract", vec![]);

        assert_eq!(
            builder.render_rholang(),
            r#"contract test_contract (contract_result) = { contract_result!(Nil) }"#
        );
    }

    #[test]
    fn test_render_with_one_channel() {
        let channels = vec![Channel::new("a")];
        let builder = ContractBuilder::new("test_contract", channels);
        let chain = builder.render_rholang();

        assert_eq!(
            chain,
            r#"contract test_contract (contract_result) = { new a, a_result in { a!(*a_result) | for (a_result_value <- a_result) { contract_result!(*a_result_value) } } }"#
        );
    }

    #[test]
    fn test_render_with_two_channels() {
        let channels = vec![Channel::new("a"), Channel::new("b")];
        let builder = ContractBuilder::new("test_contract", channels);

        let chain = builder.render_rholang();

        assert_eq!(
            chain,
            r#"contract test_contract (contract_result) = { new a, a_result in { new b, b_result in { a!(*a_result) | for (a_result_value <- a_result) { b!(*a_result_value, *b_result) | for (b_result_value <- b_result) { contract_result!(*b_result_value) } } } } }"#
        );
    }

    #[test]
    fn test_contract_arguments() {
        let mut builder = ContractBuilder::new("test_contract", vec![]);
        builder.add_argument("input_1");
        builder.add_argument("input_2");

        let chain = builder.render_rholang();

        assert_eq!(
            chain,
            r#"contract test_contract (input_1, input_2, contract_result) = { contract_result!(Nil) }"#
        );
    }

    #[test]
    fn test_contract_with_arguments() {
        let channels = vec![Channel::new("a"), Channel::new("b")];
        let mut builder = ContractBuilder::new("test_contract", channels);
        builder.add_argument("input_1");

        let chain = builder.render_rholang();

        assert_eq!(
            chain,
            r#"contract test_contract (input_1, contract_result) = { new a, a_result in { new b, b_result in { a!(*input_1, *a_result) | for (a_result_value <- a_result) { b!(*a_result_value, *b_result) | for (b_result_value <- b_result) { contract_result!(*b_result_value) } } } } }"#
        );
    }
}
