pub mod accounts;
pub mod schema;
pub mod pool;

use crate::accounts::models::*;
use diesel::prelude::*;

pub fn main() {
    openssl::init();

    use self::schema::accounts::dsl::*;

    let pool = pool::create_pool();
    let connection = &mut pool.get().expect("Failed to connect to database");
    let results = accounts
        .filter(is_active.eq(true))
        .load::<Account>(connection)
        .expect("Error loading accounts");

    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{}", account.id);
    }
}