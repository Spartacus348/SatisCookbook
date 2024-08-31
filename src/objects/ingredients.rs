
#[derive(Clone)]
pub enum Part{
    FeOre, FeIngot, FeRod, FePlate, Screw, ReinforcedPlate, Rotor, ModularFrame,
    CuOre, CuIngot, CuWire, Cable, CuSheet,
}

#[derive(Clone)]
pub(crate) struct Conveyable{
    part:Part,
    amt: usize
}
