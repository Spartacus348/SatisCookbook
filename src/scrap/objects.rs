use crate::tiers::*;

pub(crate) struct Process<'a>{
    pub(crate)name: &'a str,
    pub(crate)time: usize,
    pub(crate)building: Building,
    pub(crate)tier: Tier,
}

pub(crate) enum Building{
    Smelter       {input:(Conveyable,),   output:(Conveyable,)},
    Foundry       {input:(Conveyable,Option<Conveyable>),    output:(Conveyable,)},
    Constructor   {input:(Conveyable,),   output:(Conveyable,)},
    Assembler     {input:(Conveyable,Option<Conveyable>),    output:(Conveyable,)},
    Manufacturer  {input:(Conveyable,Option<Conveyable>,Option<Conveyable>,Option<Conveyable>),  output:(Conveyable,)},
    Refinery      {input:(Option<Conveyable>,Option<Pipeable>),    output:(Option<Conveyable>, Option<Pipeable>)},
    Blender       {input:(Option<Conveyable>, Option<Conveyable>, Option<Pipeable>, Option<Pipeable>),output:(Option<Conveyable>, Option<Pipeable>)},
    Packager      {input:(Option<Conveyable>,Option<Pipeable>),    output:(Option<Conveyable>, Option<Pipeable>)},
    BioPlant      {input:(Part,), output:()},
    CoalPlant     {input:(Conveyable, Pipeable), output:()},
    OilPlant      {input:(Pipeable,), output:()},
    NuclearPlant  {input:(Conveyable, Pipeable), output:(Conveyable,)},
    Miner1        {input:(Mineable,),  output:(Conveyable,),},
    Miner2        {input:(Mineable,),  output:(Conveyable,),},
    Miner3        {input:(Mineable,),  output:(Conveyable,),},
    WaterExtractor{input:(Pumpable,),  output:(Pipeable,),},
    OilExtractor  {input:(Pumpable,),  output:(Pipeable,),},
}

impl Building {
    pub(crate) fn get_power(self: &Self) -> isize {
        match self {
            Building::Smelter { .. } => 4,
            Building::Foundry { .. } => 16,
            Building::Constructor { .. } => 4,
            Building::Assembler { .. } => 15,
            Building::Manufacturer { .. } => 55,
            Building::Refinery { .. } => 30,
            Building::Blender { .. } => 75,
            Building::Packager { .. } => 10,
            Building::BioPlant { .. } => -25,
            Building::CoalPlant { .. } => -75,
            Building::OilPlant { .. } => -150,
            Building::NuclearPlant { .. } => -2500,
            Building::Miner1 { .. } => 5,
            Building::Miner2 { .. } => 12,
            Building::Miner3 { .. } => 30,
            Building::WaterExtractor {..} => 20,
            Building::OilExtractor {..} => 40
        }
    }

    pub(crate) fn get_input(self: &Self) -> Vec<Part> {
        match self {
            Building{input, output:_ } => Vec::from(input)
        }
    }

    pub(crate) fn get_output(self: &Self) -> Vec<Part>{
        match self {
            Building{input:_, output} => Vec::from(output)
        }
    }
}

enum NodeQuality{Low,Med,High}

#[derive(PartialEq)]
pub(crate) enum Part{
    Conveyor(Conveyable),
    Pipe(Pipeable),
    Mine(Mineable),
    Pump(Pumpable)
}

