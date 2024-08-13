
pub(crate) struct Process{
    pub(crate)name: String,
    pub(crate)time: usize,
    pub(crate)building: Building,
    pub(crate)tier: usize,
}

pub(crate) enum Building{
    Smelter{
        input: (dyn Conveyed),
        output:(dyn Conveyed),
    },
    Foundry{
        input: (dyn Conveyed, Option<dyn Conveyed>),
        output: (dyn Conveyed),
    },
    Constructor{
        input: (dyn Conveyed),
        output: (dyn Conveyed),
    },
    Assembler{
        input: (dyn Conveyed, Option<dyn Conveyed>),
        output: (dyn Conveyed),
    },
    Manufacturer{
        input: (dyn Conveyed, Option<dyn Conveyed>, Option<dyn Conveyed>, Option<dyn Conveyed>),
        output: (dyn Conveyed),
    },
    Refinery{
        input: (Option<dyn Conveyed>, Option<dyn Piped>),
        output:(Option<dyn Conveyed>, Option<dyn Piped>),
    },
    Blender {
        input: (Option<dyn Conveyed>, Option<dyn Conveyed>, Option<dyn Piped>, Option<dyn Piped>),
        output: (Option<dyn Conveyed>, Option<dyn Piped>),
    },
    Packager{
        input: (Option<dyn Conveyed>, Option<dyn Piped>),
        output:(Option<dyn Conveyed>, Option<dyn Piped>),
    },
    BioPlant{
        input:(Part)
    },
    CoalPlant{
        input:(dyn Conveyed, dyn Piped)
    },
    OilPlant{
        input: (dyn Piped)
    },
    NuclearPlant{
        input: (dyn Conveyed, dyn Piped)
    },
    Miner1{
        input: (dyn Extracted),
        output:(dyn Conveyed),
    },
    Miner2{
        input: (dyn Extracted),
        output:(dyn Conveyed),
    },
    Miner3{
        input: (dyn Extracted),
        output:(dyn Conveyed),
    },
    WaterExtractor {
        input: (dyn Extracted),
        output:(dyn Conveyed),
    },
    OilExtractor {
        input:(dyn Extracted),
        output:(dyn PumpExtracted),
    },
}

trait Conveyed{}
trait Piped{}
trait Extracted{}
trait PumpExtracted{}
trait Fracked{}

enum NodeQuality{Low,Med,High}

pub(crate) enum Part{
    IronNode{amount:usize},    CopperNode{amount:usize}, LimestoneNode{amount:usize},//nodes
    CoalNode{amount:usize},    SulfurNode{amount:usize}, QuartzNode{amount:usize},
    CateriumNode{amount:usize},UNode{amount:usize},      BauxiteNode{amount:usize},
    SAMNode{amount:usize},     IronOre{amount:usize},    CopperOre{amount:usize},
    Limestone{amount:usize},   Coal{amount:usize},       Sulfur{amount:usize},
    RawQuartz{amount:usize},   CateriumOre{amount:usize},UOre{amount:usize},
    Bauxite{amount:usize},     SAM{amount:usize},//raw ores
    WaterSource{amount:usize}, CrudeOil{amount:usize},   NNode{amount:usize},
    Water{amount:usize},       NGas{amount:usize},       HeavyOil{amount:usize},
    Fuel{amount:usize},        Turbofuel{amount:usize},  LBioFuel{amount:usize},
    NAcid{amount:usize},       AlSol{amount:usize},      SAcid{amount:usize},//pipes
    PkgdWater{amount:usize},   PkgdOil{amount:usize},    PkgdN{amount:usize},
    PkgdHOil{amount:usize},    PkgdFuel{amount:usize},   PkgdTurbofuel{amount:usize},
    PkgdLBiofuel{amount:usize},PkgdNAcid{amount:usize},  PkgdAlSol{amount:usize},
    PkgdSAcid{amount:usize},//packaged pipes
    Leaves{amount:usize},      Wood{amount:usize},       Mycelia{amount:usize},
    HatcherProtein{amount:usize},HogProtein{amount:usize},SpitterProtein{amount:usize},
    StingerProtein{amount:usize},//raw biomass
    AlienProtein{amount:usize},AlienDNA{amount:usize},   Biomass{amount:usize},
    SolidBiofuel{amount:usize},Fabric{amount:usize},     CompactedCoal{amount:usize},//processed biomatter
    FlowerPetals{amount:usize},ColorCartridge{amount:usize},//color stuff
    FicsmasGift{amount:usize}, ActualSnow{amount:usize}, CandyCane{amount:usize},
    FicsmasBow{amount:usize},  FicsmasTree{amount:usize},Snowball{amount:usize},//constructor fiscmas
    FancyFireworks{amount:usize},SparklyFireworks{amount:usize},SweetFireworks{amount:usize},//ficsmas fireworks
    IronOrnament{amount:usize},CopperOrnament{amount:usize},RedOrnament{amount:usize},
    BlueOrnament{amount:usize},FicsmasBranch{amount:usize},OrnamentBundle{amount:usize},
    FicsmasDecoration{amount:usize},FicsmasStar{amount:usize},//assembler fixmas
    IronIngot{amount:usize},   IronRod{amount:usize},    IronPlate{amount:usize},
    Screws{amount:usize},//constructor iron
    CopperIngot{amount:usize},CopperWire{amount:usize},  Cable{amount:usize},
    CopperSheet{amount:usize},CopperPowder{amount:usize},//constructor copper
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
    AlcladSheet{amount:usize},AlCasing{amount:usize},    Heatsink{amount:usize},
    Battery{amount:usize},    CoolingSystem{amount:usize},AlScrap{amount:usize},//assembler aluminum
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
    PuPellet{amount:usize},PuRod{amount:usize},          EncasedPuCell{amount:usize},
    NuclearPasta{amount:usize},PWaste{amount:usize},//assembler nuclear
    TurboMotor{amount:usize},MagneticFieldGenerator{amount:usize},ThermalPropulsionRocket{amount:usize},
    Beacon{amount:usize},    PortableMiner{amount:usize},GasFilter{amount:usize},
    IodineFilter{amount:usize},//holdable
}

