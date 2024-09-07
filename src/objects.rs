use std::fmt::{Debug, Display, Formatter};
use crate::tiers::*;

#[derive(Clone,Copy,PartialEq,Debug)]
pub(crate) struct Process<'a>{
    pub(crate)name: &'a str,
    pub(crate)time: usize,
    pub(crate)building: Building,
    pub(crate)tier: Tier,
}

impl Process<'_>{
    pub(crate) fn get_input_rate(self: &Self, search_part: Part) -> Option<f32>{
        self.building.get_input().iter()
            .find_map(|&(find_part, amt)|
                if find_part==search_part {Some(amt as f32 / self.time as f32)}
                else {None})
    }

    pub(crate) fn get_output_rate(self: &Self, search_part: Part) -> Option<f32>{
        self.building.get_output().iter()
            .find_map(|&(find_part, amt)|
                if find_part==search_part {Some(amt as f32/self.time as f32)}
                else {None})
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Building{
    Smelter       {input:(Amount<Conveyable>,), output:(Amount<Conveyable>,)},
    Foundry       {input:(Amount<Conveyable>,Amount<Conveyable>),   output:(Amount<Conveyable>,)},
    Constructor   {input:(Amount<Conveyable>,), output:(Amount<Conveyable>,)},
    Assembler     {input:(Amount<Conveyable>,Amount<Conveyable>),   output:(Amount<Conveyable>,)},
    Manufacturer  {input:(Amount<Conveyable>,Amount<Conveyable>,Amount<Conveyable>,Option<Amount<Conveyable>>),             output:(Amount<Conveyable>,)},
    Refinery      {input:(Option<Amount<Conveyable>>,Option<Amount<Pipeable>>), output:(Option<Amount<Conveyable>>,Option<Amount<Pipeable>>)},
    Blender       {input:(Option<Amount<Conveyable>>,Option<Amount<Conveyable>>,Amount<Pipeable>,Option<Amount<Pipeable>>), output:(Option<Amount<Conveyable>>,Option<Amount<Pipeable>>)},
    Packager      {input:(Amount<Conveyable>,Option<Amount<Pipeable>>),         output:(Amount<Conveyable>,Option<Amount<Pipeable>>)},
    BioPlant      {input:(Amount<Part>,),       output:()},
    CoalPlant     {input:(Amount<Conveyable>, Amount<Pipeable>),    output:()},
    OilPlant      {input:(Amount<Pipeable>,),   output:()},
    NuclearPlant  {input:(Amount<Conveyable>, Amount<Pipeable>),    output:(Amount<Conveyable>,)},
    Miner1        {input:(Amount<Mineable>,),   output:(Amount<Conveyable>,),},
    Miner2        {input:(Amount<Mineable>,),   output:(Amount<Conveyable>,),},
    Miner3        {input:(Amount<Mineable>,),   output:(Amount<Conveyable>,),},
    WaterExtractor{input:(Amount<Pumpable>,),   output:(Amount<Pipeable>,),},
    OilExtractor  {input:(Amount<Pumpable>,),   output:(Amount<Pipeable>,),},
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

    pub(crate) fn get_input(self: &Self) -> Vec<(Part,usize)> {
        match self{
            Building::Smelter       {input:(x,                          ), .. } => Vec::from([(Part::Conveyor(x.kind),x.count)]),
            Building::Foundry       {input:(a,      b                   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Conveyor(b.kind),b.count)]),
            Building::Constructor   {input:(a,                          ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Assembler     {input:(a,      b                   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Conveyor(b.kind),b.count)]),
            Building::Manufacturer  {input:(a,      b,      c,Some(d)   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Conveyor(b.kind),b.count),(Part::Conveyor(c.kind),c.count),(Part::Conveyor(d.kind),d.count)]),
            Building::Manufacturer  {input:(a,      b,      c,None      ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Conveyor(b.kind),b.count),(Part::Conveyor(c.kind),c.count)]),
            Building::Refinery      {input:(None,   None                ), .. } => Vec::<(Part,usize)>::new(),
            Building::Refinery      {input:(Some(a),Some(b)             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Pipe(b.kind),b.count)]),
            Building::Refinery      {input:(Some(a),None                ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Refinery      {input:(None,   Some(b)             ), .. } => Vec::from([(Part::Pipe(b.kind),b.count)]),
            Building::Blender       {input:(Some(a),Some(b),c,Some(d)   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Conveyor(b.kind),b.count),(Part::Pipe(c.kind),c.count),(Part::Pipe(d.kind),d.count)]),
            Building::Blender       {input:(Some(a),Some(b),c,None      ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Conveyor(b.kind),b.count),(Part::Pipe(c.kind),c.count)]),
            Building::Blender       {input:(Some(a),None,   c,Some(d)   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Pipe(c.kind),c.count),    (Part::Pipe(d.kind),d.count)]),
            Building::Blender       {input:(Some(a),None,   c,None      ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Pipe(c.kind),c.count)]),
            Building::Blender       {input:(None,   Some(b),c,Some(d)   ), .. } => Vec::from([(Part::Conveyor(b.kind),b.count),(Part::Pipe(c.kind),c.count),    (Part::Pipe(d.kind),d.count)]),
            Building::Blender       {input:(None,   Some(b),c,None      ), .. } => Vec::from([(Part::Conveyor(b.kind),b.count),(Part::Pipe(c.kind),c.count)]),
            Building::Blender       {input:(None,   None,   c,Some(d)   ), .. } => Vec::from([(Part::Pipe(c.kind),c.count),    (Part::Pipe(d.kind),d.count)]),
            Building::Blender       {input:(None,   None,   c,None      ), .. } => Vec::from([(Part::Pipe(c.kind),c.count)]),
            Building::Packager      {input:(a,      None                ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Packager      {input:(a,      Some(b)             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Pipe(b.kind),b.count)]),
            Building::BioPlant      {input:(a,                          ), .. } => Vec::<(Part,usize)>::from([(a.kind,1)]),
            Building::CoalPlant     {input:(a,      b                   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Pipe(b.kind),b.count)]),
            Building::OilPlant      {input:(a,                          ), .. } => Vec::from([(Part::Pipe(a.kind),a.count)]),
            Building::NuclearPlant  {input:(a,      b                   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Pipe(b.kind),b.count)]),
            Building::Miner1        {input:(a,                          ), .. } => Vec::from([(Part::Mine(a.kind),a.count)]),
            Building::Miner2        {input:(a,                          ), .. } => Vec::from([(Part::Mine(a.kind),a.count)]),
            Building::Miner3        {input:(a,                          ), .. } => Vec::from([(Part::Mine(a.kind),a.count)]),
            Building::WaterExtractor{input:(a,                          ), .. } => Vec::from([(Part::Pump(a.kind),a.count)]),
            Building::OilExtractor  {input:(a,                          ), .. } => Vec::from([(Part::Pump(a.kind),a.count)])
        }
    }

    pub(crate) fn get_output(self: &Self) -> Vec<(Part,usize)>{
        match self {
            Building::Smelter       {output:(x,             ), .. } => Vec::from([(Part::Conveyor(x.kind),x.count)]),
            Building::Foundry       {output:(a,             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Constructor   {output:(a,             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Assembler     {output:(a,             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Manufacturer  {output:(a,             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Refinery      {output:(None,   None   ), .. } => Vec::<(Part,usize)>::new(),
            Building::Refinery      {output:(Some(a),Some(b)), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Pipe(b.kind),b.count)]),
            Building::Refinery      {output:(Some(a),None   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Refinery      {output:(None,   Some(b)), .. } => Vec::from([(Part::Pipe(b.kind),b.count)]),
            Building::Blender       {output:(Some(a),Some(b)), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Pipe(b.kind),b.count)]),
            Building::Blender       {output:(Some(a),None   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Blender       {output:(None,   Some(b)), .. } => Vec::from([(Part::Pipe(b.kind),b.count),]),
            Building::Blender       {output:(None,   None   ), .. } => Vec::<(Part,usize)>::new(),
            Building::Packager      {output:(a,      None   ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Packager      {output:(a,      Some(b)), .. } => Vec::from([(Part::Conveyor(a.kind),a.count),(Part::Pipe(b.kind),b.count)]),
            Building::BioPlant      {                          .. } => Vec::<(Part,usize)>::new(),
            Building::CoalPlant     {                          .. } => Vec::<(Part,usize)>::new(),
            Building::OilPlant      {                          .. } => Vec::<(Part,usize)>::new(),
            Building::NuclearPlant  {output:(a,             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Miner1        {output:(a,             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Miner2        {output:(a,             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::Miner3        {output:(a,             ), .. } => Vec::from([(Part::Conveyor(a.kind),a.count)]),
            Building::WaterExtractor{output:(a,             ), .. } => Vec::from([(Part::Pipe(a.kind),a.count)]),
            Building::OilExtractor  {output:(a,             ), .. } => Vec::from([(Part::Pipe(a.kind),a.count)])
        }
    }

    fn get_name(self: &Self) -> String {
        String::from(match self {
            Building::Smelter { .. } => {"smelter"},
            Building::Foundry { .. } => {"foundry"},
            Building::Constructor { .. } => {"constructor"},
            Building::Assembler { .. } => {"assembler"},
            Building::Manufacturer { .. } => {"manufacturer"},
            Building::Refinery { .. } => {"refinery"},
            Building::Blender { .. } => {"blender"},
            Building::Packager { .. } => {"packager"},
            Building::BioPlant { .. } => {"bio_plant"},
            Building::CoalPlant { .. } => {"coal_plan"},
            Building::OilPlant { .. } => {"oil_plan"},
            Building::NuclearPlant { .. } => {"nul_plan"},
            Building::Miner1 { .. } => {"miner"},
            Building::Miner2 { .. } => {"miner"},
            Building::Miner3 { .. } => {"miner"},
            Building::WaterExtractor { .. } => {"water_extractor"},
            Building::OilExtractor { .. } => {"oil_extractor"},
        })
    }
}

impl Display for Building{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pt1 = self.get_input().iter()
            .map(|a| a.0.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let pt2 = self.get_output().iter()
            .map(|a| a.0.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}({}->{})", self.get_name(), pt1, pt2)
    }
}


//enum NodeQuality{Low,Med,High}

#[derive(Eq, Debug, Clone, Copy, PartialEq, Hash)]
pub(crate) enum Part{
    Conveyor(Conveyable),
    Pipe(Pipeable),
    Mine(Mineable),
    Pump(Pumpable)
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Conveyor(x) => {x.fmt(f)},
            Part::Pipe(x) => {x.fmt(f)},
            Part::Mine(x) => {x.fmt(f)},
            Part::Pump(x) => {x.fmt(f)},
        }
    }
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Conveyable{
    FeOre ,         CuOre,                  Limestone,
    Coal,           Sulfur,                 RawQuartz,
    CateriumOre,    UOre,                   Bauxite,
    SAM,//raw ores
    PkgdWater,      PkgdOil,                PkgdN,
    PkgdHOil,       PkgdFuel,               PkgdTurbofuel,
    PkgdLBiofuel,   PkgdNAcid,              PkgdAlSol,
    PkgdSAcid,//packaged pipes
    WoodOrLeaves,   Mycelia,//raw biomass
    AlienProtein,   AlienDNA,               Biomass,
    SolidBiofuel,   Fabric,                 CompactedCoal,//processed biomatter
    FlowerPetals,   ColorCartridge,//color stuff
    FicsmasGift,    ActualSnow,             CandyCane,
    FicsmasBow,     FicsmasTree,            Snowball,//constructor fiscmas
    FancyFireworks, SparklyFireworks,       SweetFireworks,//ficsmas fireworks
    FeOrnament ,    CuOrnament,             RedOrnament,
    BlueOrnament,   FicsmasBranch,          OrnamentBundle,
    FicsmasDecoration,FicsmasStar,//assembler fixmas
    FeIngot ,       FeRod ,                 FePlate ,
    Screws,//constructor iron
    CuIngot ,       CuWire ,                Cable,
    CuSheet ,       CuPowder ,//constructor copper
    Concrete,//constructor limestone
    CrushedQuartz,  Silica,                 CrystalOscillator,//constructor quartz
    CateriumIngot,  Quickwire,//constructor caterium
    Plastic,        Rubber,                 PolymerResin,
    EmptyCanister,  PetroleumCoke,//constructor plastic
    PowerSlugBlue,  PowerSlugYellow,        PowerSlugPurple,
    PowerShard,//power slugs
    SteelIngot,     SteelBeam,              SteelPipe,
    IndustrialBeam,//constructor steel
    ReinforcedIronPlate,ModularFrame,       HeavyModularFrame,
    FusedModularFrame,//assembler iron
    Rotor,          Stator,                 Motor,//assembler motors
    CircuitBoard,   HighSpeedConnector,     Computer,
    AILimiter,      RadioControlUnit,       SuperComputer,//assembler computer
    AlIngot,        AlcladSheet,AlCasing,   Heatsink,
    Battery,        CoolingSystem,          AlScrap,
    EmptyFluidTank ,//assembler aluminum
    BaseRebar,      ShatterRebar,           PulseRebar,
    ExplosiveRebar,//rebar ammo
    BlackPowder,    SmokelessPowder,        RifleAmmo,
    HomingRifleAmmo,TurboRifleAmmo,//assembler rifle
    Nobelisk,       GasNobelisk,            ClusterNobelisk,
    PulseNobelisk,  NuclearNobelisk,//assembler nobelisk
    SmartPlating,   VersatileFramework,     AutomatedWiring,
    ModularEngine,  AdaptiveControlUnit,    AssemblyDirectorSystem,//assembler elevator
    EMControlRod,   PressureConversionCube,//assembler pre-nuclear
    EncasedUCell,   URod,                   UWaste,
    NonFissileU,//uranium
    PuPellet,       PuRod,                  EncasedPuCell,
    NuclearPasta,   PWaste,//assembler nuclear
    TurboMotor,     MagneticFieldGenerator, ThermalPropulsionRocket,
    Beacon,         PortableMiner,          GasFilter,
    IodineFilter,//holdable
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Pipeable{
    Water, CrudeOil, NGas,      HeavyOil,
    Fuel,  Turbofuel, LBioFuel,
    NAcid, AlSol,     SAcid
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Mineable{
    FeNode ,     CuNode ,   LimestoneNode,//nodes
    CoalNode,    SulfurNode,QuartzNode,
    CateriumNode,UNode,     BauxiteNode,
    SAMNode,
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Pumpable{
    WaterSource, OilSource,   NNode,
}

#[derive(Clone,Copy,Debug, PartialEq)]
pub(crate) struct Amount<T>{
    pub(crate) count: usize,
    pub(crate) kind: T
}

impl<T> Amount<T> {
    pub(crate) const fn new(count:usize, kind:T) -> Self{
        Self{count,kind}
    }
}