#[derive(PartialEq)]
pub(crate) enum Conveyable{
    FeOre {amount:usize},     CuOre{amount:usize},  Limestone{amount:usize},
    Coal{amount:usize},       Sulfur{amount:usize}, RawQuartz{amount:usize},
    CateriumOre{amount:usize},UOre{amount:usize},   Bauxite{amount:usize},
    SAM{amount:usize},//raw ores
    PkgdWater{amount:usize},   PkgdOil{amount:usize},  PkgdN{amount:usize},
    PkgdHOil{amount:usize},    PkgdFuel{amount:usize}, PkgdTurbofuel{amount:usize},
    PkgdLBiofuel{amount:usize},PkgdNAcid{amount:usize},PkgdAlSol{amount:usize},
    PkgdSAcid{amount:usize},//packaged pipes
    Leaves{amount:usize},      Wood{amount:usize},     Mycelia{amount:usize},
    HatcherProtein{amount:usize},HogProtein{amount:usize},SpitterProtein{amount:usize},
    StingerProtein{amount:usize},//raw biomass
    AlienProtein{amount:usize},AlienDNA{amount:usize}, Biomass{amount:usize},
    SolidBiofuel{amount:usize},Fabric{amount:usize},   CompactedCoal{amount:usize},//processed biomatter
    FlowerPetals{amount:usize},ColorCartridge{amount:usize},//color stuff
    FicsmasGift{amount:usize}, ActualSnow{amount:usize},CandyCane{amount:usize},
    FicsmasBow{amount:usize},  FicsmasTree{amount:usize},Snowball{amount:usize},//constructor fiscmas
    FancyFireworks{amount:usize},SparklyFireworks{amount:usize},SweetFireworks{amount:usize},//ficsmas fireworks
    FeOrnament {amount:usize},CuOrnament {amount:usize},RedOrnament{amount:usize},
    BlueOrnament{amount:usize},FicsmasBranch{amount:usize},OrnamentBundle{amount:usize},
    FicsmasDecoration{amount:usize},FicsmasStar{amount:usize},//assembler fixmas
    FeIngot {amount:usize},   FeRod {amount:usize},    FePlate {amount:usize},
    Screws{amount:usize},//constructor iron
    CuIngot {amount:usize}, CuWire {amount:usize},  Cable{amount:usize},
    CuSheet {amount:usize}, CuPowder {amount:usize},//constructor copper
    Concrete{amount:usize},//constructor limestone
    CrushedQuartz{amount:usize},Silica{amount:usize},    CrystalOscillator{amount:usize},//constructor quartz
    CateriumIngot{amount:usize},Quickwire{amount:usize},//constructor caterium
    Plastic{amount:usize},    Rubber{amount:usize},      PolymerResin{amount:usize},
    EmptyCanister{amount:usize},PetroleumCoke{amount:usize},//constructor plastic
    PowerSlugBlue{amount:usize},PowerSlugYellow{amount:usize},PowerSlugPurple{amount:usize},
    PowerShard{amount:usize},//power slugs
    SteelIngot{amount:usize}, SteelBeam{amount:usize},   SteelPipe{amount:usize},
    IndustrialBeam{amount:usize},//constructor steel
    ReinforcedIronPlate{amount:usize},ModularFrame{amount:usize},HeavyModularFrame{amount:usize},
    FusedModularFrame{amount:usize},//assembler iron
    Rotor{amount:usize},      Stator{amount:usize},      Motor{amount:usize},//assembler motors
    CircuitBoard{amount:usize},HighSpeedConnector{amount:usize},Computer{amount:usize},
    AILimiter{amount:usize},  RadioControlUnit{amount:usize},SuperComputer{amount:usize},//assembler computer
    AlIngot{amount:usize},    AlcladSheet{amount:usize},AlCasing{amount:usize},    Heatsink{amount:usize},
    Battery{amount:usize},    CoolingSystem{amount:usize},AlScrap{amount:usize},
    EmptyFluidTank {amount:usize},//assembler aluminum
    BaseRebar{amount:usize},  ShatterRebar{amount:usize}, PulseRebar{amount:usize},
    ExplosiveRebar{amount:usize},//rebar ammo
    BlackPowder{amount:usize},SmokelessPowder{amount:usize},RifleAmmo{amount:usize},
    HomingRifleAmmo{amount:usize},TurboRifleAmmo{amount:usize},//assembler rifle
    Nobelisk{amount:usize},   GasNobelisk{amount:usize},ClusterNobelisk{amount:usize},
    PulseNobelisk{amount:usize},NuclearNobelisk{amount:usize},//assembler nobelisk
    SmartPlating{amount:usize},VersatileFramework{amount:usize},AutomatedWiring{amount:usize},
    ModularEngine{amount:usize},AdaptiveControlUnit{amount:usize},AssemblyDirectorSystem{amount:usize},//assembler elevator
    EMControlRod{amount:usize},PressureConversionCube{amount:usize},//assembler pre-nuclear
    EncasedUCell{amount:usize},URod{amount:usize},UWaste{amount:usize},
    NonFissileU{amount:usize},//uranium
    PuPellet{amount:usize},    PuRod{amount:usize},                 EncasedPuCell{amount:usize},
    NuclearPasta{amount:usize},PWaste{amount:usize},//assembler nuclear
    TurboMotor{amount:usize},  MagneticFieldGenerator{amount:usize},ThermalPropulsionRocket{amount:usize},
    Beacon{amount:usize},      PortableMiner{amount:usize},         GasFilter{amount:usize},
    IodineFilter{amount:usize},//holdable
}
#[derive(PartialEq)]
pub(crate) enum Pipeable{
    Water{amount:usize}, CrudeOil{amount:usize}, NGas{amount:usize},      HeavyOil{amount:usize},
    Fuel{amount:usize},  Turbofuel{amount:usize}, LBioFuel{amount:usize},
    NAcid{amount:usize}, AlSol{amount:usize},     SAcid{amount:usize}
}
#[derive(PartialEq)]
pub(crate) enum Mineable{
    FeNode {amount:usize},     CuNode {amount:usize},   LimestoneNode{amount:usize},//nodes
    CoalNode{amount:usize},    SulfurNode{amount:usize},QuartzNode{amount:usize},
    CateriumNode{amount:usize},UNode{amount:usize},     BauxiteNode{amount:usize},
    SAMNode{amount:usize},
}
#[derive(PartialEq)]
pub(crate) enum Pumpable{
    WaterSource{amount:usize}, OilSource{amount:usize},   NNode{amount:usize},
}
