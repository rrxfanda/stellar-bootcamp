#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, String, Symbol, Address, Vec, symbol_short
};

// =======================
// STRUCT
// =======================

#[contracttype]
#[derive(Clone)]
pub struct MenuItem {
    pub id: u32,
    pub name: String,
    pub price: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Order {
    pub id: u32,
    pub items: Vec<u32>,
    pub total: u32,
    pub customer: Address,
    pub status: Symbol,
}

// =======================
// STORAGE KEY
// =======================

const ADMIN: Symbol = symbol_short!("ADMIN");

// =======================
// CONTRACT
// =======================

#[contract]
pub struct RestaurantContract;

#[contractimpl]
impl RestaurantContract {

    // =======================
    // INIT ADMIN
    // =======================
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
    }

    fn check_admin(env: &Env, user: &Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if admin != *user {
            panic!("Not authorized");
        }
    }

    // =======================
    // MENU CRUD
    // =======================

    // CREATE
    pub fn add_menu(env: Env, user: Address, id: u32, name: String, price: u32) {
        user.require_auth();
        Self::check_admin(&env, &user);

        if env.storage().instance().has(&id) {
            panic!("Menu already exists");
        }

        let item = MenuItem { id, name, price };
        env.storage().instance().set(&id, &item);
    }

    // READ
    pub fn get_menu(env: Env, id: u32) -> MenuItem {
        env.storage()
            .instance()
            .get(&id)
            .expect("Menu not found")
    }

    // UPDATE
    pub fn update_menu(env: Env, user: Address, id: u32, name: String, price: u32) {
        user.require_auth();
        Self::check_admin(&env, &user);

        let mut item: MenuItem = env.storage().instance().get(&id).unwrap();

        item.name = name;
        item.price = price;

        env.storage().instance().set(&id, &item);
    }

    // DELETE
    pub fn delete_menu(env: Env, user: Address, id: u32) {
        user.require_auth();
        Self::check_admin(&env, &user);

        if !env.storage().instance().has(&id) {
            panic!("Menu not found");
        }

        env.storage().instance().remove(&id);
    }

    // =======================
    // ORDER CRUD
    // =======================

    // CREATE ORDER
    pub fn create_order(
        env: Env,
        id: u32,
        items: Vec<u32>,
        customer: Address,
    ) {
        customer.require_auth();

        let mut total: u32 = 0;

        // hitung total (O(n))
        for item_id in items.iter() {
            let menu: MenuItem = env.storage()
                .instance()
                .get(&item_id)
                .expect("Menu not found");

            total += menu.price;
        }

        let order = Order {
            id,
            items,
            total,
            customer,
            status: symbol_short!("NEW"),
        };

        env.storage().instance().set(&id, &order);
    }

    // READ ORDER
    pub fn get_order(env: Env, id: u32) -> Order {
        env.storage()
            .instance()
            .get(&id)
            .expect("Order not found")
    }

    // UPDATE STATUS
    pub fn update_status(
        env: Env,
        user: Address,
        id: u32,
        status: Symbol
    ) {
        user.require_auth();
        Self::check_admin(&env, &user);

        let mut order: Order = env.storage().instance().get(&id).unwrap();
        order.status = status;

        env.storage().instance().set(&id, &order);
    }

    // DELETE ORDER
    pub fn delete_order(env: Env, user: Address, id: u32) {
        user.require_auth();
        Self::check_admin(&env, &user);

        env.storage().instance().remove(&id);
    }
}