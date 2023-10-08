use serde::{Deserialize, Serialize};

use crate::service::link::Link;

pub mod link;

#[derive(Deserialize, Serialize, Clone)]
pub enum Service {
    Link(Link),
}
