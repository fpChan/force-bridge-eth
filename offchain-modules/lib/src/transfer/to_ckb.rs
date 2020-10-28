use anyhow::Result;

use crate::util::eth_util::Web3Client;
use ethabi::{Function, Param, ParamType, Token};
use futures::executor;
use web3::types::{H160, H256, U256};

pub fn approve(from: H160, to: H160, url: String, key_path: String) -> H256 {
    let mut rpc_client = Web3Client::new(url);
    let function = Function {
        name: "approve".to_owned(),
        inputs: vec![
            Param {
                name: "_spender".to_owned(),
                kind: ParamType::Address,
            },
            Param {
                name: "_value".to_owned(),
                kind: ParamType::Uint(256),
            },
        ],
        outputs: vec![Param {
            name: "success".to_owned(),
            kind: ParamType::Bool,
        }],
        constant: false,
    };
    let tokens = [Token::Address(from), Token::Uint(U256::max_value())];
    let input_data = function.encode_input(&tokens).expect("invalid input data");
    let f = rpc_client.send_transaction(from, to, key_path, input_data);
    executor::block_on(f).expect("invalid tx hash")
    // let res = rpc_client.send_transaction(from, to, key_path, data)
}

pub fn lock(from: H160, to: H160, url: String, key_path: String, data: &[Token]) -> H256 {
    let mut rpc_client = Web3Client::new(url);
    let function = Function {
        name: "lock".to_owned(),
        inputs: vec![
            Param {
                name: "token".to_owned(),
                kind: ParamType::Address,
            },
            Param {
                name: "amount".to_owned(),
                kind: ParamType::Uint(256),
            },
            Param {
                name: "ckbAddress".to_owned(),
                kind: ParamType::String,
            },
        ],
        outputs: vec![],
        constant: false,
    };
    let input_data = function.encode_input(data).expect("invalid input data");
    // rpc_client.send_transaction(from, to, key_path, data)
    let f = rpc_client.send_transaction(from, to, key_path, input_data);
    executor::block_on(f).expect("invalid tx hash")
}

pub fn parse_eth_proof() -> Result<()> {
    todo!()
}

pub fn verify_eth_spv_proof() -> bool {
    todo!()
}