impl Extracted for Part::IronNode{}
impl Extracted for Part::CopperNode{}
impl Extracted for Part::LimestoneNode{}
impl Extracted for Part::CoalNode{}
impl Extracted for Part::SulfurNode{}
impl Extracted for Part::QuartzNode{}
impl Extracted for Part::CateriumNode{}
impl Extracted for Part::UNode{}
impl Extracted for Part::BauxiteNode{}
impl Extracted for Part::SAMNode{}

impl Conveyed for Part::IronOre{}
impl Conveyed for Part::CopperOre{}
impl Conveyed for Part::Limestone{}
impl Conveyed for Part::Coal{}
impl Conveyed for Part::Sulfur{}
impl Conveyed for Part::RawQuartz{}
impl Conveyed for Part::CateriumOre{}
impl Conveyed for Part::UOre{}
impl Conveyed for Part::Bauxite{}
impl Conveyed for Part::SAM{}

impl PumpExtracted for Part::WaterSource{}
impl Fracked for Part::WaterSource{}
impl Piped for Part::Water{}
impl Piped for Part::CrudeOil{}
impl Fracked for Part::NNode{}
impl Piped for Part::NGas{}
impl Piped for Part::HeavyOil{}
impl Piped for Part::Fuel{}
impl Piped for Part::Turbofuel{}
impl Piped for Part::LBioFuel{}
impl Piped for Part::NAcid{}
impl Piped for Part::AlSol{}
impl Piped for Part::SAcid{}

impl Conveyed for Part::PkgdWater{}
impl Conveyed for Part::PkgdAlSol{}
impl Conveyed for Part::PkgdFuel{}
impl Conveyed for Part::PkgdN{}
impl Conveyed for Part::PkgdHOil{}
impl Conveyed for Part::PkgdFuel{}
impl Conveyed for Part::PkgdTurbofuel{}
impl Conveyed for Part::PkgdLBiofuel{}
impl Conveyed for Part::PkgdNAcid{}
impl Conveyed for Part::PkgdAlSol{}
impl Conveyed for Part::PkgdSAcid{}

impl Conveyed for Part::Leaves{}
impl Conveyed for Part::Wood{}
impl Conveyed for Part::Mycelia{}
impl Conveyed for Part::HatcherProtein{}
impl Conveyed for Part::HogProtein{}
impl Conveyed for Part::SpitterProtein{}
impl Conveyed for Part::StingerProtein{}
impl Conveyed for Part::AlienProtein{}
impl Conveyed for Part::AlienDNA{}
impl Conveyed for Part::Biomass{}
impl Conveyed for Part::SolidBiofuel{}
impl Conveyed for Part::Fabric{}
impl Conveyed for Part::CompactedCoal{}

impl Conveyed for Part::FlowerPetals{}

impl Conveyed for Part::ColorCartridge{}

