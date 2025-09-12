//! Rholang contract builder module for generating contract code.
//!
//! This module provides tools for constructing and rendering Rholang contracts
//! with chainable channel operations. The primary component is `ContractBuilder`,
//! which allows you to define a contract with a name, arguments, and a sequence
//! of channels that will be executed in order.

use super::channel::Channel;

/// A builder for generating Rholang contract code.
///
/// `ContractBuilder` allows you to construct a Rholang contract with a given name
/// and a sequence of channels that will be chained together in the contract execution.
/// The builder supports adding custom arguments to the contract and generates
/// complete Rholang code that can be deployed and executed.
///
/// # Examples
///
/// ```
/// use crate::rholang::{channel::Channel, contract_builder::ContractBuilder};
///
/// // Create a simple contract with no channels
/// let contract = ContractBuilder::new("simple_contract", vec![]);
/// let rholang_code = contract.render_rholang();
///
/// // Create a contract with multiple channels
/// let channels = vec![Channel::new("process_data"), Channel::new("validate")];
/// let mut contract = ContractBuilder::new("data_pipeline", channels);
/// contract.add_argument("input_data");
/// let rholang_code = contract.render_rholang();
/// ```
#[derive(Debug, Default)]
pub struct ContractBuilder {
    /// The name of the contract to be generated.
    pub contract_name: String,
    /// The channels that will be used in the contract execution chain.
    /// Each channel will receive the output of the previous channel as input.
    pub channels: Vec<Channel>,
    /// The arguments that will be passed to the contract.
    /// These arguments are available to the first channel in the execution chain.
    pub arguments: Vec<String>,
}

impl ContractBuilder {
    /// Creates a new `ContractBuilder` with the specified contract name and channels.
    ///
    /// The channels will be executed in the order they appear in the vector,
    /// with each channel's output being passed as input to the next channel.
    /// The contract starts with no arguments - use `add_argument` to add them.
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
    /// contract name, arguments, and channels. The generated contract follows these rules:
    ///
    /// - If no channels are provided, the contract simply returns `Nil`
    /// - If channels are provided, they are chained together where each channel's
    ///   result is passed to the next channel in the sequence
    /// - Contract arguments (if any) are passed to the first channel in the chain
    /// - The final channel's result is returned via the `contract_result` callback
    ///
    /// # Returns
    ///
    /// A `String` containing the complete Rholang contract code ready for deployment.
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

    /// Adds a new argument to the contract.
    ///
    /// Arguments are passed to the contract when it's invoked and are available
    /// to the first channel in the execution chain. Arguments are added in the
    /// order this method is called.
    ///
    /// # Arguments
    ///
    /// * `name` - A string-like type that will be converted into the argument name
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::rholang::{channel::Channel, contract_builder::ContractBuilder};
    ///
    /// let mut contract = ContractBuilder::new("my_contract", vec![]);
    /// contract.add_argument("input_data");
    /// contract.add_argument("config");
    ///
    /// let rholang_code = contract.render_rholang();
    /// // The contract will now accept input_data and config as parameters
    /// ```
    pub fn add_argument(&mut self, name: impl Into<String>) {
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
