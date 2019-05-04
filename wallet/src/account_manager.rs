use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, ChildNumber};
use bitcoin::util::key::PublicKey;
use secp256k1::{Secp256k1};

use std::error::Error;

pub struct AccountManager {
    account_key:    ExtendedPrivKey,
    external_index: u32,
    internal_index: u32,
}

impl AccountManager {
    pub fn from_account_key(account_key: ExtendedPrivKey) -> Self {
        Self {
            account_key,
            external_index: 0,
            internal_index: 0,
        }
    }

    pub fn next_external_pk(&mut self) -> Result<PublicKey, Box<Error>> {
        let path: &[ChildNumber] = &[
            ChildNumber::Normal{index: 0},
            ChildNumber::Normal{index: self.external_index},
        ];

        self.external_index += 1;

        let extended_priv_key = self.account_key.derive_priv(&Secp256k1::new(),  &path)?;
        let extended_pub_key = ExtendedPubKey::from_private(&Secp256k1::new(), &extended_priv_key);
        Ok(extended_pub_key.public_key)
    }

    pub fn next_internal_pk(&mut self) -> Result<PublicKey, Box<Error>> {
        let path: &[ChildNumber] = &[
            ChildNumber::Normal{index: 1},
            ChildNumber::Normal{index: self.internal_index},
        ];

        self.internal_index += 1;

        let extended_priv_key =self.account_key.derive_priv(&Secp256k1::new(), &path)?;
        let extended_pub_key = ExtendedPubKey::from_private(&Secp256k1::new(), &extended_priv_key);
        Ok(extended_pub_key.public_key)
    }
}