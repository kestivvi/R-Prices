use async_trait::async_trait;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use diesel::dsl::any;
use diesel::r2d2::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::collections::HashMap;

use crate::db::PostgresPool;
use crate::diesel_schema::{offers, products, products_offers_relation};
use crate::models::offer::Offer;

pub type OfferLoader = Loader<i32, Vec<Offer>, OfferBatcher>;

pub fn get_offer_loader(pool: PostgresPool) -> OfferLoader {
    Loader::new(OfferBatcher { pool })
}

pub struct OfferBatcher {
    pool: PostgresPool,
}

impl OfferBatcher {
    pub async fn get_offers_by_product_ids(
        &self,
        hashmap: &mut HashMap<i32, Vec<Offer>>,
        ids: &[i32],
    ) -> Result<(), Error> {
        let conn = self.pool.get().unwrap();

        ids.iter().for_each(|&id| {
            hashmap.entry(id).or_insert_with(Vec::new);
        });

        offers::table
            .inner_join(products_offers_relation::table.inner_join(products::table))
            .filter(products_offers_relation::columns::product_id.eq(any(&ids)))
            .select((
                offers::all_columns,
                products_offers_relation::columns::product_id,
            ))
            .get_results::<(Offer, i32)>(&conn)
            .unwrap()
            .iter()
            .fold(hashmap, |map, (offer, product_id)| {
                map.entry(*product_id).and_modify(|v| v.push(offer.clone()));
                map
            });

        Ok(())
    }
}

#[async_trait]
impl BatchFn<i32, Vec<Offer>> for OfferBatcher {
    // TODO: There should be werid Result for errors
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Vec<Offer>> {
        let mut offers_map: HashMap<i32, Vec<Offer>> = HashMap::new();
        self.get_offers_by_product_ids(&mut offers_map, keys)
            .await
            .unwrap();
        offers_map
    }
}
