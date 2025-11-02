use serde::Serialize;

use crate::{entities_helper::WishlistModel, map_fields};

#[derive(Debug, Clone, Serialize)]
pub struct ReadWishlistSerializer {
    pub wishlist_id: i64,
    pub retreat_id: i64,
    pub user_id: i64
}

impl From<WishlistModel> for ReadWishlistSerializer {
    fn from(value: WishlistModel) -> Self {
        map_fields!(value, ReadWishlistSerializer, {
            wishlist_id,
            user_id,
            retreat_id
        })
    }
}