use stellar_contract_sdk::{
    contractimpl, contracttype, Address, Env, IntoVal, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreditProfile {
    pub user_address: Address,
    pub credit_score: u32,
    pub reputation_score: u32,
    pub transaction_count: u64,
    pub repayment_history: Vec<bool>,
    pub last_updated: u64,
    pub verification_status: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoanRecord {
    pub borrower: Address,
    pub lender: Address,
    pub amount: i128,
    pub interest_rate: u32,
    pub term_days: u32,
    pub start_date: u64,
    pub repayment_status: bool,
    pub collateral: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReputationReview {
    pub reviewer: Address,
    pub reviewed_user: Address,
    pub rating: u8, // 1-5 stars
    pub comment: String,
    pub timestamp: u64,
    pub transaction_id: Option<Symbol>,
}

// Contract storage keys
const CREDIT_PROFILES: Symbol = Symbol::short("CRED_PRO");
const LOAN_RECORDS: Symbol = Symbol::short("LOAN_REC");
const REPUTATION_REVIEWS: Symbol = Symbol::short("REP_REV");
const ADMIN_ADDRESS: Symbol = Symbol::short("ADMIN");
const CRED_TOKEN: Symbol = Symbol::short("CRED_TOK");

pub struct CredChainContract;

#[contractimpl]
impl CredChainContract {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address, cred_token_address: Address) {
        if env.storage().instance().has(&ADMIN_ADDRESS) {
            panic!("Contract already initialized");
        }
        
        env.storage().instance().set(&ADMIN_ADDRESS, &admin);
        env.storage().instance().set(&CRED_TOKEN, &cred_token_address);
    }

    // Create or update credit profile
    pub fn create_credit_profile(env: Env, user: Address) {
        let key = user.clone();
        
        if env.storage().instance().has(&key) {
            panic!("Credit profile already exists");
        }

        let profile = CreditProfile {
            user_address: user.clone(),
            credit_score: 300, // Base score
            reputation_score: 50, // Base reputation
            transaction_count: 0,
            repayment_history: Vec::new(&env),
            last_updated: env.ledger().timestamp(),
            verification_status: false,
        };

        env.storage().instance().set(&key, &profile);
    }

    // Get credit profile
    pub fn get_credit_profile(env: Env, user: Address) -> CreditProfile {
        let key = user.clone();
        
        if !env.storage().instance().has(&key) {
            panic!("Credit profile not found");
        }

        env.storage().instance().get(&key).unwrap()
    }

    // Update credit score based on transaction
    pub fn update_credit_score(env: Env, user: Address, transaction_type: Symbol, amount: i128) {
        let mut profile = Self::get_credit_profile(env.clone(), user.clone());
        
        match transaction_type {
            Symbol::short("REPAY") => {
                // Successful repayment increases score
                profile.credit_score = (profile.credit_score + 10).min(850);
                profile.repayment_history.push_back(true);
            }
            Symbol::short("DEFAULT") => {
                // Default decreases score
                profile.credit_score = (profile.credit_score - 50).max(300);
                profile.repayment_history.push_back(false);
            }
            Symbol::short("BORROW") => {
                // Borrowing activity
                profile.transaction_count += 1;
                if amount > 1000 {
                    profile.credit_score = (profile.credit_score + 5).min(850);
                }
            }
            _ => panic!("Invalid transaction type"),
        }

        profile.last_updated = env.ledger().timestamp();
        env.storage().instance().set(&user, &profile);
    }

    // Add reputation review
    pub fn add_reputation_review(env: Env, reviewer: Address, reviewed_user: Address, rating: u8, comment: String) {
        if rating < 1 || rating > 5 {
            panic!("Rating must be between 1 and 5");
        }

        let review = ReputationReview {
            reviewer: reviewer.clone(),
            reviewed_user: reviewed_user.clone(),
            rating,
            comment,
            timestamp: env.ledger().timestamp(),
            transaction_id: None,
        };

        // Store the review
        let review_key = (reviewed_user.clone(), reviewer.clone());
        env.storage().instance().set(&review_key, &review);

        // Update reputation score
        let mut profile = Self::get_credit_profile(env.clone(), reviewed_user.clone());
        
        // Calculate new reputation based on weighted average
        let rating_impact = rating as i32 - 3; // Neutral rating is 3
        profile.reputation_score = (profile.reputation_score as i32 + rating_impact * 2).max(0).min(100) as u32;
        
        env.storage().instance().set(&reviewed_user, &profile);
    }

    // Create loan record
    pub fn create_loan_record(
        env: Env,
        borrower: Address,
        lender: Address,
        amount: i128,
        interest_rate: u32,
        term_days: u32,
        collateral: Option<Address>,
    ) {
        if amount <= 0 {
            panic!("Loan amount must be positive");
        }

        let loan_id = env.ledger().sequence();
        let loan = LoanRecord {
            borrower: borrower.clone(),
            lender: lender.clone(),
            amount,
            interest_rate,
            term_days,
            start_date: env.ledger().timestamp(),
            repayment_status: false,
            collateral,
        };

        let loan_key = Symbol::short(&format!("LOAN_{}", loan_id));
        env.storage().instance().set(&loan_key, &loan);

        // Update borrower's transaction count
        Self::update_credit_score(env.clone(), borrower, Symbol::short("BORROW"), amount);
    }

    // Mark loan as repaid
    pub fn repay_loan(env: Env, loan_id: Symbol) {
        let loan_key = loan_id.clone();
        
        if !env.storage().instance().has(&loan_key) {
            panic!("Loan record not found");
        }

        let mut loan: LoanRecord = env.storage().instance().get(&loan_key).unwrap();
        
        if loan.repayment_status {
            panic!("Loan already repaid");
        }

        loan.repayment_status = true;
        env.storage().instance().set(&loan_key, &loan);

        // Update borrower's credit score
        Self::update_credit_score(env.clone(), loan.borrower, Symbol::short("REPAY"), loan.amount);
    }

    // Mark loan as defaulted
    pub fn default_loan(env: Env, loan_id: Symbol) {
        let loan_key = loan_id.clone();
        
        if !env.storage().instance().has(&loan_key) {
            panic!("Loan record not found");
        }

        let mut loan: LoanRecord = env.storage().instance().get(&loan_key).unwrap();
        
        if loan.repayment_status {
            panic!("Cannot default a repaid loan");
        }

        loan.repayment_status = false;
        env.storage().instance().set(&loan_key, &loan);

        // Update borrower's credit score
        Self::update_credit_score(env.clone(), loan.borrower, Symbol::short("DEFAULT"), loan.amount);
    }

    // Verify user identity (admin only)
    pub fn verify_user(env: Env, admin: Address, user: Address) {
        Self::require_admin(env.clone(), admin);
        
        let mut profile = Self::get_credit_profile(env.clone(), user.clone());
        profile.verification_status = true;
        profile.credit_score = (profile.credit_score + 25).min(850); // Verification bonus
        
        env.storage().instance().set(&user, &profile);
    }

    // Get loan record
    pub fn get_loan_record(env: Env, loan_id: Symbol) -> LoanRecord {
        let loan_key = loan_id.clone();
        
        if !env.storage().instance().has(&loan_key) {
            panic!("Loan record not found");
        }

        env.storage().instance().get(&loan_key).unwrap()
    }

    // Get user's loan history
    pub fn get_user_loans(env: Env, user: Address) -> Vec<LoanRecord> {
        let mut loans = Vec::new(&env);
        
        // This is a simplified implementation
        // In production, you'd need a more efficient way to query user loans
        // For now, we'll return an empty vector as placeholder
        loans
    }

    // Helper function to check admin permissions
    fn require_admin(env: Env, caller: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN_ADDRESS).unwrap();
        if caller != admin {
            panic!("Admin access required");
        }
    }

    // Calculate credit score factors (for transparency)
    pub fn get_score_factors(env: Env, user: Address) -> Vec<(String, u32)> {
        let profile = Self::get_credit_profile(env.clone(), user.clone());
        let mut factors = Vec::new(&env);
        
        factors.push_back(("Base Score".into_val(&env), 300));
        factors.push_back(("Transaction History".into_val(&env), profile.transaction_count as u32 * 2));
        factors.push_back(("Reputation".into_val(&env), profile.reputation_score));
        
        if profile.verification_status {
            factors.push_back(("Verification Bonus".into_val(&env), 25));
        }

        factors
    }
}
