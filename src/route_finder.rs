// contains the tools to solve production chains to given parts

use std::collections::HashMap;
use crate::{objects::{Part,
                      Building,
                      Amount},
            recipebook};

type Multiverse = Vec<ProductionNode>;
type BookOfPaths = HashMap<Part, Multiverse>;

#[derive(Debug)]
pub(crate) struct ProductionNode {
    pub(crate)amount:f32,
    pub(crate)building:Building,
    pub(crate)sources: BookOfPaths
}

enum ReportMode{
    Consolidate,
    Disparate
}

enum OptimizationMode{
    MinimizePower,
    MinimizeResources,
    MinimizeBuildings,
    ExploitNodes(NodeSet),
    WhatWeHave(Vec<Amount<Part>>)
}

#[derive(Default)]
struct NodesAvailable{
    poor:usize,
    mid: usize,
    perf:usize
}

#[derive(Default)]
struct NodeSet{
    iron_nodes: NodesAvailable,
    copper_nodes: NodesAvailable,
    limestone_nodes: NodesAvailable,
    coal_nodes: NodesAvailable,
    sulfur_nodes: NodesAvailable,
    quartz_nodes: NodesAvailable,
    caterium_nodes: NodesAvailable,
    oil_nodes: NodesAvailable,
    water_nodes: NodesAvailable,
    nitrogen_nodes: NodesAvailable,
    bauxite_nodes: NodesAvailable,
    uranium_nodes: NodesAvailable
}


pub(crate) fn generate_possibilities(ingredient: Part, amount: usize) -> Multiverse {
    println!("Called for {} of {:?}",amount, ingredient);
    recipebook::RECIPES.iter()
        .filter(|recipe| {
            recipe.building
                .get_output().iter()
                .any(|&output| {
                    output.0 == ingredient
                })})
        .map(|recipe| {
            ProductionNode{
                amount: ((amount as f32) / (recipe.building
                    .get_output().iter()
                    .find(|(part, ..)| *part == ingredient).unwrap().1) as f32),
                building: recipe.building,
                sources: recipe.building
                    .get_input().iter()
                    .map(|&(part, volume)| (part, generate_possibilities(part, volume)))
                    .collect::<BookOfPaths>()
            }
        })
        .collect::<Vec<ProductionNode>>()
}

pub(crate) fn walk_one_path(options: Multiverse){
    todo!()
}

