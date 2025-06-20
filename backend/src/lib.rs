use candid::{CandidType, Principal};
use ic_cdk::{api::caller, query, update};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UserRole {
    Manager,
    Admin,
    Unauthorized,
}

#[derive(CandidType, Serialize, Deserialize, Default)]
pub struct UserRegistry {
    users: HashMap<Principal, UserRole>,
}

thread_local! {
    static USER_REGISTRY: RefCell<UserRegistry> = RefCell::new(UserRegistry::default());
}

#[update]
fn add_initial_admin(admin_principal: Principal) -> Result<String, String> {
    USER_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();

        let has_admin = registry.users.values().any(|role| matches!(role, UserRole::Admin));
        if has_admin {
            return Err("Initial admin already exists".to_string());
        }

        registry.users.insert(admin_principal, UserRole::Admin);
        Ok(format!("Initial admin added: {}", admin_principal))
    })
}

#[update]
fn add_manager(manager_principal: Principal) -> Result<String, String> {
    let caller_principal = caller();

    USER_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();

        match registry.users.get(&caller_principal) {
            Some(UserRole::Admin) => {
                if registry.users.contains_key(&manager_principal) {
                    return Err("User already exists".to_string());
                }

                registry.users.insert(manager_principal, UserRole::Manager);
                Ok(format!("Manager added: {}", manager_principal))
            }
            _ => Err("Only admins can add managers".to_string()),
        }
    })
}

#[query]
fn get_user_role() -> UserRole {
    let caller_principal = caller();

    USER_REGISTRY.with(|registry| {
        let registry = registry.borrow();
        registry.users.get(&caller_principal)
            .cloned()
            .unwrap_or(UserRole::Unauthorized)
    })
}

#[query]
fn get_all_users() -> Vec<(Principal, UserRole)> {
    let caller_principal = caller();

    USER_REGISTRY.with(|registry| {
        let registry = registry.borrow();

        match registry.users.get(&caller_principal) {
            Some(UserRole::Admin) => {
                registry.users.iter()
                    .map(|(k, v)| (*k, v.clone()))
                    .collect()
            }
            _ => vec![],
        }
    })
}

#[update]
fn remove_user(user_principal: Principal) -> Result<String, String> {
    let caller_principal = caller();

    USER_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();

        match registry.users.get(&caller_principal) {
            Some(UserRole::Admin) => {
                match registry.users.remove(&user_principal) {
                    Some(role) => Ok(format!("User removed: {} (role: {:?})", user_principal, role)),
                    None => Err("User not found".to_string()),
                }
            }
            _ => Err("Only admins can remove users".to_string()),
        }
    })
}

ic_cdk::export_candid!();