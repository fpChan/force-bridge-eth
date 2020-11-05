use crate::util::settings::{OutpointConf, Settings};
use anyhow::Result;
use ckb_sdk::{Address, GenesisInfo, HttpRpcClient};
use ckb_types::core::{BlockView, Capacity, DepType, TransactionView};
use ckb_types::packed::HeaderVec;
use ckb_types::prelude::{Builder, Entity, Pack};
use ckb_types::{
    bytes::Bytes,
    packed::{self, Byte32, CellDep, CellOutput, OutPoint, Script},
    H256,
};
use faster_hex::hex_decode;
use force_sdk::cell_collector::{collect_sudt_amount, get_live_cell_by_typescript};
use force_sdk::indexer::IndexerRpcClient;
use force_sdk::tx_helper::TxHelper;
use force_sdk::util::{check_capacity, get_live_cell};
use std::str::FromStr;
use web3::types::H160;

pub fn make_ckb_transaction(_from_lockscript: Script) -> Result<TransactionView> {
    todo!()
}

pub struct Generator {
    pub rpc_client: HttpRpcClient,
    pub indexer_client: IndexerRpcClient,
    _genesis_info: GenesisInfo,
    _settings: Settings,
}

impl Generator {
    pub fn new(rpc_url: String, indexer_url: String, settings: Settings) -> Result<Self, String> {
        let mut rpc_client = HttpRpcClient::new(rpc_url);
        let indexer_client = IndexerRpcClient::new(indexer_url);
        let genesis_block: BlockView = rpc_client
            .get_block_by_number(0)?
            .expect("Can not get genesis block?")
            .into();
        let genesis_info = GenesisInfo::from_block(&genesis_block)?;
        Ok(Self {
            rpc_client,
            indexer_client,
            _genesis_info: genesis_info,
            _settings: settings,
        })
    }

    fn _add_cell_deps(
        &mut self,
        helper: &mut TxHelper,
        outpoints: Vec<OutpointConf>,
    ) -> Result<(), String> {
        let mut builder = helper.transaction.as_advanced_builder();
        for conf in outpoints {
            let outpoint = OutPoint::new_builder()
                .tx_hash(
                    Byte32::from_slice(
                        &hex::decode(conf.tx_hash)
                            .map_err(|e| format!("invalid OutpointConf config. err: {}", e))?,
                    )
                    .map_err(|e| format!("invalid OutpointConf config. err: {}", e))?,
                )
                .index(conf.index.pack())
                .build();
            builder = builder.cell_dep(
                CellDep::new_builder()
                    .out_point(outpoint)
                    .dep_type(DepType::Code.into())
                    .build(),
            );
        }
        helper.transaction = builder.build();
        Ok(())
    }

    fn _get_ckb_cell(
        &mut self,
        helper: &mut TxHelper,
        cell_typescript: Script,
        add_to_input: bool,
    ) -> Result<(CellOutput, Bytes), String> {
        let genesis_info = self._genesis_info.clone();
        let cell = get_live_cell_by_typescript(&mut self.indexer_client, cell_typescript)?
            .ok_or("cell not found")?;
        let ckb_cell = CellOutput::from(cell.output);
        let ckb_cell_data = packed::Bytes::from(cell.output_data).raw_data();
        if add_to_input {
            let mut get_live_cell_fn = |out_point: OutPoint, with_data: bool| {
                get_live_cell(&mut self.rpc_client, out_point, with_data).map(|(output, _)| output)
            };

            helper.add_input(
                cell.out_point.into(),
                None,
                &mut get_live_cell_fn,
                &genesis_info,
                true,
            )?;
        }
        Ok((ckb_cell, ckb_cell_data))
    }
    pub fn get_ckb_headers(&mut self, block_numbers: Vec<u64>) -> Result<Vec<u8>> {
        let mut mol_header_vec: Vec<packed::Header> = Default::default();
        for number in block_numbers {
            let block_opt = self
                .rpc_client
                .get_block_by_number(number)
                .map_err(|e| anyhow::anyhow!("failed to get block: {}", e))?;

            if let Some(block) = block_opt {
                mol_header_vec.push(block.header.inner.into());
            }
        }
        let mol_headers = HeaderVec::new_builder().set(mol_header_vec).build();
        Ok(Vec::from(mol_headers.as_slice()))
    }
    pub fn burn(
        &mut self,
        tx_fee: u64,
        from_lockscript: Script,
        burn_sudt_amount: u128,
        token_addr: H160,
        eth_receiver_addr: H160,
    ) -> Result<TransactionView, String> {
        let mut helper = TxHelper::default();

        // add cellDeps
        {
            let outpoints = vec![
                self._settings.bridge_lock_sctipt.outpoint.clone(),
                // self._settings.typescript.outpoint,
                self._settings.sudt.outpoint.clone(),
            ];
            self._add_cell_deps(&mut helper, outpoints)?;
        }

        // let bridge_lockscript_code_hash = hex::decode(&self._settings.bridge_lock_sctipt.code_hash)
        //     .expect("wrong sudt_script code hash config");
        //
        // let bridge_lockscript: Script = Script::new_builder()
        //     .code_hash(Byte32::from_slice(&bridge_lockscript_code_hash).unwrap())
        //     .hash_type(DepType::Code.into())
        //     .args(token_addr.as_bytes().pack())
        //     .build();
        //
        // // 通过 type_script 校验 ctoken
        // let sudt_typescript_code_hash = hex::decode(&self._settings.sudt.code_hash)
        //     .expect("wrong sudt_script code hash config");
        // let sudt_typescript = Script::new_builder()
        //     .code_hash(Byte32::from_slice(&sudt_typescript_code_hash).unwrap())
        //     .hash_type(DepType::Code.into())
        //     .args(bridge_lockscript.calc_script_hash().as_bytes().pack())
        //     .build();

        let sudt_typescript = get_sudt_lock_script(
            &self._settings.bridge_lock_sctipt.code_hash,
            &self._settings.sudt.code_hash,
            token_addr,
        );

        let ckb_amount = 200;
        // gen output of eth_recipient cell
        {
            let eth_recipient_data: Bytes = eth_receiver_addr.as_bytes().to_vec().into();
            // check_capacity(ckb_amount, eth_recipient_data.len())?;
            // burn cToken 所产生的 eth_recipient cell lock依旧是销毁者
            let eth_recipient_output = CellOutput::new_builder()
                .capacity(Capacity::shannons(ckb_amount).pack()) // check cap
                .lock(from_lockscript.clone())
                .build();
            // eth_recipient cell data = eth 接受地址
            helper.add_output(eth_recipient_output, eth_recipient_data);
        }

        helper.supply_sudt(
            &mut self.rpc_client,
            &mut self.indexer_client,
            from_lockscript.clone(),
            &self._genesis_info,
            burn_sudt_amount,
            sudt_typescript.clone(),
        )?;

        // build tx
        let tx = helper.supply_capacity(
            &mut self.rpc_client,
            &mut self.indexer_client,
            from_lockscript,
            &self._genesis_info,
            tx_fee,
        )?;
        Ok(tx)
    }

