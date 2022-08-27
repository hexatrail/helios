use ethers::utils::keccak256;
use ethers::utils::rlp::{decode_list, RlpStream};

use crate::execution::types::Proof;

pub fn verify_proof(proof: &Vec<Vec<u8>>, root: &Vec<u8>, path: &Vec<u8>, value: &Vec<u8>) -> bool {
    let mut expected_hash = root.clone();
    let mut path_offset = 0;

    for (i, node) in proof.iter().enumerate() {
        if expected_hash != keccak256(node).to_vec() {
            return false;
        }

        let node_list: Vec<Vec<u8>> = decode_list(node);

        if node_list.len() == 17 {
            let nibble = get_nibble(&path, path_offset);
            expected_hash = node_list[nibble as usize].clone();

            path_offset += 1;
        } else if node_list.len() == 2 {
            if i == proof.len() - 1 {
                // exclusion proof
                if &node_list[0][skip_length(&node_list[0])..] != &path[path_offset..]
                    && value[0] == 0x80
                {
                    return true;
                }

                // inclusion proof
                if &node_list[1] != value {
                    return false;
                }
            } else {
                panic!("not implemented");
            }
        } else {
            return false;
        }
    }

    true
}

fn skip_length(node: &Vec<u8>) -> usize {
    let nibble = get_nibble(node, 0);
    match nibble {
        2 => 2,
        3 => 1,
        _ => 0,
    }
}

fn get_nibble(path: &Vec<u8>, offset: usize) -> u8 {
    let byte = path[offset / 2];
    if offset % 2 == 0 {
        byte >> 4
    } else {
        byte & 0xF
    }
}

pub fn encode_account(proof: &Proof) -> Vec<u8> {
    let mut stream = RlpStream::new_list(4);
    stream.append(&proof.nonce);
    stream.append(&proof.balance);
    stream.append(&proof.storage_hash);
    stream.append(&proof.code_hash);
    let encoded = stream.out();
    encoded.to_vec()
}
