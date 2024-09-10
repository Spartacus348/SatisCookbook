// contains the tools to solve production chains to given parts

use std::collections::HashMap;
use crate::{objects::{Part, Building, Amount},
            recipebook
};
use crate::objects::Process;

type Multiverse<'a> = Vec<ProductionNode<'a>>;
type BookOfPaths<'a> = HashMap<Part, Multiverse<'a>>;

#[derive(Debug)]
pub(crate) struct ProductionNode<'a> {
    pub(crate) source_recipe: &'a Process<'a>,
    pub(crate) amount:f32,
    pub(crate) sources: BookOfPaths<'a>
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
pub(crate) struct NodeSet{
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

pub(crate) fn generate_possibilities<'a>(ingredient: &Part, rate_per_min: f32, upline: Vec<Part>) -> Multiverse<'a> {
    // generates a tree of all possible production lines
    recipebook::RECIPES.iter()
        .filter_map(|recipe| {
            // escape if invalid recipe
            if upline.iter().any(|output| recipe.building.get_input().iter().any(|(part, _)| part == output)) {
                return None}
            let rate_produced = recipe.get_output_rate_per_min(ingredient)?;

            let mut new_upline = upline.clone();
            new_upline.push(ingredient.clone());

            let scale_factor = rate_per_min/rate_produced;
            let sources = recipe.building
                .get_input().iter()
                .map(|&(part, _)| {
                    let part_per_min: f32 = scale_factor * recipe.get_input_rate_per_min(&part).unwrap();
                    let possibilities = generate_possibilities(&part, part_per_min, new_upline.clone());
                    (part, possibilities)
                })
                .collect::<BookOfPaths>();
            // if any part needs a building and there are no possibilities, return None
            if sources.iter().any(|(part, poss)| poss.is_empty() && part.needs_building()) {
                return None
            }
            Some(ProductionNode{
                source_recipe: recipe,
                amount: scale_factor,
                sources
            })
        })
        .collect::<Vec<ProductionNode>>()
}

#[derive(Debug)]
pub(crate) struct OnePath<'a>{
    pub(crate) source_recipe: &'a Process<'a>,
    pub(crate) amount: f32,
    pub(crate) inputs: HashMap<Part, OnePath<'a>>
}

impl OnePath<'_>{
    pub(crate) fn get_power(self: &Self) -> f32 {
        let power_scale: f32 = (self.amount/self.amount.ceil()).powf(1.321928);
        self.source_recipe.building.get_power() as f32 * self.amount * power_scale + self.inputs.iter()
            .map(|(_, path)| path.get_power())
            .sum::<f32>()
    }

    pub(crate) fn get_raw_resources(self: &Self) -> HashMap<Part, f32> {
        let mut raw_resources: HashMap<Part, f32> = self.source_recipe.building
            .get_input().iter()
            .filter_map(|&(part, size)| match part {
                Part::Mine(_) | Part::Pump(_) => Some((part, size as f32*self.amount)),
                _ => None
            }).collect();
        self.inputs.iter().for_each(|(_, path)| {
            update_counter(&mut raw_resources, path.get_raw_resources())});
        raw_resources
    }

    pub(crate) fn count_buildings(self: &Self) -> usize {
        self.amount.ceil() as usize + self.inputs.iter()
            .map(|(_,path)| path.count_buildings())
            .sum::<usize>()
    }
}


fn update_counter(map: &mut HashMap<Part, f32>, addition: HashMap<Part, f32>){
    // merges addition into map, but if there's a key that's already in map, the values get added
    for (key, &val) in addition.iter(){
        let temp =  map.get(key).unwrap_or(&0f32);
        map.insert(*key, val + *temp);
    }
}


pub(crate) fn walk_one_path<'a>(options: &'a Multiverse<'a>, algo: OptimizationMode) -> Option<OnePath<'a>>{
    // finds the optimal path from a tree of paths, depending on which optimization is entered
    match algo{
        OptimizationMode::MinimizeResources => least_resources(options),
        OptimizationMode::MinimizePower => least_power(options),
        OptimizationMode::MinimizeBuildings => fewest_buildings(options),
        OptimizationMode::ExploitNodes(nodes)    => best_from_nodes(options, nodes),
        OptimizationMode::WhatWeHave(starting_parts) => from_parts(options, starting_parts)
    }
}

fn from_parts<'a>(p0: &'a Multiverse<'a>, p1: Vec<Amount<Part>>)  -> Option<OnePath<'a>>{
    // find the recipe that produces the most of the target using the parts given
    // stretch goal: allow for non-given parts, but minimize the amount used
    todo!()
}

fn best_from_nodes<'a>(p0: &'a Multiverse<'a>, p1: NodeSet)  -> Option<OnePath<'a>>{
    // given a set of nodes, return the overall path that produces the most parts
    // recursion doesn't work here since there isn't a way to know which nodes get
    // assigned were
    // as a starting move, find any resources we have 0 of and remove the possibilities that
    // utilize them
    todo!()
}

fn fewest_buildings<'a>(p0: &'a Multiverse<'a>)  -> Option<OnePath<'a>>{
    // for each part choose the possibility that contains the fewest number of buildings
    // recursively descend the tree, returning the most compact choice for each input
    p0.iter().map(|node| {
        OnePath{
            source_recipe: node.source_recipe,
            amount: node.amount,
            inputs: node.sources.iter()
                .filter_map(|(&part, multiverse)|
                    if let Some(path) = fewest_buildings(multiverse){
                        Some((part, path))
                    } else {None}
                )
                .collect()
        }
    }).reduce(|p0,p1| if p1.count_buildings() < p0.count_buildings() {p1} else {p0})
}

fn least_power<'a>(p0: &'a Multiverse<'a>)  -> Option<OnePath<'a>>{
    // for each part choose the possibility that consumes the least total power
    // recursively descend the tree, returning the cheapest choice for each input
    p0.iter().map(|node| {
        OnePath{
            source_recipe: node.source_recipe,
            amount: node.amount,
            inputs: node.sources.iter()
                .filter_map(|(&part, multiverse)|
                    if let Some(path) = least_power(multiverse){
                        Some((part, path))
                    } else {None}
                )
                .collect()
        }
    }).reduce(|p0,p1| if p1.get_power() < p0.get_power() {p1} else {p0})
}

fn least_resources<'a>(p0: &'a Multiverse<'a>) -> Option<OnePath<'a>> {
    // for each part, choose the possibility that consumes the least raw resources
    //recursively descend the tree, returning the cheapest choice for each input
    p0.iter().map(|node| {
        OnePath{
            source_recipe: node.source_recipe,
            amount: node.amount,
            inputs: node.sources.iter()
                .filter_map(|(&part, multiverse)|
                    if let Some(path) = least_resources(multiverse){
                        Some((part, path))
                    } else {None}
                ).collect()
        }
    }).reduce(|p0, p1|
        if p1.get_raw_resources().values().sum::<f32>() < p0.get_raw_resources().values().sum::<f32>() {
            p1
        } else {
            p0
        })
}

