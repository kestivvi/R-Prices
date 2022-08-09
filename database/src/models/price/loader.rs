use async_trait::async_trait;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use diesel::dsl::any;
use diesel::r2d2::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::collections::HashMap;

use crate::db::PostgresPool;
use crate::diesel_schema::prices;
use crate::models::price::Price;

pub type PriceLoader = Loader<i32, Vec<Price>, PriceBatcher>;

pub fn get_price_loader(pool: PostgresPool) -> PriceLoader {
    Loader::new(PriceBatcher { pool })
}

pub struct PriceBatcher {
    pool: PostgresPool,
}

impl PriceBatcher {
    pub async fn get_prices_by_offer_ids(
        &self,
        hashmap: &mut HashMap<i32, Vec<Price>>,
        ids: &[i32],
    ) -> Result<(), Error> {
        let conn = self.pool.get().unwrap();

        ids.iter().for_each(|&id| {
            hashmap.entry(id).or_insert_with(|| Vec::<Price>::new());
        });

        prices::table
            .filter(prices::columns::offer_id.eq(any(&ids)))
            .get_results::<Price>(&conn)
            .unwrap()
            .iter()
            .fold(hashmap, |map, price| {
                map.entry(price.offer_id)
                    .and_modify(|v| v.push(price.clone()));
                map
            });

        Ok(())
    }
}

#[async_trait]
impl BatchFn<i32, Vec<Price>> for PriceBatcher {
    // TODO: There should be werid Result for errors
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Vec<Price>> {
        let mut prices_map: HashMap<i32, Vec<Price>> = HashMap::new();
        self.get_prices_by_offer_ids(&mut prices_map, keys)
            .await
            .unwrap();
        prices_map
    }
}
