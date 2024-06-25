use crate::u256_numeric_string;
use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{Address, Bytes, B256, U256};
use serde::{Deserialize, Serialize};

/// Bundle of transactions for `eth_callBundle`
///
/// <https://docs.flashbots.net/flashbots-auction/searchers/advanced/rpc-endpoint#eth_callBundle>
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthCallBundle {
    /// A list of hex-encoded signed transactions
    pub txs: Vec<Bytes>,
    /// hex encoded block number for which this bundle is valid on
    #[serde(with = "alloy_serde::quantity")]
    pub block_number: u64,
    /// Either a hex encoded number or a block tag for which state to base this simulation on
    pub state_block_number: BlockNumberOrTag,
    /// the timestamp to use for this bundle simulation, in seconds since the unix epoch
    #[serde(default, with = "alloy_serde::quantity::opt", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}

/// Response for `eth_callBundle`
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EthCallBundleResponse {
    /// The hash of the bundle bodies.
    pub bundle_hash: B256,
    /// The gas price of the entire bundle
    #[serde(with = "u256_numeric_string")]
    pub bundle_gas_price: U256,
    /// The difference in Ether sent to the coinbase after all transactions in the bundle
    #[serde(with = "u256_numeric_string")]
    pub coinbase_diff: U256,
    /// The total amount of Ether sent to the coinbase after all transactions in the bundle
    #[serde(with = "u256_numeric_string")]
    pub eth_sent_to_coinbase: U256,
    /// The total gas fees paid for all transactions in the bundle
    #[serde(with = "u256_numeric_string")]
    pub gas_fees: U256,
    /// Results of individual transactions within the bundle
    pub results: Vec<EthCallBundleTransactionResult>,
    /// The block number used as a base for this simulation
    #[serde(with = "alloy_serde::quantity")]
    pub state_block_number: u64,
    /// The total gas used by all transactions in the bundle
    #[serde(with = "alloy_serde::quantity")]
    pub total_gas_used: u64,
}

/// Result of a single transaction in a bundle for `eth_callBundle`
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthCallBundleTransactionResult {
    /// The difference in Ether sent to the coinbase after the transaction
    #[serde(with = "u256_numeric_string")]
    pub coinbase_diff: U256,
    /// The amount of Ether sent to the coinbase after the transaction
    #[serde(with = "u256_numeric_string")]
    pub eth_sent_to_coinbase: U256,
    /// The address from which the transaction originated
    pub from_address: Address,
    /// The gas fees paid for the transaction
    #[serde(with = "u256_numeric_string")]
    pub gas_fees: U256,
    /// The gas price used for the transaction
    #[serde(with = "u256_numeric_string")]
    pub gas_price: U256,
    /// The amount of gas used by the transaction
    #[serde(with = "alloy_serde::quantity")]
    pub gas_used: u64,
    /// The address to which the transaction is sent (optional)
    pub to_address: Option<Address>,
    /// The transaction hash
    pub tx_hash: B256,
    /// Contains the return data if the transaction succeeded
    ///
    /// Note: this is mutually exclusive with `revert`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Bytes>,
    /// Contains the return data if the transaction reverted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert: Option<Bytes>,
}

#[cfg(test)]
mod tests {
    use super::EthCallBundleResponse;

    #[test]
    fn can_deserialize_eth_call_resp() {
        let s = r#"{ "bundleGasPrice": "476190476193",
"bundleHash": "0x73b1e258c7a42fd0230b2fd05529c5d4b6fcb66c227783f8bece8aeacdd1db2e",
"coinbaseDiff": "20000000000126000",
"ethSentToCoinbase": "20000000000000000",
"gasFees": "126000",
"results": [
  {
    "coinbaseDiff": "10000000000063000",
    "ethSentToCoinbase": "10000000000000000",
    "fromAddress": "0x02A727155aeF8609c9f7F2179b2a1f560B39F5A0",
    "gasFees": "63000",
    "gasPrice": "476190476193",
    "gasUsed": 21000,
    "toAddress": "0x73625f59CAdc5009Cb458B751b3E7b6b48C06f2C",
    "txHash": "0x669b4704a7d993a946cdd6e2f95233f308ce0c4649d2e04944e8299efcaa098a",
    "value": "0x"
  },
  {
    "coinbaseDiff": "10000000000063000",
    "ethSentToCoinbase": "10000000000000000",
    "fromAddress": "0x02A727155aeF8609c9f7F2179b2a1f560B39F5A0",
    "gasFees": "63000",
    "gasPrice": "476190476193",
    "gasUsed": 21000,
    "toAddress": "0x73625f59CAdc5009Cb458B751b3E7b6b48C06f2C",
    "txHash": "0xa839ee83465657cac01adc1d50d96c1b586ed498120a84a64749c0034b4f19fa",
    "value": "0x"
  }
],
"stateBlockNumber": 5221585,
"totalGasUsed": 42000
}"#;

        let _call = serde_json::from_str::<EthCallBundleResponse>(s).unwrap();
    }
}
