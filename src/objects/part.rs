use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, Debug, Clone, Copy, PartialEq, Hash)]
pub(crate) enum Part {
    Conveyor(Conveyable),
    Pipe(Pipeable),
    Mine(Mineable),
    Pump(Pumpable),
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Conveyor(x) => x.fmt(f),
            Part::Pipe(x) => x.fmt(f),
            Part::Mine(x) => x.fmt(f),
            Part::Pump(x) => x.fmt(f),
        }
    }
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Conveyable {
    FeOre,
    CuOre,
    Limestone,
    Coal,
    Sulfur,
    RawQuartz,
    CateriumOre,
    UOre,
    Bauxite,
    SAM, //raw ores
    PkgdWater,
    PkgdOil,
    PkgdN,
    PkgdHOil,
    PkgdFuel,
    PkgdTurbofuel,
    PkgdLBiofuel,
    PkgdNAcid,
    PkgdAlSol,
    PkgdSAcid, //packaged pipes
    WoodOrLeaves,
    Mycelia, //raw biomass
    AlienProtein,
    AlienDNA,
    Biomass,
    SolidBiofuel,
    Fabric,
    CompactedCoal, //processed biomatter
    FlowerPetals,
    ColorCartridge, //color stuff
    FicsmasGift,
    ActualSnow,
    CandyCane,
    FicsmasBow,
    FicsmasTree,
    Snowball, //constructor fiscmas
    FancyFireworks,
    SparklyFireworks,
    SweetFireworks, //ficsmas fireworks
    FeOrnament,
    CuOrnament,
    RedOrnament,
    BlueOrnament,
    FicsmasBranch,
    OrnamentBundle,
    FicsmasDecoration,
    FicsmasStar, //assembler fixmas
    FeIngot,
    FeRod,
    FePlate,
    Screws, //constructor iron
    CuIngot,
    CuWire,
    Cable,
    CuSheet,
    CuPowder, //constructor copper
    Concrete, //constructor limestone
    CrushedQuartz,
    Silica,
    CrystalOscillator, //constructor quartz
    CateriumIngot,
    Quickwire, //constructor caterium
    Plastic,
    Rubber,
    PolymerResin,
    EmptyCanister,
    PetroleumCoke, //constructor plastic
    PowerSlugBlue,
    PowerSlugYellow,
    PowerSlugPurple,
    PowerShard, //power slugs
    SteelIngot,
    SteelBeam,
    SteelPipe,
    IndustrialBeam, //constructor steel
    ReinforcedIronPlate,
    ModularFrame,
    HeavyModularFrame,
    FusedModularFrame, //assembler iron
    Rotor,
    Stator,
    Motor, //assembler motors
    CircuitBoard,
    HighSpeedConnector,
    Computer,
    AILimiter,
    RadioControlUnit,
    SuperComputer, //assembler computer
    AlIngot,
    AlcladSheet,
    AlCasing,
    Heatsink,
    Battery,
    CoolingSystem,
    AlScrap,
    EmptyFluidTank, //assembler aluminum
    BaseRebar,
    ShatterRebar,
    PulseRebar,
    ExplosiveRebar, //rebar ammo
    BlackPowder,
    SmokelessPowder,
    RifleAmmo,
    HomingRifleAmmo,
    TurboRifleAmmo, //assembler rifle
    Nobelisk,
    GasNobelisk,
    ClusterNobelisk,
    PulseNobelisk,
    NuclearNobelisk, //assembler nobelisk
    SmartPlating,
    VersatileFramework,
    AutomatedWiring,
    ModularEngine,
    AdaptiveControlUnit,
    AssemblyDirectorSystem, //assembler elevator
    EMControlRod,
    PressureConversionCube, //assembler pre-nuclear
    EncasedUCell,
    URod,
    UWaste,
    NonFissileU, //uranium
    PuPellet,
    PuRod,
    EncasedPuCell,
    NuclearPasta,
    PWaste, //assembler nuclear
    TurboMotor,
    MagneticFieldGenerator,
    ThermalPropulsionRocket,
    Beacon,
    PortableMiner,
    GasFilter,
    IodineFilter, //holdable
}


#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Pipeable {
    Water,
    CrudeOil,
    NGas,
    HeavyOil,
    Fuel,
    Turbofuel,
    LBioFuel,
    NAcid,
    AlSol,
    SAcid,
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Mineable {
    FeNode,
    CuNode,
    LimestoneNode, //nodes
    CoalNode,
    SulfurNode,
    QuartzNode,
    CateriumNode,
    UNode,
    BauxiteNode,
    SAMNode,
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Pumpable {
    WaterSource,
    OilSource,
    NNode,
}
