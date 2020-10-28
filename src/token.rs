use near_sdk::{
    near_bindgen,
	borsh::{
		self,
		BorshDeserialize, 
		BorshSerialize
	}
};

/** 
 * @title Non-funbile-token
 */

/**
 * @notice The state struct for the Flux Protocol implementation 
 */
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct NonFungibleToken {

}
impl Default for NonFungibleToken {
    fn default() -> Self {
        panic!("Contract should be initialized before usage")
    }
}

#[near_bindgen]
impl NonFungibleToken {
}


#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
	use super::*;
	mod utils;
	use utils::{ntoy, ExternalUser, init_markets_contract};
    use near_sdk::MockedBlockchain;
    use near_sdk::{VMContext, testing_env};
	use near_runtime_standalone::{RuntimeStandalone};
	use near_primitives::transaction::{ExecutionStatus, ExecutionOutcome};

}