    pub fn transfer_sudt(
        &mut self,
        from_lockscript: Script,
        token_addr: H160,
        to_lockscript: Script,
        sudt_amount: u128,
        ckb_amount: u64,
        tx_fee: u64,
    ) -> Result<TransactionView, String> {
        //let ckb_amount: u64 = CapacityParser.parse(&ckb_amount)?.into();
        let mut helper = TxHelper::default();

        // add cellDeps
        let outpoints = vec![self._settings.sudt.outpoint.clone()];
        self._add_cell_deps(&mut helper, outpoints)?;

        {
            let sudt_typescript = get_sudt_lock_script(
                &self._settings.bridge_lock_sctipt.code_hash,
                &self._settings.sudt.code_hash,
                token_addr,
            );

            let sudt_output = CellOutput::new_builder()
                .capacity(Capacity::shannons(ckb_amount).pack())
                .type_(Some(sudt_typescript.clone()).pack())
                .lock(to_lockscript)
                .build();

            helper.add_output(sudt_output, sudt_amount.to_le_bytes().to_vec().into());

            helper.supply_sudt(
                &mut self.rpc_client,
                &mut self.indexer_client,
                from_lockscript.clone(),
                &self._genesis_info,
                sudt_amount,
                sudt_typescript.clone(),
            )?;
        }

        // add signature to pay tx fee
        let tx = helper.supply_capacity(
            &mut self.rpc_client,
            &mut self.indexer_client,
            from_lockscript,
            &self._genesis_info,
            tx_fee,
        )?;
        Ok(tx)
    }

    pub fn get_sudt_balance(&mut self, address: String, token_addr: H160) -> Result<u128, String> {
        let addr_lockscript: Script = Address::from_str(&address)?.payload().into();

        let sudt_typescript = get_sudt_lock_script(
            &self._settings.bridge_lock_sctipt.code_hash,
            &self._settings.sudt.code_hash,
            token_addr,
        );
        collect_sudt_amount(&mut self.indexer_client, addr_lockscript, sudt_typescript)
    }
}

pub fn covert_to_h256(mut tx_hash: &str) -> Result<H256> {
    if tx_hash.starts_with("0x") || tx_hash.starts_with("0X") {
        tx_hash = &tx_hash[2..];
    }
    if tx_hash.len() % 2 != 0 {
        anyhow::bail!(format!("Invalid hex string length: {}", tx_hash.len()))
    }
    let mut bytes = vec![0u8; tx_hash.len() / 2];
    hex_decode(tx_hash.as_bytes(), &mut bytes)
        .map_err(|err| anyhow::anyhow!("parse hex string failed: {:?}", err))?;
    H256::from_slice(&bytes).map_err(|e| anyhow::anyhow!("failed to covert tx hash: {}", e))
}

pub fn get_sudt_lock_script(
    bridge_lock_code_hash: &str,
    sudt_code_hash: &str,
    token_addr: H160,
) -> Script {
    let bridge_lockscript_code_hash =
        hex::decode(bridge_lock_code_hash).expect("wrong sudt_script code hash config");

    let bridge_lockscript: Script = Script::new_builder()
        .code_hash(Byte32::from_slice(&bridge_lockscript_code_hash).unwrap())
        .hash_type(DepType::Code.into())
        .args(token_addr.as_bytes().pack())
        .build();

    // 通过 type_script 校验 ctoken
    let sudt_typescript_code_hash =
        hex::decode(sudt_code_hash).expect("wrong sudt_script code hash config");
    Script::new_builder()
        .code_hash(Byte32::from_slice(&sudt_typescript_code_hash).unwrap())
        .hash_type(DepType::Code.into())
        .args(bridge_lockscript.calc_script_hash().as_bytes().pack())
        .build()
}
