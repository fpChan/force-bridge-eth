#[cfg(target_arch = "riscv64")]
pub mod chain;
#[cfg(feature = "std")]
use mockall::predicate::*;
#[cfg(feature = "std")]
use mockall::*;

use crate::actions::calc_eth_bridge_lock_hash;
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::{
    bytes::Bytes,
    packed::{Byte32, Script},
    prelude::Pack,
};
use ckb_std::error::SysError;
use force_eth_types::config::{SUDT_CODE_HASH, UDT_LEN};
use force_eth_types::eth_recipient_cell::{ETHAddress, ETHRecipientDataView};
use molecule::prelude::{Builder, Byte, Entity};
use std::convert::TryFrom;

#[cfg_attr(feature = "std", automock)]
pub trait Adapter {
    fn load_output_data_by_trait(&self) -> Vec<Vec<u8>>;

    fn load_cell_data_by_trait(&self, index: usize, source: Source) -> Vec<u8>;

    fn load_cell_type_by_trait(
        &self,
        index: usize,
        source: Source,
    ) -> Result<Option<Script>, SysError>;
}

pub fn load_output_data(adapter: &dyn Adapter) -> Option<ETHRecipientDataView> {
    let data_list = adapter.load_output_data_by_trait();
    match data_list.len() {
        0 => None,
        1 => Some(
            ETHRecipientDataView::new(data_list[0].as_slice())
                .expect("ETHRecipientDataView coding error"),
        ),
        _ => panic!("outputs have more than 1 eth recipient cell"),
    }
}

pub fn get_sudt_amount_from_source(
    adapter: &dyn Adapter,
    source: Source,
    eth_bridge_lock_hash: &[u8],
) -> u128 {
    fn is_sudt_typescript(script: Option<Script>, lock_hash: &[u8]) -> bool {
        if script.is_none() {
            return false;
        }
        let script = script.unwrap();
        if script.code_hash().raw_data().as_ref() == SUDT_CODE_HASH.as_ref()
            && script.args().raw_data().as_ref() == lock_hash
            && script.hash_type() == 0u8.into()
        {
            return true;
        }
        return false;
    }

    let mut index = 0;
    let mut sudt_sum = 0;
    loop {
        let cell_type = adapter.load_cell_type_by_trait(index, source);
        match cell_type {
            Err(SysError::IndexOutOfBound) => break,
            Err(_err) => panic!("iter input return an error"),
            Ok(cell_type) => {
                if !(is_sudt_typescript(cell_type, eth_bridge_lock_hash)) {
                    index += 1;
                    continue;
                }

                let data = adapter.load_cell_data_by_trait(index, source);
                let mut buf = [0u8; UDT_LEN];
                if data.len() >= UDT_LEN {
                    buf.copy_from_slice(&data[0..UDT_LEN]);
                    sudt_sum += u128::from_le_bytes(buf)
                }
                index += 1;
            }
        }
    }
    sudt_sum
}

pub enum CellType {
    IndexOutOfBound,
    OtherErr,
    Success,
}

pub fn get_mock_load_output_data(token_amount: u128, fee: u128) -> (Vec<Vec<u8>>, [u8; 32]) {
    let data = ETHRecipientDataView {
        eth_recipient_address: ETHAddress::try_from(vec![0; 20]).unwrap(),
        eth_token_address: ETHAddress::try_from(vec![0; 20]).unwrap(),
        eth_lock_contract_address: ETHAddress::try_from(vec![0; 20]).unwrap(),
        eth_bridge_lock_hash: [1u8; 32],
        token_amount,
        fee,
    };
    let output_data = vec![data.clone().as_molecule_data().unwrap().to_vec()];
    let eth_bridge_lock_hash = calc_eth_bridge_lock_hash(
        data.eth_lock_contract_address,
        data.eth_token_address,
        &data.eth_bridge_lock_hash,
    );
    (output_data, eth_bridge_lock_hash)
}

pub fn get_mock_load_cell_type(
    cell_type: CellType,
    lock_hash: &[u8],
) -> Result<Option<Script>, SysError> {
    match cell_type {
        CellType::IndexOutOfBound => Err(SysError::IndexOutOfBound),
        CellType::OtherErr => Err(SysError::Unknown(1)),
        CellType::Success => {
            let sudt_sctipt = Script::new_builder()
                .code_hash(Byte32::from_slice(SUDT_CODE_HASH.as_ref()).unwrap())
                .hash_type(Byte::new(0))
                .args(Bytes::from(lock_hash).pack())
                .build();
            Ok(Some(sudt_sctipt))
        }
    }
}

pub fn set_mock_chain_data(
    mut mock: MockAdapter,
    mol_data_vec: Vec<Vec<u8>>,
    lock_hash: [u8; 32],
    input_sudt_cell_data: String,
    output_sudt_cell_data: String,
) -> MockAdapter {
    mock.expect_load_output_data_by_trait()
        .times(1)
        .returning(move || mol_data_vec.clone());
    mock.expect_load_cell_type_by_trait()
        .times(4)
        .returning(move |index, _| {
            if index == 0 {
                get_mock_load_cell_type(CellType::Success, &lock_hash)
            } else {
                get_mock_load_cell_type(CellType::IndexOutOfBound, &lock_hash)
            }
        });

    mock.expect_load_cell_data_by_trait()
        .times(2)
        .returning(move |_, input| {
            if input == Source::Input {
                hex::decode(&input_sudt_cell_data).unwrap()
            } else {
                hex::decode(&output_sudt_cell_data).unwrap()
            }
        });
    mock
}
