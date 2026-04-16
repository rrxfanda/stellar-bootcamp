#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol, Vec, String, Map
};

#[contract]
pub struct TipJar;

// =======================
// STORAGE KEYS
// =======================
const TOTAL: Symbol = symbol_short!("TOTAL");
const COUNT: Symbol = symbol_short!("COUNT");
const TIPS: Symbol = symbol_short!("TIPS");
const USER_TOTAL: Symbol = symbol_short!("USR_TOT");
const CREATOR_TOTAL: Symbol = symbol_short!("CRT_TOT");

// =======================
// STRUCT
// =======================
#[contracttype]
#[derive(Clone)]
pub struct Tip {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub message: String,
    pub timestamp: u64,
}

// =======================
// CONTRACT IMPLEMENTATION
// =======================
#[contractimpl]
impl TipJar {

    // =======================
    // SEND TIP
    // =======================
    pub fn tip(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        message: String,
    ) {
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be > 0");
        }

        // =======================
        // GLOBAL STATS
        // =======================
        let mut total: i128 = env.storage().instance().get(&TOTAL).unwrap_or(0);
        let mut count: i128 = env.storage().instance().get(&COUNT).unwrap_or(0);

        total += amount;
        count += 1;

        env.storage().instance().set(&TOTAL, &total);
        env.storage().instance().set(&COUNT, &count);

        // =======================
        // USER TOTAL
        // =======================
        let mut user_map: Map<Address, i128> =
            env.storage().instance().get(&USER_TOTAL).unwrap_or(Map::new(&env));

        let user_total = user_map.get(from.clone()).unwrap_or(0) + amount;
        user_map.set(from.clone(), user_total);

        env.storage().instance().set(&USER_TOTAL, &user_map);

        // =======================
        // CREATOR TOTAL
        // =======================
        let mut creator_map: Map<Address, i128> =
            env.storage().instance().get(&CREATOR_TOTAL).unwrap_or(Map::new(&env));

        let creator_total = creator_map.get(to.clone()).unwrap_or(0) + amount;
        creator_map.set(to.clone(), creator_total);

        env.storage().instance().set(&CREATOR_TOTAL, &creator_map);

        // =======================
        // TIP HISTORY
        // =======================
        let mut tips: Vec<Tip> =
            env.storage().instance().get(&TIPS).unwrap_or(Vec::new(&env));

        let new_tip = Tip {
            from,
            to,
            amount,
            message,
            timestamp: env.ledger().timestamp(),
        };

        tips.push_back(new_tip);
        env.storage().instance().set(&TIPS, &tips);
    }

    // =======================
    // GET GLOBAL STATS
    // =======================
    pub fn get_total(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL).unwrap_or(0)
    }

    pub fn get_count(env: Env) -> i128 {
        env.storage().instance().get(&COUNT).unwrap_or(0)
    }

    // =======================
    // GET ALL TIPS
    // =======================
    pub fn get_all_tips(env: Env) -> Vec<Tip> {
        env.storage().instance().get(&TIPS).unwrap_or(Vec::new(&env))
    }

    // =======================
    // GET TIP BY INDEX
    // =======================
    pub fn get_tip(env: Env, index: u32) -> Tip {
        let tips: Vec<Tip> =
            env.storage().instance().get(&TIPS).unwrap_or(Vec::new(&env));

        tips.get(index).unwrap()
    }

    // =======================
    // GET USER TOTAL
    // =======================
    pub fn get_user_total(env: Env, user: Address) -> i128 {
        let map: Map<Address, i128> =
            env.storage().instance().get(&USER_TOTAL).unwrap_or(Map::new(&env));

        map.get(user).unwrap_or(0)
    }

    // =======================
    // GET CREATOR TOTAL
    // =======================
    pub fn get_creator_total(env: Env, creator: Address) -> i128 {
        let map: Map<Address, i128> =
            env.storage().instance().get(&CREATOR_TOTAL).unwrap_or(Map::new(&env));

        map.get(creator).unwrap_or(0)
    }

    // =======================
    // RESET (DEBUG PURPOSE)
    // =======================
    pub fn reset(env: Env) {
        env.storage().instance().set(&TOTAL, &0);
        env.storage().instance().set(&COUNT, &0);

        let empty_vec: Vec<Tip> = Vec::new(&env);
        env.storage().instance().set(&TIPS, &empty_vec);

        let empty_map1: Map<Address, i128> = Map::new(&env);
        let empty_map2: Map<Address, i128> = Map::new(&env);

        env.storage().instance().set(&USER_TOTAL, &empty_map1);
        env.storage().instance().set(&CREATOR_TOTAL, &empty_map2);
    }
}