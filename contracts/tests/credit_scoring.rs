use stellar_contract_sdk::{contractimpl, Address, Env, Symbol};
use credchain_contracts::{CredChainContract, CreditProfile, LoanRecord};

#[test]
fn test_initialize_contract() {
    let env = Env::default();
    let admin = Address::random(&env);
    let cred_token = Address::random(&env);
    
    CredChainContract::initialize(env.clone(), admin.clone(), cred_token);
    
    // Verify initialization
    let stored_admin: Address = env.storage().instance().get(&Symbol::short("ADMIN")).unwrap();
    assert_eq!(stored_admin, admin);
}

#[test]
fn test_create_credit_profile() {
    let env = Env::default();
    let admin = Address::random(&env);
    let cred_token = Address::random(&env);
    let user = Address::random(&env);
    
    CredChainContract::initialize(env.clone(), admin.clone(), cred_token);
    CredChainContract::create_credit_profile(env.clone(), user.clone());
    
    let profile = CredChainContract::get_credit_profile(env.clone(), user.clone());
    assert_eq!(profile.credit_score, 300);
    assert_eq!(profile.reputation_score, 50);
    assert_eq!(profile.transaction_count, 0);
    assert!(!profile.verification_status);
}

#[test]
fn test_credit_score_update_repayment() {
    let env = Env::default();
    let admin = Address::random(&env);
    let cred_token = Address::random(&env);
    let user = Address::random(&env);
    
    CredChainContract::initialize(env.clone(), admin.clone(), cred_token);
    CredChainContract::create_credit_profile(env.clone(), user.clone());
    
    // Test successful repayment
    CredChainContract::update_credit_score(env.clone(), user.clone(), Symbol::short("REPAY"), 1000);
    
    let profile = CredChainContract::get_credit_profile(env.clone(), user.clone());
    assert_eq!(profile.credit_score, 310);
    assert_eq!(profile.repayment_history.len(), 1);
    assert!(profile.repayment_history.get(0).unwrap());
}

#[test]
fn test_credit_score_update_default() {
    let env = Env::default();
    let admin = Address::random(&env);
    let cred_token = Address::random(&env);
    let user = Address::random(&env);
    
    CredChainContract::initialize(env.clone(), admin.clone(), cred_token);
    CredChainContract::create_credit_profile(env.clone(), user.clone());
    
    // Test default
    CredChainContract::update_credit_score(env.clone(), user.clone(), Symbol::short("DEFAULT"), 1000);
    
    let profile = CredChainContract::get_credit_profile(env.clone(), user.clone());
    assert_eq!(profile.credit_score, 300); // Should stay at minimum
    assert_eq!(profile.repayment_history.len(), 1);
    assert!(!profile.repayment_history.get(0).unwrap());
}

#[test]
fn test_loan_record_creation() {
    let env = Env::default();
    let admin = Address::random(&env);
    let cred_token = Address::random(&env);
    let borrower = Address::random(&env);
    let lender = Address::random(&env);
    
    CredChainContract::initialize(env.clone(), admin.clone(), cred_token);
    CredChainContract::create_credit_profile(env.clone(), borrower.clone());
    
    CredChainContract::create_loan_record(
        env.clone(),
        borrower.clone(),
        lender.clone(),
        1000,
        500, // 5% interest rate
        30,   // 30 days
        None, // No collateral
    );
    
    let profile = CredChainContract::get_credit_profile(env.clone(), borrower.clone());
    assert_eq!(profile.transaction_count, 1);
}

#[test]
fn test_reputation_review() {
    let env = Env::default();
    let admin = Address::random(&env);
    let cred_token = Address::random(&env);
    let reviewer = Address::random(&env);
    let reviewed_user = Address::random(&env);
    
    CredChainContract::initialize(env.clone(), admin.clone(), cred_token);
    CredChainContract::create_credit_profile(env.clone(), reviewed_user.clone());
    
    // Add a 5-star review
    CredChainContract::add_reputation_review(
        env.clone(),
        reviewer.clone(),
        reviewed_user.clone(),
        5,
        "Excellent borrower!".into(),
    );
    
    let profile = CredChainContract::get_credit_profile(env.clone(), reviewed_user.clone());
    assert_eq!(profile.reputation_score, 54); // 50 + (5-3)*2
}

#[test]
fn test_user_verification() {
    let env = Env::default();
    let admin = Address::random(&env);
    let cred_token = Address::random(&env);
    let user = Address::random(&env);
    
    CredChainContract::initialize(env.clone(), admin.clone(), cred_token);
    CredChainContract::create_credit_profile(env.clone(), user.clone());
    
    // Verify user
    CredChainContract::verify_user(env.clone(), admin.clone(), user.clone());
    
    let profile = CredChainContract::get_credit_profile(env.clone(), user.clone());
    assert!(profile.verification_status);
    assert_eq!(profile.credit_score, 325); // 300 + 25 verification bonus
}
