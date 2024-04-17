use zvariant::Type;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub enum Status {
    Success,
    Failure,
}
