// contains the tools to solve production chains to given parts

use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::{objects, recipebook};

struct ProductionNode {
    amount:f32,
    building:objects::Building,
    sources: Option<Vec<ProductionNode>>
}
