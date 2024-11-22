use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Color {
    pub id: String,
    pub name: String,
    pub hex_code: String,
}