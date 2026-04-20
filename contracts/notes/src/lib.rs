#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec, Address
};

// =====================
// STRUCT DATA
// =====================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: u64,
    pub stock: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Order {
    pub id: u64,
    pub buyer: Address,
    pub product_id: u64,
    pub quantity: u64,
    pub total_price: u64,
}

// =====================
// STORAGE KEY
// =====================

const PRODUCTS: Symbol = symbol_short!("PRODUCTS");
const ORDERS: Symbol = symbol_short!("ORDERS");

// =====================
// CONTRACT
// =====================

#[contract]
pub struct EcommerceContract;

#[contractimpl]
impl EcommerceContract {

    // =====================
    // PRODUCT
    // =====================

    pub fn add_product(env: Env, name: String, price: u64, stock: u64) -> String {
        let mut products: Vec<Product> =
            env.storage().instance().get(&PRODUCTS).unwrap_or(Vec::new(&env));

        let product = Product {
            id: env.prng().gen::<u64>(),
            name,
            price,
            stock,
        };

        products.push_back(product);

        env.storage().instance().set(&PRODUCTS, &products);

        String::from_str(&env, "Produk berhasil ditambahkan")
    }

    pub fn get_products(env: Env) -> Vec<Product> {
        env.storage().instance().get(&PRODUCTS).unwrap_or(Vec::new(&env))
    }

    // =====================
    // ORDER / BELI
    // =====================

    pub fn buy_product(
        env: Env,
        buyer: Address,
        product_id: u64,
        quantity: u64,
    ) -> String {

        buyer.require_auth();

        let mut products: Vec<Product> =
            env.storage().instance().get(&PRODUCTS).unwrap_or(Vec::new(&env));

        let mut orders: Vec<Order> =
            env.storage().instance().get(&ORDERS).unwrap_or(Vec::new(&env));

        for i in 0..products.len() {
            let mut product = products.get(i).unwrap();

            if product.id == product_id {

                if product.stock < quantity {
                    return String::from_str(&env, "Stock tidak cukup");
                }

                product.stock -= quantity;
                products.set(i, product.clone());

                let total_price = product.price * quantity;

                let order = Order {
                    id: env.prng().gen::<u64>(),
                    buyer,
                    product_id,
                    quantity,
                    total_price,
                };

                orders.push_back(order);

                env.storage().instance().set(&PRODUCTS, &products);
                env.storage().instance().set(&ORDERS, &orders);

                return String::from_str(&env, "Pembelian berhasil");
            }
        }

        String::from_str(&env, "Produk tidak ditemukan")
    }

    pub fn get_orders(env: Env) -> Vec<Order> {
        env.storage().instance().get(&ORDERS).unwrap_or(Vec::new(&env))
    }
}