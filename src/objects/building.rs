use crate::objects::amount::Amount;
use crate::objects::part::{Conveyable, Mineable, Part, Pipeable, Pumpable};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
struct RangeCap<const MIN: usize, const EXTRA: usize, T>
where
    T: Copy,
{
    guaranteed_data: [T; MIN],
    possible_data: [Option<T>; EXTRA],
}

impl<const MIN: usize, const EXTRA: usize, T> IntoIterator for RangeCap<MIN, EXTRA, T>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut guarantees: Vec<T> = Vec::from(self.guaranteed_data);

        let options: Vec<T> = Vec::<T>::from(
            self.possible_data
                .iter()
                .filter_map(|&x| x)
                .collect::<Vec<T>>(),
        );
        guarantees.extend(options.into_iter());
        guarantees.into_iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Building<const MIN_ITEM_IN: usize, const EXTRA_ITEM_IN: usize,
    const MIN_FLUID_IN: usize, const EXTRA_FLUID_IN: usize, const MIN_ITEM_OUT: usize,
    const EXTRA_ITEM_OUT: usize, const MIN_FLUID_OUT: usize, const EXTRA_FLUID_OUT: usize,
    ConvInType, FluidInType,
> where
    ConvInType: Copy,
    FluidInType: Copy,
{
    input_item: RangeCap<MIN_ITEM_IN, EXTRA_ITEM_IN, Amount<ConvInType>>,
    input_fluid: RangeCap<MIN_FLUID_IN, EXTRA_FLUID_IN, Amount<FluidInType>>,
    output_item: RangeCap<MIN_ITEM_OUT, EXTRA_ITEM_OUT, Amount<Conveyable>>,
    output_fluid: RangeCap<MIN_FLUID_OUT, EXTRA_FLUID_OUT, Amount<Pipeable>>,
}

impl<const MIN_ITEM_IN: usize, const EXTRA_ITEM_IN: usize, const MIN_FLUID_IN: usize, const EXTRA_FLUID_IN: usize,
    const MIN_ITEM_OUT: usize, const EXTRA_ITEM_OUT: usize, const MIN_FLUID_OUT: usize, const EXTRA_FLUID_OUT: usize,
    ConvInType, FluidInType, >
    Building<
        MIN_ITEM_IN, EXTRA_ITEM_IN, MIN_FLUID_IN, EXTRA_FLUID_IN,
        MIN_ITEM_OUT, EXTRA_ITEM_OUT, MIN_FLUID_OUT, EXTRA_FLUID_OUT, ConvInType, FluidInType, >
