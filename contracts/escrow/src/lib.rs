#![no_std]

mod admin;
mod errors;
mod events;
mod storage;
mod test;

pub use errors::EscrowError;
pub use storage::{DataKey, EscrowInfo, EscrowState};

use soroban_sdk::{contract, contractimpl, Address, Env};

use admin::transfer_token;
use storage::DataKey::{Amount, Arbiter, Buyer, BuyerApproved, Deadline, Seller, SellerDelivered, State, TokenContract};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(
        env: Env,
        buyer: Address,
        seller: Address,
        arbiter: Address,
        token_contract: Address,
        amount: i128,
        deadline_ledger: u32,
    ) -> Result<(), EscrowError> {
        if env.storage().instance().has(&State) {
            return Err(EscrowError::AlreadyInitialized);
        }
        if deadline_ledger <= env.ledger().sequence() {
            panic!("Deadline must be in the future");
        }
        env.storage().instance().set(&Buyer, &buyer);
        env.storage().instance().set(&Seller, &seller);
        env.storage().instance().set(&Arbiter, &arbiter);
        env.storage().instance().set(&TokenContract, &token_contract);
        env.storage().instance().set(&Amount, &amount);
        env.storage().instance().set(&Deadline, &deadline_ledger);
        env.storage().instance().set(&State, &EscrowState::Created);
        env.storage().instance().set(&BuyerApproved, &false);
        env.storage().instance().set(&SellerDelivered, &false);
        events::escrow_created(&env, &buyer, &seller, amount);
        Ok(())
    }

    pub fn fund(env: Env) -> Result<(), EscrowError> {
        let state: EscrowState = env.storage().instance().get(&State).ok_or(EscrowError::NotInitialized)?;
        if state != EscrowState::Created { return Err(EscrowError::InvalidState); }
        let buyer: Address = env.storage().instance().get(&Buyer).unwrap();
        let amount: i128 = env.storage().instance().get(&Amount).unwrap();
        buyer.require_auth();
        transfer_token(&env, &buyer, &env.current_contract_address(), amount);
        env.storage().instance().set(&State, &EscrowState::Funded);
        events::escrow_funded(&env, &buyer, amount);
        Ok(())
    }

    pub fn mark_delivered(env: Env) -> Result<(), EscrowError> {
        let state: EscrowState = env.storage().instance().get(&State).ok_or(EscrowError::NotInitialized)?;
        if state != EscrowState::Funded { return Err(EscrowError::InvalidState); }
        let seller: Address = env.storage().instance().get(&Seller).unwrap();
        seller.require_auth();
        env.storage().instance().set(&SellerDelivered, &true);
        env.storage().instance().set(&State, &EscrowState::Delivered);
        events::delivery_marked(&env, &seller);
        Ok(())
    }

    pub fn approve_delivery(env: Env) -> Result<(), EscrowError> {
        let state: EscrowState = env.storage().instance().get(&State).ok_or(EscrowError::NotInitialized)?;
        if state != EscrowState::Delivered { return Err(EscrowError::InvalidState); }
        let buyer: Address = env.storage().instance().get(&Buyer).unwrap();
        buyer.require_auth();
        Self::release_to_seller(env)
    }

    pub fn request_refund(env: Env) -> Result<(), EscrowError> {
        let state: EscrowState = env.storage().instance().get(&State).ok_or(EscrowError::NotInitialized)?;
        let buyer: Address = env.storage().instance().get(&Buyer).unwrap();
        let deadline: u32 = env.storage().instance().get(&Deadline).unwrap();
        buyer.require_auth();
        let can_refund = matches!(state, EscrowState::Funded | EscrowState::Delivered)
            && env.ledger().sequence() > deadline;
        if !can_refund { return Err(EscrowError::DeadlineNotReached); }
        Self::refund_to_buyer(env)
    }

    pub fn resolve_dispute(env: Env, release_to_seller: bool) -> Result<(), EscrowError> {
        let state: EscrowState = env.storage().instance().get(&State).ok_or(EscrowError::NotInitialized)?;
        if !matches!(state, EscrowState::Funded | EscrowState::Delivered) {
            return Err(EscrowError::InvalidState);
        }
        let arbiter: Address = env.storage().instance().get(&Arbiter).unwrap();
        arbiter.require_auth();
        if release_to_seller { Self::release_to_seller(env) } else { Self::refund_to_buyer(env) }
    }

    pub fn get_escrow_info(env: Env) -> EscrowInfo {
        EscrowInfo {
            buyer: env.storage().instance().get(&Buyer).unwrap(),
            seller: env.storage().instance().get(&Seller).unwrap(),
            arbiter: env.storage().instance().get(&Arbiter).unwrap(),
            token_contract: env.storage().instance().get(&TokenContract).unwrap(),
            amount: env.storage().instance().get(&Amount).unwrap(),
            deadline: env.storage().instance().get(&Deadline).unwrap(),
            state: env.storage().instance().get(&State).unwrap(),
        }
    }

    pub fn get_state(env: Env) -> EscrowState {
        env.storage().instance().get(&State).unwrap_or(EscrowState::Created)
    }

    pub fn is_deadline_passed(env: Env) -> bool {
        let deadline: u32 = env.storage().instance().get(&Deadline).unwrap_or(0);
        env.ledger().sequence() > deadline
    }
}

impl EscrowContract {
    fn release_to_seller(env: Env) -> Result<(), EscrowError> {
        let seller: Address = env.storage().instance().get(&Seller).unwrap();
        let amount: i128 = env.storage().instance().get(&Amount).unwrap();
        transfer_token(&env, &env.current_contract_address(), &seller, amount);
        env.storage().instance().set(&State, &EscrowState::Completed);
        events::funds_released(&env, &seller, amount);
        Ok(())
    }

    fn refund_to_buyer(env: Env) -> Result<(), EscrowError> {
        let buyer: Address = env.storage().instance().get(&Buyer).unwrap();
        let amount: i128 = env.storage().instance().get(&Amount).unwrap();
        transfer_token(&env, &env.current_contract_address(), &buyer, amount);
        env.storage().instance().set(&State, &EscrowState::Refunded);
        events::funds_refunded(&env, &buyer, amount);
        Ok(())
    }
}
