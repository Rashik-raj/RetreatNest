use crate::{entities_helper::RetreatReviewModel, map_fields, utils::serializer::deserialize_some};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateRetreatReviewSerializer {
    pub rating: f64,
    pub review: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ReadRetreatReviewSerializer {
    review_id: i64,
    retreat_id: i64,
    user_id: i64,
    rating: f64,
    review: Option<String>
}

impl From<RetreatReviewModel> for ReadRetreatReviewSerializer {
    fn from(value: RetreatReviewModel) -> Self {
        map_fields!(value, ReadRetreatReviewSerializer, {
            review_id,
            retreat_id,
            user_id,
            rating,
            review
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct UpdateRetreatReviewSerializer {
    pub rating: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_some")]
    pub review: Option<Option<String>>,
}