impl Conveyed for Part::FicsmasGift{}
impl Conveyed for Part::FicsmasBow{}
impl Conveyed for Part::FicsmasBranch{}
impl Conveyed for Part::FicsmasDecoration{}
impl Conveyed for Part::FicsmasStar{}
impl Conveyed for Part::FicsmasTree{}
impl Conveyed for Part::ActualSnow{}
impl Conveyed for Part::Snowball{}
impl Conveyed for Part::FancyFireworks{}
impl Conveyed for Part::SparklyFireworks{}
impl Conveyed for Part::SweetFireworks{}
impl Conveyed for Part::IronOrnament{}
impl Conveyed for Part::CopperOrnament{}
impl Conveyed for Part::RedOrnament{}
impl Conveyed for Part::BlueOrnament{}
impl Conveyed for Part::CandyCane{}
impl Conveyed for Part::Snowball{}
impl Conveyed for Part::OrnamentBundle{}
impl Conveyed for Part::IronIngot{}
impl Conveyed for Part::IronRod{}
impl Conveyed for Part::IronPlate{}
impl Conveyed for Part::Screws{}
impl Conveyed for Part::CopperIngot{}
impl Conveyed for Part::CopperWire{}
impl Conveyed for Part::Cable{}
impl Conveyed for Part::CopperSheet{}
impl Conveyed for Part::CopperPowder{}
impl Conveyed for Part::Concrete{}
impl Conveyed for Part::CrushedQuartz{}
impl Conveyed for Part::Silica{}
impl Conveyed for Part::CrystalOscillator{}
impl Conveyed for Part::CateriumIngot{}
impl Conveyed for Part::Quickwire{}
impl Conveyed for Part::Plastic{}
impl Conveyed for Part::Rubber{}
impl Conveyed for Part::PolymerResin{}
impl Conveyed for Part::EmptyCanister{}
impl Conveyed for Part::PetroleumCoke{}
impl Conveyed for Part::PowerSlugBlue{}
impl Conveyed for Part::PowerSlugYellow{}
impl Conveyed for Part::PowerSlugPurple{}
impl Conveyed for Part::PowerShard{}
impl Conveyed for Part::SteelIngot{}
impl Conveyed for Part::SteelBeam{}
impl Conveyed for Part::SteelPipe{}
impl Conveyed for Part::IndustrialBeam{}
impl Conveyed for Part::ReinforcedIronPlate{}
impl Conveyed for Part::ModularFrame{}
impl Conveyed for Part::HeavyModularFrame{}
impl Conveyed for Part::FusedModularFrame{}
impl Conveyed for Part::Rotor{}
impl Conveyed for Part::Stator{}
impl Conveyed for Part::Motor{}
impl Conveyed for Part::CircuitBoard{}
impl Conveyed for Part::HighSpeedConnector{}
impl Conveyed for Part::Computer{}
impl Conveyed for Part::AILimiter{}
impl Conveyed for Part::RadioControlUnit{}
impl Conveyed for Part::SuperComputer{}
impl Conveyed for Part::AlcladSheet{}
impl Conveyed for Part::AlCasing{}
impl Conveyed for Part::Heatsink{}
impl Conveyed for Part::Battery{}
impl Conveyed for Part::CoolingSystem{}
impl Conveyed for Part::AlScrap{}
impl Conveyed for Part::BaseRebar{}
impl Conveyed for Part::ShatterRebar{}
impl Conveyed for Part::PulseRebar{}
impl Conveyed for Part::ExplosiveRebar{}
impl Conveyed for Part::BlackPowder{}
impl Conveyed for Part::SmokelessPowder{}
impl Conveyed for Part::RifleAmmo{}
impl Conveyed for Part::HomingRifleAmmo{}
impl Conveyed for Part::TurboRifleAmmo{}
impl Conveyed for Part::Nobelisk{}
impl Conveyed for Part::GasNobelisk{}
impl Conveyed for Part::ClusterNobelisk{}
impl Conveyed for Part::PulseNobelisk{}
impl Conveyed for Part::NuclearNobelisk{}
impl Conveyed for Part::SmartPlating{}
impl Conveyed for Part::VersatileFramework{}
impl Conveyed for Part::AutomatedWiring{}
impl Conveyed for Part::ModularEngine{}
impl Conveyed for Part::AdaptiveControlUnit{}
impl Conveyed for Part::AssemblyDirectorSystem{}
impl Conveyed for Part::EMControlRod{}
impl Conveyed for Part::PressureConversionCube{}
impl Conveyed for Part::EncasedUCell{}
impl Conveyed for Part::URod{}
impl Conveyed for Part::UWaste{}
impl Conveyed for Part::NonFissileU{}
impl Conveyed for Part::PuPellet{}
impl Conveyed for Part::PuRod{}
impl Conveyed for Part::EncasedPuCell{}
impl Conveyed for Part::NuclearPasta{}
impl Conveyed for Part::PWaste{}
impl Conveyed for Part::TurboMotor{}
impl Conveyed for Part::MagneticFieldGenerator{}
impl Conveyed for Part::ThermalPropulsionRocket{}
impl Conveyed for Part::Beacon{}
impl Conveyed for Part::PortableMiner{}
impl Conveyed for Part::GasFilter{}
impl Conveyed for Part::IodineFilter{}

pub(crate) fn get_power(building: &Building) -> isize{
    match building{
        Building::Smelter { .. } => 4,
        Building::Foundry { .. } => 16,
        Building::Constructor {..} => 4,
        Building::Assembler {..} => 15,
        Building::Manufacturer {..} => 55,
        Building::Refinery {..}   => 30,
        Building::Blender {..} => 75,
        Building::Packager {..} => 10,
        Building::BioPlant {..} => -25,
        Building::CoalPlant {..} => -75,
        Building::OilPlant {..} => -150,
        Building::NuclearPlant {..} => -2500,
        Building::Miner1 {..} => 5,
        Building::Miner2 {..} => 12,
        Building::Miner3 {..} => 30,
        Building::WaterExtractor {..} => 20,
        Building::OilExtractor {..} => 40
    }
}