use coin_ethereum::ethapi::{EthTxReq, EthMessageSignReq};
use crate::message_handler::encode_message;
use coin_ethereum::transaction::Transaction;
use coin_ethereum::types::Action;
use ethereum_types::{Address, U256};
use hex;
use prost::Message;
use std::str::FromStr;
use crate::error_handling::Result;

pub fn sign_eth_transaction(data: &[u8]) -> Result<Vec<u8>> {

    let input: EthTxReq = EthTxReq::decode(data).expect("imkey_illegal_param");
    let data_vec = if input.data.starts_with("0x") {
        hex::decode(&input.data[2..]).unwrap()
    } else {
        hex::decode(&input.data).unwrap()
    };

    let mut to = input.to;
    if to.starts_with("0x"){
        to = to[2..].to_string();
    }

    let eth_tx = Transaction {
        nonce: U256::from_dec_str(&input.nonce).unwrap(),
        gas_price: U256::from_dec_str(&input.gas_price).unwrap(),
        gas_limit: U256::from_dec_str(&input.gas_limit).unwrap(),
        to: Action::Call(Address::from_str(&to).unwrap()),
        value: U256::from_dec_str(&input.value).unwrap(),
        data: Vec::from(data_vec.as_slice()),
    };

    println!("trans create ..");

    let chain_id = input.chain_id.parse::<u64>().unwrap();
    let tx_out = eth_tx.sign(
        Some(chain_id),
        &input.path,
        &input.payment,
        &input.receiver,
        &input.sender,
        &input.fee,
    )?;
    println!("signed..");
    encode_message(tx_out)
}

pub fn sign_eth_message(data: &[u8]) -> Result<Vec<u8>> {
    let input: EthMessageSignReq = EthMessageSignReq::decode(data).expect("imkey_illegal_param");
    let signed = Transaction::sign_persional_message(input).unwrap();
    encode_message(signed)
}

#[cfg(test)]
mod test{
    use crate::ethapi::EthTxInput;
    use crate::ethereum_signer::sign_eth_transaction;

    #[test]
    fn test_eth_trans(){
        let eth_input = EthTxInput{
            nonce: "8".to_string(),
            gas_price: "20000000008".to_string(),
            gas_limit: "189000".to_string(),
            to: "3535353535353535353535353535353535353535".to_string(),
            value: "512".to_string(),
            data: "".to_string(),
            chain_id: "".to_string(),
            path: "".to_string(),
            payment: "".to_string(),
            receiver: "".to_string(),
            sender: "".to_string(),
            fee: "".to_string()
        };

        // let eth_tx = Transaction {
        //     nonce: U256::from_dec_str(&input.nonce).unwrap(),
        //     gas_price: U256::from_dec_str(&input.gas_price).unwrap(),
        //     gas_limit: U256::from_dec_str(&input.gas_limit).unwrap(),
        //     to: Action::Call(Address::from_str(&input.to).unwrap()),
        //     value: U256::from_dec_str(&input.value).unwrap(),
        //     data: Vec::from(data),
        // };
    }

}