where
    ConvInType: Copy,
    FluidInType: Copy,
{

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Buildings {
    Smelter(Building<1, 0, 0, 0, 1, 0, 0, 0, Conveyable, NoPart>),
    Foundry(Building<2, 0, 0, 0, 1, 0, 0, 0, Conveyable, NoPart>),
    Constructor(Building<1, 0, 0, 0, 1, 0, 0, 0, Conveyable, NoPart>),
    Assembler(Building<2, 0, 0, 0, 1, 0, 0, 0, Conveyable, NoPart>),
    Manufacturer(Building<3, 1, 0, 0, 1, 0, 0, 0, Conveyable, NoPart>),
    Refinery(Building<0, 1, 0, 1, 0, 1, 0, 1, Conveyable, Pipeable>),
    Blender(Building<0, 2, 1, 1, 0, 1, 0, 1, Conveyable, Pipeable>),
    Packager(Building<0, 1, 0, 1, 0, 1, 0, 1, Conveyable, Pipeable>),
    BioPlant(Building<1, 0, 0, 0, 0, 0, 0, 0, Conveyable, Pipeable>),
    CoalPlant(Building<1, 0, 1, 0, 0, 0, 0, 0, Conveyable, Pipeable>),
    OilPlant(Building<0, 0, 1, 0, 0, 0, 0, 0, NoPart, Pipeable>),
    NuclearPlant(Building<1, 0, 1, 0, 1, 0, 0, 0, Conveyable, Pipeable>),
    Miner1(Building<1, 0, 0, 0, 1, 0, 0, 0, Mineable, Pipeable>),
    Miner2(Building<1, 0, 0, 0, 1, 0, 0, 0, Mineable, Pipeable>),
    Miner3(Building<1, 0, 0, 0, 1, 0, 0, 0, Mineable, Pipeable>),
    WaterExtractor(Building<0, 0, 1, 0, 0, 0, 1, 0, NoPart, Pumpable>),
    OilExtractor(Building<0, 0, 1, 0, 0, 0, 1, 0, NoPart, Pumpable>),
}

type Smelter = Building<1, 0, 0, 0, 1, 0, 0, 0, Conveyable, NoPart>;

impl Buildings {
    fn get_name(self: &Self) -> String {
        String::from(match self {
            Buildings::Smelter { .. } => "smelter",
            Buildings::Foundry { .. } => "foundry",
            Buildings::Constructor { .. } => "constructor",
            Buildings::Assembler { .. } => "assembler",
            Buildings::Manufacturer { .. } => "manufacturer",
            Buildings::Refinery { .. } => "refinery",
            Buildings::Blender { .. } => "blender",
            Buildings::Packager { .. } => "packager",
            Buildings::BioPlant { .. } => "bio_plant",
            Buildings::CoalPlant { .. } => "coal_plan",
            Buildings::OilPlant { .. } => "oil_plan",
            Buildings::NuclearPlant { .. } => "nul_plan",
            Buildings::Miner1 { .. } => "miner",
            Buildings::Miner2 { .. } => "miner",
            Buildings::Miner3 { .. } => "miner",
            Buildings::WaterExtractor { .. } => "water_extractor",
            Buildings::OilExtractor { .. } => "oil_extractor",
        })
    }

    pub(crate) fn get_input(self: &Self) -> Vec<(Part, usize)>{
        if let x: Building = self {
                let mut inputs: Vec<(Part, usize)> = x.input_item
                    .clone()
                    .into_iter()
                    .map(|item| (Part::Conveyor(item.kind), item.rate_per_period))
                    .collect();
                inputs.append(
                    x.input_fluid
                        .clone()
                        .into_iter()
                        .map(|fluid| (Part::Pipe(fluid.kind), fluid.rate_per_period))
                        .collect(),
                );
                inputs
        }
    }

    pub(crate) fn get_output(self: &Self) -> Vec<(Part, usize)> {
        let mut outputs: Vec<(Part, usize)> = self
            .output_item
            .clone()
            .into_iter()
            .map(|item| (Part::Conveyor(item.kind), item.rate_per_period))
            .collect();
        outputs.append(
            self.output_fluid
                .clone()
                .into_iter()
                .map(|fluid| (Part::Pipe(fluid.kind), fluid.rate_per_period))
                .collect(),
        );
        outputs
    }
}

impl Display for Buildings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            building => building,
        };
        write!(f, "{}({})", self.get_name(), x)
    }
}

impl<
        const MIN_ITEM_IN: usize,
        const EXTRA_ITEM_IN: usize,
        const MIN_FLUID_IN: usize,
        const EXTRA_FLUID_IN: usize,
        const MIN_ITEM_OUT: usize,
        const EXTRA_ITEM_OUT: usize,
        const MIN_FLUID_OUT: usize,
        const EXTRA_FLUID_OUT: usize,
        ConvInType,
        FluidInType,
    > Display
    for Building<
        MIN_ITEM_IN,
        EXTRA_ITEM_IN,
        MIN_FLUID_IN,
        EXTRA_FLUID_IN,
        MIN_ITEM_OUT,
        EXTRA_ITEM_OUT,
        MIN_FLUID_OUT,
        EXTRA_FLUID_OUT,
        ConvInType,
        FluidInType,
    >
where
    ConvInType: Copy,
    FluidInType: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pt1 = self
            .get_input()
            .iter()
            .map(|a| a.0.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let pt2 = self
            .get_output()
            .iter()
            .map(|a| a.0.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}->{}", pt1, pt2)
    }
}
