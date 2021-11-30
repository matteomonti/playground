use std::time::Instant;

use zebra::database::{Database, TableTransaction};

fn main() {
    let database = Database::<u32, u32>::new();
    let mut table = database.empty_table();

    loop {
        let start = Instant::now();

        let mut transaction = TableTransaction::new();

        for _ in 0..50000 {
            let _ = transaction.set(rand::prelude::random(), rand::prelude::random());
        }

        println!("Time to build transaction: {} ms", start.elapsed().as_millis());

        let start = Instant::now();

        table.execute(transaction);

        println!("Time to execute: {} ms", start.elapsed().as_millis());
    }
}