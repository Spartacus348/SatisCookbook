// contains the tools to solve production chains to given parts

use std::collections::HashMap;
use crate::{objects::{Part, Building, Amount},
            recipebook
};

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

pub(crate) enum OptimizationMode{
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
    recipebook::RECIPES.iter()
        .filter(|recipe| {
            recipe.building
                .get_output().iter()
                .any(|&output| {
                    output.0 == ingredient
                })})
        .map(|recipe| {
            ProductionNode{
                amount: (amount as f32) / recipe.building
                    .get_output().iter()
                    .find(|(part, ..)| *part == ingredient).unwrap().1 as f32,
                building: recipe.building,
                sources: recipe.building
                    .get_input().iter()
                    .map(|&(part, volume)| (part, generate_possibilities(part, volume)))
                    .collect::<BookOfPaths>()
            }
        })
        .collect::<Vec<ProductionNode>>()
}

#[derive(Debug)]
pub(crate) struct OnePath{
    pub(crate) amount: f32,
    pub(crate) building: Building,
    pub(crate) inputs: HashMap<Part, OnePath>
}

impl OnePath{
    pub(crate) fn get_power(self: &Self) -> f32 {
        self.amount * (self.building.get_power() as f32 + self.inputs.iter()
            .map(|(_, path)| path.get_power())
            .sum::<f32>())
    }

    pub(crate) fn get_raw_resources(self: &Self) -> HashMap<Part, f32> {
        let mut ret: HashMap<Part, f32> = self.building.get_input().iter().filter_map(|&(part, size)| match part {
            Part::Mine(_) | Part::Pump(_) => Some((part, size as f32*self.amount)),
            _ => None
        }).collect();
        self.inputs.iter().for_each(|(part, path)| {
            let mut cost = path.get_raw_resources();
            scale_map(&mut cost, self.amount);
            update_counter(&mut ret, cost)});
        ret
    }
}

fn scale_map(map: &mut HashMap<Part, f32>, scale: f32) {
    // scales the values in map by the scale
    map.iter_mut().for_each(|(_, val)| *val *= scale)
}

fn update_counter(map: &mut HashMap<Part, f32>, addition: HashMap<Part, f32>){
    // merges addition into map, but if there's a key that's already in map, the values get added
    for (key, &val) in addition.iter(){
        let temp =  map.get(key).unwrap_or(&0f32);
        map.insert(*key, val + *temp);
    }
}


pub(crate) fn walk_one_path(options: Multiverse, algo: OptimizationMode) -> Option<OnePath>{
    // finds the optimal path from a tree of paths, depending on which optimization is entered
    match algo{
        OptimizationMode::MinimizeResources => least_resources(&options),
        OptimizationMode::MinimizePower => least_power(&options),
        OptimizationMode::MinimizeBuildings => fewest_buildings(&options),
        OptimizationMode::ExploitNodes(nodes)    => best_from_nodes(&options, nodes),
        OptimizationMode::WhatWeHave(starting_parts) => from_parts(&options, starting_parts)
    }
}

fn from_parts(p0: &Multiverse, p1: Vec<Amount<Part>>)  -> Option<OnePath>{
    // find the recipe that produces the most of the target using the parts given
    // stretch goal: allow for non-given parts, but minimize the amount used
    todo!()
}

fn best_from_nodes(p0: &Multiverse, p1: NodeSet)  -> Option<OnePath>{
    // given a set of nodes, return the overall path that produces the most parts
    // recursion doesn't work here since there isn't a way to know which nodes get
    // assigned were
    // as a starting move, find any resources we have 0 of and remove the possibilities that
    // utilize them
    todo!()
}

fn fewest_buildings(p0: &Multiverse)  -> Option<OnePath>{
    // for each part choose the possibility that contains the fewest number of buildings
    // recursively descend the tree, returning the most compact choice for each input
    todo!()
}

fn least_power(p0: &Multiverse)  -> Option<OnePath>{
    // for each part choose the possibility that consumes the least total power
    // recursively descend the tree, returning the cheapest choice for each input
    p0.iter().map(|node| {
        let path = OnePath{
            amount: node.amount,
            building: node.building,
            inputs: node.sources.iter()
                .filter_map(|(&part, multiverse)|
                    if let Some(path) = least_power(multiverse){
                        Some((part, path))
                    } else {None}
                )
                .collect()
        };
        path
    }).reduce(|p0,p1| if p1.get_power() < p0.get_power() {p1} else {p0})
}

fn least_resources(p0: &Multiverse) -> Option<OnePath> {
    // for each part, choose the possibility that consumes the least raw resources
    //recursively descend the tree, returning the cheapest choice for each input
    p0.iter().map(|node| {
        let path = OnePath{
            amount: node.amount,
            building: node.building,
            inputs: node.sources.iter()
                .filter_map(|(&part, multiverse)|
                    if let Some(path) = least_resources(multiverse){
                        Some((part, path))
                    } else {None}
                ).collect()
        };
        path
    }).reduce(|p0, p1| if p1.get_raw_resources().values().sum::<f32>() < p0.get_raw_resources().values().sum::<f32>() {p1} else {p0})
}

