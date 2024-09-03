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
    // generates a tree of all possible production lines
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

struct OnePath{
    pub(crate) amount: f32,
    pub(crate) building: Building,
    pub(crate) inputs: HashMap<Part, ProductionNode>
}

pub(crate) fn walk_one_path(options: Multiverse, algo: OptimizationMode) -> OnePath{
    // finds the optimal path from a tree of paths, depending on which optimization is entered
    match algo{
        OptimizationMode::MinimizeResources => least_resources(options),
        OptimizationMode::MinimizePower => least_power(options),
        OptimizationMode::MinimizeBuildings => fewest_buildings(options),
        OptimizationMode::ExploitNodes(nodes)    => best_from_nodes(options, nodes),
        OptimizationMode::WhatWeHave(starting_parts) => from_parts(options, starting_parts)
    }
}

fn from_parts(p0: Multiverse, p1: Vec<Amount<Part>>)  -> OnePath{
    // find the recipe that produces the most of the target using the parts given
    // stretch goal: allow for non-given parts, but minimize the amount used
    todo!()
}

fn best_from_nodes(p0: Multiverse, p1: NodeSet)  -> OnePath{
    // given a set of nodes, return the overall path that produces the most parts
    // recursion doesn't work here since there isn't a way to know which nodes get
    // assigned were
    // as a starting move, find any resources we have 0 of and remove the possibilities that
    // utilize them
    todo!()
}

fn fewest_buildings(p0: Multiverse)  -> OnePath{
    // for each part choose the possibility that contains the fewest number of buildings
    // recursively descend the tree, returning the most compact choice for each input
    todo!()
}

fn least_power(p0: Multiverse)  -> OnePath{
    // for each part choose the possibility that consumes the least total power
    // recursively descend the tree, returning the cheapest choice for each input
    todo!()
}

fn least_resources(p0: Multiverse)  -> OnePath{
    // for each part, choose the possibility that consumes the least raw resources
    //recursively descend the tree, returning the cheapest choice for each input
    todo!()
}
