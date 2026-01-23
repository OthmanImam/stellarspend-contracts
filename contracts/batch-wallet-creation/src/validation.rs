//! Validation utilities for batch wallet creation.

use soroban_sdk::{Address, Env, Vec};
use crate::types::{WalletCreateRequest, MAX_BATCH_SIZE};

/// Validation error types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    /// Invalid owner address
    InvalidAddress,
    /// Batch is empty
    EmptyBatch,
    /// Batch size exceeds maximum
    BatchTooLarge,
    /// Wallet already exists
    WalletAlreadyExists,
}

/// Validates a batch of wallet creation requests.
pub fn validate_batch(_env: &Env, requests: &Vec<WalletCreateRequest>) -> Result<(), ValidationError> {
    if requests.is_empty() {
        return Err(ValidationError::EmptyBatch);
    }
    if requests.len() > MAX_BATCH_SIZE {
        return Err(ValidationError::BatchTooLarge);
    }
    for request in requests.iter() {
        validate_address(_env, &request.owner)?;
    }
    Ok(())
}

/// Validates an owner address.
pub fn validate_address(_env: &Env, _address: &Address) -> Result<(), ValidationError> {
    // For now, assume all addresses are valid
    Ok(())
}

/// Checks if a wallet already exists for the given address.
pub fn wallet_exists(env: &Env, address: &Address) -> bool {
    use crate::types::DataKey;
    env.storage().persistent().has(&DataKey::Wallets(address.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, vec};

    #[test]
    fn test_validate_batch_empty() {
        let env = Env::default();
        let requests = vec![&env];
        assert_eq!(validate_batch(&env, &requests), Err(ValidationError::EmptyBatch));
    }

    #[test]
    fn test_validate_batch_too_large() {
        let env = Env::default();
        let mut requests = vec![&env];
        for _ in 0..MAX_BATCH_SIZE + 1 {
            requests.push_back(WalletCreateRequest {
                owner: Address::generate(&env),
            });
        }
        assert_eq!(validate_batch(&env, &requests), Err(ValidationError::BatchTooLarge));
    }

    #[test]
    fn test_validate_batch_valid() {
        let env = Env::default();
        let requests = vec![
            &env,
            WalletCreateRequest {
                owner: Address::generate(&env),
            },
        ];
        assert!(validate_batch(&env, &requests).is_ok());
    }

    #[test]
    fn test_validate_address() {
        let env = Env::default();
        let address = Address::generate(&env);
        assert!(validate_address(&env, &address).is_ok());
    }
}