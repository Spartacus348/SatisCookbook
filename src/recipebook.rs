use std::string::ToString;
use crate::objects::{
    Process,
    Building::*,
    Part::*};

const RECIPES: [Process] = *[
    Process {
        name:"Mine Iron".to_string(),
        time:1,
        building: Miner1{
            input:(IronNode{amount:1}),
            output:(IronOre{amount:1}),
        },
        tier:1
    },
    Process{
        name:"Mine Copper".to_string(),
        time:1,
        building: Miner1 {
            input:(CopperNode {amount:1}),
            output:(CopperOre{amount:1}),
        },
        tier:1
    }
];
