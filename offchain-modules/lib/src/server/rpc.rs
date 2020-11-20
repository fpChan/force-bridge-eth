use super::types::{BurnArgs, GetSudtBalanceArgs, LockArgs};
use crate::util::ckb_util::{build_lockscript_from_address, Generator};
use crate::util::eth_util::{
    build_lock_eth_payload, build_lock_token_payload, convert_eth_address, make_transaction,
};
use crate::util::settings::Settings;
use ckb_jsonrpc_types::TransactionView;
use ckb_sdk::Address;
use ckb_types::packed::Script;
use ethabi::Token;
use ethereum_tx_sign::RawTransaction;
use jsonrpc_core::{IoHandler, Result};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;
use molecule::prelude::Entity;
use std::str::FromStr;
use web3::types::U256;

#[rpc]
pub trait Rpc {
    #[rpc(name = "burn")]
    fn burn(&self, args: BurnArgs) -> Result<TransactionView>;
    #[rpc(name = "lock")]
    fn lock(&self, args: LockArgs) -> Result<RawTransaction>;
    #[rpc(name = "get_sudt_balance")]
    fn get_sudt_balance(&self, args: GetSudtBalanceArgs) -> Result<u128>;
}

pub struct RpcImpl {
    indexer_url: String,
    ckb_rpc_url: String,
    settings: Settings,
}

impl RpcImpl {
    fn new(config_path: String, indexer_url: String, ckb_rpc_url: String) -> Result<Self> {
        let settings = Settings::new(config_path.as_str()).expect("invalid settings");
        Ok(Self {
            indexer_url,
            ckb_rpc_url,
            settings,
        })
    }

    fn get_generator(&self) -> Result<Generator> {
        Generator::new(
            self.ckb_rpc_url.clone(),
            self.indexer_url.clone(),
            self.settings.clone(),
        )
        .map_err(|e| jsonrpc_core::Error::invalid_params(format!("new geneartor fail, err: {}", e)))
    }
}

impl Rpc for RpcImpl {
    fn burn(&self, args: BurnArgs) -> Result<TransactionView> {
        let from_lockscript = Script::from(
            Address::from_str(args.from_lockscript_addr.as_str())
                .map_err(|err| {
                    jsonrpc_core::Error::invalid_params_with_details(
                        err,
                        "ckb_address to script fail",
                    )
                })?
                .payload(),
        );
        let token_address = convert_eth_address(args.token_address.as_str())
            .map_err(|_| jsonrpc_core::Error::invalid_params("token address parse fail"))?;
        let lock_contract_address = convert_eth_address(args.lock_contract_address.as_str())
            .map_err(|_| jsonrpc_core::Error::invalid_params("lock contract address parse fail"))?;
        let recipient_address = convert_eth_address(args.recipient_address.as_str())
            .map_err(|_| jsonrpc_core::Error::invalid_params("recipient address parse fail"))?;

        let mut generator = self.get_generator()?;

        let tx = generator
            .burn(
                args.tx_fee,
                from_lockscript,
                args.unlock_fee,
                args.amount,
                token_address,
                lock_contract_address,
                recipient_address,
            )
            .map_err(|e| jsonrpc_core::Error::invalid_params(format!("burn fail, err: {}", e)))?;
        let rpc_tx = ckb_jsonrpc_types::TransactionView::from(tx);
        Ok(rpc_tx)
    }

    fn get_sudt_balance(&self, args: GetSudtBalanceArgs) -> Result<u128> {
        let token_address = convert_eth_address(args.token_address.as_str())
            .map_err(|_| jsonrpc_core::Error::invalid_params("token address parse fail"))?;
        let lock_contract_address = convert_eth_address(args.lock_contract_address.as_str())
            .map_err(|_| jsonrpc_core::Error::invalid_params("lock contract address parse fail"))?;

        let mut generator = self.get_generator()?;

        let balance = generator
            .get_sudt_balance(args.address, token_address, lock_contract_address)
            .map_err(|e| {
                jsonrpc_core::Error::invalid_params(format!("get_sudt_balance fail, err: {}", e))
            })?;
        Ok(balance)
    }

    fn lock(&self, args: LockArgs) -> Result<RawTransaction> {
        let to = convert_eth_address(args.lock_contract_address.as_str())
            .map_err(|_| jsonrpc_core::Error::invalid_params("lock contract address parse fail"))?;
        let nonce = U256::from(args.nonce);
        let gas_price = U256::from(args.gas_price);
        let amount = U256::from(args.amount);
        let bridge_fee = U256::from(args.bridge_fee);

        let token_addr = convert_eth_address(&args.token_address)
            .map_err(|_| jsonrpc_core::Error::invalid_params("token address parse fail"))?;
        let recipient_lockscript = build_lockscript_from_address(&args.ckb_recipient_address)
            .map_err(|_| jsonrpc_core::Error::invalid_params("ckb recipient address parse fail"))?;

        let data = [
            Token::Address(token_addr),
            Token::Uint(amount),
            Token::Uint(bridge_fee),
            Token::Bytes(recipient_lockscript.as_slice().to_vec()),
            Token::Bytes(hex::decode(args.replay_resist_outpoint).map_err(|e| {
                jsonrpc_core::Error::invalid_params(format!(
                    "decode replay_resist_outpoint fail, err: {}",
                    e
                ))
            })?),
            Token::Bytes(args.sudt_extra_data.as_bytes().to_vec()),
        ];

        let mut eth_value = amount;

        let input_data = {
            if token_addr.0 == [0u8; 20] {
                let lock_eth_data = &data[2..];
                build_lock_eth_payload(lock_eth_data).map_err(|e| {
                    jsonrpc_core::Error::invalid_params(format!(
                        "abi encode lock eth data fail, err: {}",
                        e
                    ))
                })?
            } else {
                eth_value = U256::from(0);
                build_lock_token_payload(&data).map_err(|e| {
                    jsonrpc_core::Error::invalid_params(format!(
                        "abi encode lock token data fail, err: {}",
                        e
                    ))
                })?
            }
        };

        Ok(make_transaction(
            to, nonce, input_data, gas_price, eth_value,
        ))
    }
}

pub fn start(
    config_path: String,
    ckb_rpc_url: String,
    indexer_url: String,
    listen_url: String,
    threads_num: usize,
) {
    let mut io = IoHandler::new();
    let rpc = RpcImpl::new(config_path, indexer_url, ckb_rpc_url).expect("init handler error");
    io.extend_with(rpc.to_delegate());

    let server = ServerBuilder::new(io)
        .threads(threads_num)
        .start_http(&listen_url.parse().unwrap())
        .unwrap();
    server.wait();
}