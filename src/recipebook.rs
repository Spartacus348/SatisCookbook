use crate::objects::{Amount, Building::*, Conveyable,
                     Conveyable::*, Mineable, Mineable::*, Pipeable,
                     Pipeable::*, Process, Pumpable, Pumpable::*};
use crate::tiers::{CateriumTier::*, FicsmasTier::*, FlowerTier::*, FungusTier::*,
                   HardDriveTier::*, MamTrees::*, OrgoTier::*, ProgressTier::*, QuartzTier::*,
                   SlugTier::*, SulfurTier::*, Tier::*, Tier0::*, Tier1::*, Tier2::*, Tier3::*,
                   Tier4::*, Tier5::*, Tier7::*, Tier8::*};


pub static RECIPES: [Process; 144] = [
    Process {
        name: "Mine Iron",
        time: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, FeNode),),
            output: (Amount::<Conveyable>::new(1, FeOre),)
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Mine Copper",
        time: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, CuNode),),
            output: (Amount::<Conveyable>::new(1, CuOre),),
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Mine Caterium",
        time: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, CateriumNode),),
            output: (Amount::<Conveyable>::new(1, CateriumOre),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Mine Coal",
        time: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, CoalNode),),
            output: (Amount::<Conveyable>::new(1, Coal),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Mine Sulfur",
        time: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, SulfurNode),),
            output: (Amount::<Conveyable>::new(1, Sulfur),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Mine Raw Quartz",
        time: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, QuartzNode),),
            output: (Amount::<Conveyable>::new(1, RawQuartz),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Blue Ficsmas Ornament",
        time: 12,
        building: Smelter {
            input: (Amount::<Conveyable>::new(1, FicsmasGift),),
            output: (Amount::<Conveyable>::new(2, BlueOrnament),)
        },
        tier: MAM(Ficsmas(TreeUpgrade1))
    },
    Process {
        name: "Caterium Ingot",
        time: 4,
        building: Smelter {
            input: (Amount::<Conveyable>::new(3, CateriumOre),),
            output: (Amount::<Conveyable>::new(1, CateriumIngot),)
        },
        tier: MAM(Caterium(CateriumIngotResearch))
    },
    Process {
        name: "Copper Ingot",
        time: 2,
        building: Smelter {
            input: (Amount::<Conveyable>::new(1, CuOre),),
            output: (Amount::<Conveyable>::new(1, CuIngot),)
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Iron Ingot",
        time: 2,
        building: Smelter {
            input: (Amount::<Conveyable>::new(1, FeOre),),
            output: (Amount::<Conveyable>::new(1, FeIngot),),
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Red Ficsmas Ornament",
        time: 12,
        building: Smelter {
            input: (Amount::<Conveyable>::new(1, FicsmasGift),),
            output: (Amount::<Conveyable>::new(1, RedOrnament),)
        },
        tier: MAM(Ficsmas(TreeUpgrade1))
    },
    Process {
        name: "Pure Aluminum Ingot",
        time: 2,
        building: Smelter {
            input: (Amount::<Conveyable>::new(2, AlScrap),),
            output: (Amount::<Conveyable>::new(1, AlIngot),)
        },
        tier: HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process {
        name: "Extract Water",
        time: 1,
        building: WaterExtractor {
            input: (Amount::<Pumpable>::new(1, WaterSource),),
            output: (Amount::<Pipeable>::new(120, Water),)
        },
        tier: MainProgression(Tier3(CoalPower))
    },
    Process {
        name: "Extract Oil",
        time: 1,
        building: OilExtractor {
            input: (Amount::<Pumpable>::new(1, OilSource),),
            output: (Amount::<Pipeable>::new(1, CrudeOil),)
        },
        tier: MainProgression(Tier5(OilProcessing))
    },
    Process {
        name: "Aluminum Ingot",
        time: 4,
        building: Foundry {
            input: (Amount::<Conveyable>::new(6, AlScrap), Amount::<Conveyable>::new(5, Silica)),
            output: (Amount::<Conveyable>::new(4, AlIngot),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Copper Ficsmas Ornament",
        time: 12,
        building: Foundry {
            input: (Amount::<Conveyable>::new(2, RedOrnament), Amount::<Conveyable>::new(2, CuIngot)),
            output: (Amount::<Conveyable>::new(1, CuOrnament),)
        },
        tier: MAM(Ficsmas(FicsmasGiftTree))
    },
    Process {
        name: "Iron Ficsmas Ornament",
        time: 12,
        building: Foundry {
            input: (Amount::<Conveyable>::new(3, BlueOrnament), Amount::<Conveyable>::new(3, FeIngot)),
            output: (Amount::<Conveyable>::new(1, FeOrnament),)
        },
        tier: MAM(Ficsmas(FicsmasGiftTree))
    },
    Process {
        name: "Steel Ingot",
        time: 4,
        building: Foundry {
            input: (Amount::<Conveyable>::new(3, FeOre), Amount::<Conveyable>::new(3, Coal)),
            output: (Amount::<Conveyable>::new(3, SteelIngot),)
        },
        tier: MainProgression(Tier3(BasicSteel))
    },
    Process {
        name: "Coke Steel Ingot",
        time: 12,
        building: Foundry {
            input: (Amount::<Conveyable>::new(15, FeOre), Amount::<Conveyable>::new(15, PetroleumCoke)),
            output: (Amount::<Conveyable>::new(20, SteelIngot),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Compacted Steel Ingot",
        time: 16,
        building: Foundry {
            input: (Amount::<Conveyable>::new(15, FeOre), Amount::<Conveyable>::new(3, CompactedCoal)),
            output: (Amount::<Conveyable>::new(10, SteelIngot),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Compacted Steel Ingot",
        time: 16,
        building: Foundry {
            input: (Amount::<Conveyable>::new(15, FeOre), Amount::<Conveyable>::new(3, CompactedCoal)),
            output: (Amount::<Conveyable>::new(10, SteelIngot),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Copper Alloy Ingot",
        time: 12,
        building: Foundry {
            input: (Amount::<Conveyable>::new(10, CuOre), Amount::<Conveyable>::new(5, FeOre)),
            output: (Amount::<Conveyable>::new(20, CuIngot),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Iron Alloy Ingot",
        time: 6,
        building: Foundry {
            input: (Amount::<Conveyable>::new(2, FeOre), Amount::<Conveyable>::new(2, CuOre)),
            output: (Amount::<Conveyable>::new(5, FeIngot),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Solid Steel Ingot",
        time: 3,
        building: Foundry {
            input: (Amount::<Conveyable>::new(2, FeIngot), Amount::<Conveyable>::new(2, Coal)),
            output: (Amount::<Conveyable>::new(3, SteelIngot),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Actual Snow",
        time: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FicsmasGift),),
            output: (Amount::<Conveyable>::new(2, ActualSnow),)
        },
        tier: MAM(Ficsmas(AFriend))
    },
    Process {
        name: "Alien DNA Capsule",
        time: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, AlienProtein),),
            output: (Amount::<Conveyable>::new(1, AlienDNA),)
        },
        tier: MAM(Organisms(BioOrganicProperties))
    },
    Process {
        name: "Aluminum Casing",
        time: 2,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, AlIngot),),
            output: (Amount::<Conveyable>::new(2, AlCasing),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Biomass (Alien Protein)",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, AlienProtein),),
            output: (Amount::<Conveyable>::new(100, Biomass),)
        },
        tier: MAM(Organisms(BioOrganicProperties))
    },
    Process {
        name: "Biomass (Mycelia)",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, Mycelia),),
            output: (Amount::<Conveyable>::new(10, Biomass),)
        },
        tier: MAM(Fungi(MyceliaResearch))
    },
    Process {
        name: "Biomass (Wood)",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(4, WoodOrLeaves),),
            output: (Amount::<Conveyable>::new(20, Biomass),)
        },
        tier: MainProgression(Tier0(HubUpgrade6))
    },
    Process {
        name: "Cable",
        time: 2,
        building: Constructor {
            input: (Amount::<Conveyable>::new(2, CuWire),),
            output: (Amount::<Conveyable>::new(1, Cable),)
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Candy Cane",
        time: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, FicsmasGift),),
            output: (Amount::<Conveyable>::new(5, CandyCane),)
        },
        tier: MAM(Ficsmas(CandyCaneBasher))
    },
    Process {
        name: "Color Cartridge",
        time: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FlowerPetals),),
            output: (Amount::<Conveyable>::new(10, ColorCartridge),)
        },
        tier: MAM(Flowers(ColorCartridges))
    },
    Process {
        name: "Color Cartridge",
        time: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FlowerPetals),),
            output: (Amount::<Conveyable>::new(10, ColorCartridge),)
        },
        tier: MainProgression(Tier2(ResourceSinkBonus))
    },
    Process {
        name: "Concrete",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, Limestone),),
            output: (Amount::<Conveyable>::new(1, Concrete),)
        },
        tier: MainProgression(Tier0(HubUpgrade3))
    },
    Process {
        name: "Copper Powder",
        time: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(30, CuIngot),),
            output: (Amount::<Conveyable>::new(5, CuPowder),)
        },
        tier: MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name: "Copper Sheet",
        time: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(2, CuIngot),),
            output: (Amount::<Conveyable>::new(1, CuSheet),)
        },
        tier: MainProgression(Tier2(PartAssembly))
    },
    Process {
        name: "Empty Canister",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(2, Plastic),),
            output: (Amount::<Conveyable>::new(4, EmptyCanister),)
        },
        tier: MainProgression(Tier5(AlternativeFuelTransport))
    },
    Process {
        name: "Empty Fluid Tank",
        time: 1,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, AlIngot),),
            output: (Amount::<Conveyable>::new(1, EmptyFluidTank),)
        },
        tier: MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process {
        name: "Ficsmas Bow",
        time: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(2, FicsmasGift),),
            output: (Amount::<Conveyable>::new(1, FicsmasBow),)
        },
        tier: MAM(Ficsmas(CandyCaneDecor))
    },
    Process {
        name: "Ficsmas Tree Branch",
        time: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, FicsmasGift),),
            output: (Amount::<Conveyable>::new(1, FicsmasBranch),)
        },
        tier: MAM(Ficsmas(TreeUpgrade0))
    },
    Process {
        name: "Iron Plate",
        time: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, FeIngot),),
            output: (Amount::<Conveyable>::new(2, FePlate),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Iron Rebar",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, FeRod),),
            output: (Amount::<Conveyable>::new(1, BaseRebar),)
        },
        tier: MAM(Organisms(RebarGun))
    },
    Process {
        name: "Iron Rod",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, FeIngot),),
            output: (Amount::<Conveyable>::new(1, FeRod),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Power Shard (1)",
        time: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, PowerSlugBlue),),
            output: (Amount::<Conveyable>::new(1, PowerShard),)
        },
        tier: MAM(PowerSlugs(BluePowerSlugs))
    },
    Process {
        name: "Power Shard (2)",
        time: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, PowerSlugYellow),),
            output: (Amount::<Conveyable>::new(2, PowerShard),)
        },
        tier: MAM(PowerSlugs(YellowPowerShards))
    },
    Process {
        name: "Power Shard (5)",
        time: 24,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, PowerSlugPurple),),
            output: (Amount::<Conveyable>::new(5, PowerShard),)
        },
        tier: MAM(PowerSlugs(PurplePowerShards))
    },
    Process {
        name: "Quartz Crystal",
        time: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, RawQuartz),),
            output: (Amount::<Conveyable>::new(3, CrushedQuartz),)
        },
        tier: MAM(Quartz(QuartzResearch))
    },
    Process {
        name: "Quickwire",
        time: 5,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, CateriumIngot),),
            output: (Amount::<Conveyable>::new(5, Quickwire),)
        },
        tier: MAM(Caterium(QuickwireResearch))
    },
    Process {
        name: "Screw",
        time: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, FeRod),),
            output: (Amount::<Conveyable>::new(4, Screws),)
        },
        tier: MainProgression(Tier0(HubUpgrade3))
    },
    Process {
        name: "Silica",
        time: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, RawQuartz),),
            output: (Amount::<Conveyable>::new(5, Silica),)
        },
        tier: MAM(Quartz(SilicaResearch))
    },
    Process {
        name: "Silica",
        time: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, RawQuartz),),
            output: (Amount::<Conveyable>::new(5, Silica),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Snowball",
        time: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, ActualSnow),),
            output: (Amount::<Conveyable>::new(1, Snowball),)
        },
        tier: MAM(Ficsmas(Snowfight))
    },
    Process {
        name: "Solid Biofuel",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(8, Biomass),),
            output: (Amount::<Conveyable>::new(4, SolidBiofuel),)
        },
        tier: MainProgression(Tier2(ObstacleClearing))
    },
    Process {
        name: "Steel Beam",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(4, SteelIngot),),
            output: (Amount::<Conveyable>::new(1, SteelBeam),)
        },
        tier: MainProgression(Tier3(BasicSteel))
    },
    Process {
        name: "Steel Pipe",
        time: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, SteelIngot),),
            output: (Amount::<Conveyable>::new(2, SteelPipe),)
        },
        tier: MainProgression(Tier3(BasicSteel))
    },
    Process {
        name: "Wire",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, CuIngot),),
            output: (Amount::<Conveyable>::new(2, CuWire),)
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Biocoal",
        time: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, Biomass),),
            output: (Amount::<Conveyable>::new(6, Coal),)
        },
        tier: HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name: "Biocoal",
        time: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, Biomass),),
            output: (Amount::<Conveyable>::new(6, Coal),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Cast Screw",
        time: 24,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FeIngot),),
            output: (Amount::<Conveyable>::new(20, Screws),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Caterium Wire",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, CateriumIngot),),
            output: (Amount::<Conveyable>::new(8, CuWire),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumResearch)))
    },
    Process {
        name: "Charcoal",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, WoodOrLeaves),),
            output: (Amount::<Conveyable>::new(10, Coal),)
        },
        tier: HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name: "Charcoal",
        time: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, WoodOrLeaves),),
            output: (Amount::<Conveyable>::new(10, Coal),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Iron Wire",
        time: 24,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FeIngot),),
            output: (Amount::<Conveyable>::new(9, CuWire),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Steel Canister",
        time: 3,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, SteelIngot),),
            output: (Amount::<Conveyable>::new(2, EmptyCanister),)
        },
        tier: HardDrive(MainUnlock(Tier5(AlternativeFuelTransport)))
    },
    Process {
        name: "Steel Rod",
        time: 5,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, SteelIngot),),
            output: (Amount::<Conveyable>::new(4, FeRod),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Steel Screw",
        time: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, SteelBeam),),
            output: (Amount::<Conveyable>::new(52, Screws),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "AI Limiter",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, CuSheet), Amount::<Conveyable>::new(20, Quickwire)),
            output: (Amount::<Conveyable>::new(5, AILimiter),)
        },
        tier: MAM(Caterium(AILimiterResearch))
    },
    Process {
        name: "AI Limiter",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, CuSheet), Amount::<Conveyable>::new(20, Quickwire)),
            output: (Amount::<Conveyable>::new(5, AILimiter),)
        },
        tier: MainProgression(Tier7(AeronauticalEngineering))
    },
    Process {
        name: "Alclad Aluminum Sheet",
        time: 6,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, AlIngot), Amount::<Conveyable>::new(1, CuIngot)),
            output: (Amount::<Conveyable>::new(3, AlcladSheet),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Assembly Director System",
        time: 80,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, AdaptiveControlUnit), Amount::<Conveyable>::new(1, SuperComputer)),
            output: (Amount::<Conveyable>::new(1, AssemblyDirectorSystem),)
        },
        tier: MainProgression(Tier7(AeronauticalEngineering))
    },
    Process {
        name: "Automated Wiring",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, Stator), Amount::<Conveyable>::new(20, Cable)),
            output: (Amount::<Conveyable>::new(1, AutomatedWiring),)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process {
        name: "Black Powder",
        time: 4,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, Coal), Amount::<Conveyable>::new(1, Sulfur)),
            output: (Amount::<Conveyable>::new(2, BlackPowder),)
        },
        tier: MAM(SulfurTier(BlackPowderResearch))
    },
    Process {
        name: "Circuit Board",
        time: 8,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, CuSheet), Amount::<Conveyable>::new(4, Plastic)),
            output: (Amount::<Conveyable>::new(1, CircuitBoard),)
        },
        tier: MainProgression(Tier5(OilProcessing))
    },
    Process {
        name: "Cluster Nobelisk",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Nobelisk), Amount::<Conveyable>::new(4, SmokelessPowder)),
            output: (Amount::<Conveyable>::new(1, ClusterNobelisk),)
        },
        tier: MAM(SulfurTier(ClusterNobeliskResearch))
    },
    Process {
        name: "Electromagnetic Control Rod",
        time: 30,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Stator), Amount::<Conveyable>::new(2, AILimiter)),
            output: (Amount::<Conveyable>::new(2, EMControlRod),)
        },
        tier: MainProgression(Tier8(NuclearPower))
    },
    Process {
        name: "Encased Industrial Beam",
        time: 10,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, SteelBeam), Amount::<Conveyable>::new(5, Concrete)),
            output: (Amount::<Conveyable>::new(1, IndustrialBeam),)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process {
        name: "Encased Plutonium Cell",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, PuPellet), Amount::<Conveyable>::new(4, Concrete)),
            output: (Amount::<Conveyable>::new(1, EncasedPuCell),)
        },
        tier: MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name: "Ficsmas Decoration",
        time: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(15, FicsmasBranch), Amount::<Conveyable>::new(6, OrnamentBundle)),
            output: (Amount::<Conveyable>::new(2, FicsmasDecoration),)
        },
        tier: MAM(Ficsmas(ItsSnowing))
    },
    Process {
        name: "Ficsmas Wonder Star",
        time: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, FicsmasDecoration), Amount::<Conveyable>::new(20, CandyCane)),
            output: (Amount::<Conveyable>::new(1, FicsmasStar),)
        },
        tier: MAM(Ficsmas(Wreath))
    },
    Process {
        name: "Ficsmas Ornament Bundle",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, CuOrnament), Amount::<Conveyable>::new(1, FeOrnament)),
            output: (Amount::<Conveyable>::new(1, OrnamentBundle),)
        },
        tier: MAM(Ficsmas(Lights))
    },
    Process {
        name: "Fabric",
        time: 4,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, Mycelia), Amount::<Conveyable>::new(5, Biomass)),
            output: (Amount::<Conveyable>::new(1, Fabric),)
        },
        tier: MAM(Fungi(FabricResearch))
    },
    Process {
        name: "Fancy Fireworks",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, FicsmasBranch), Amount::<Conveyable>::new(3, FicsmasBow)),
            output: (Amount::<Conveyable>::new(1, FancyFireworks),)
        },
        tier: MAM(Ficsmas(Fireworks))
    },
    Process {
        name: "Gas Nobelisk",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, Nobelisk), Amount::<Conveyable>::new(10, Biomass)),
            output: (Amount::<Conveyable>::new(1, GasNobelisk),)
        },
        tier: MAM(Fungi(ToxicCellularModification))
    },
    Process {
        name: "Heat Sink",
        time: 8,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, AlcladSheet), Amount::<Conveyable>::new(3, CuSheet)),
            output: (Amount::<Conveyable>::new(1, Heatsink),)
        },
        tier: MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process {
        name: "Homing Rifle Ammo",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(20, RifleAmmo), Amount::<Conveyable>::new(1, HighSpeedConnector)),
            output: (Amount::<Conveyable>::new(10, HomingRifleAmmo),)
        },
        tier: MAM(Caterium(BulletGuidanceSystem))
    },
    Process {
        name: "Modular Frame",
        time: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, ReinforcedIronPlate), Amount::<Conveyable>::new(12, FeRod)),
            output: (Amount::<Conveyable>::new(2, ModularFrame),)
        },
        tier: MainProgression(Tier2(PartAssembly))
    },
    Process {
        name: "Motor",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, Rotor), Amount::<Conveyable>::new(2, Stator)),
            output: (Amount::<Conveyable>::new(1, Motor),)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process {
        name: "Nobelisk",
        time: 6,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, BlackPowder), Amount::<Conveyable>::new(2, SteelPipe)),
            output: (Amount::<Conveyable>::new(1, Nobelisk),)
        },
        tier: MAM(SulfurTier(NobeliskDetonator))
    },
    Process {
        name: "Pressure Conversion Cube",
        time: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, FusedModularFrame), Amount::<Conveyable>::new(2, RadioControlUnit)),
            output: (Amount::<Conveyable>::new(1, PressureConversionCube),)
        },
        tier: MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name: "Pulse Nobelisk",
        time: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, Nobelisk), Amount::<Conveyable>::new(1, CrystalOscillator)),
            output: (Amount::<Conveyable>::new(5, PulseNobelisk),)
        },
        tier: MAM(Quartz(ExplosiveResonanceApplication))
    },
    Process {
        name: "Reinforced Iron Plate",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(6, FePlate), Amount::<Conveyable>::new(12, Screws)),
            output: (Amount::<Conveyable>::new(1, ReinforcedIronPlate),)
        },
        tier: MainProgression(Tier0(HubUpgrade3))
    },
    Process {
        name: "Rifle Ammo",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, CuSheet), Amount::<Conveyable>::new(2, SmokelessPowder)),
            output: (Amount::<Conveyable>::new(15, RifleAmmo),)
        },
        tier: MAM(SulfurTier(Rifle))
    },
    Process {
        name: "Rotor",
        time: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, FeRod), Amount::<Conveyable>::new(25, Screws)),
            output: (Amount::<Conveyable>::new(1, Rotor),)
        },
        tier: MainProgression(Tier2(PartAssembly))
    },
    Process {
        name: "Shatter Rebar",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, BaseRebar), Amount::<Conveyable>::new(3, CrushedQuartz)),
            output: (Amount::<Conveyable>::new(1, ShatterRebar),)
        },
        tier: MAM(Quartz(ShatterRebarResearch))
    },
    Process {
        name: "Smart Plating",
        time: 30,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, ReinforcedIronPlate), Amount::<Conveyable>::new(1, Rotor)),
            output: (Amount::<Conveyable>::new(1, SmartPlating),)
        },
        tier: MainProgression(Tier2(PartAssembly))
    },
    Process {
        name: "Sparkly Fireworks",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, FicsmasBranch), Amount::<Conveyable>::new(2, ActualSnow)),
            output: (Amount::<Conveyable>::new(1, SparklyFireworks),)
        },
        tier: MAM(Ficsmas(Fireworks))
    },
    Process {
        name: "Stator",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, SteelPipe), Amount::<Conveyable>::new(8, CuWire)),
            output: (Amount::<Conveyable>::new(1, Stator),)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process {
        name: "Stun Rebar",
        time: 6,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, BaseRebar), Amount::<Conveyable>::new(5, Quickwire)),
            output: (Amount::<Conveyable>::new(1, PulseRebar),)
        },
        tier: MAM(Caterium(StunRebarResearch))
    },
    Process {
        name: "Sweet Fireworks",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(6, FicsmasBranch), Amount::<Conveyable>::new(3, CandyCane)),
            output: (Amount::<Conveyable>::new(1, SweetFireworks),)
        },
        tier: MAM(Ficsmas(Fireworks))
    },
    Process {
        name: "Versatile Framework",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, ModularFrame), Amount::<Conveyable>::new(12, SteelBeam)),
            output: (Amount::<Conveyable>::new(2, VersatileFramework),)
        },
        tier: MainProgression(Tier3(BasicSteel))
    },
    Process {
        name: "Adhered Iron Plate",
        time: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, FePlate), Amount::<Conveyable>::new(1, Rubber)),
            output: (Amount::<Conveyable>::new(1, ReinforcedIronPlate),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Alclad Casing",
        time: 8,
        building: Assembler {
            input: (Amount::<Conveyable>::new(20, AlIngot), Amount::<Conveyable>::new(10, CuIngot)),
            output: (Amount::<Conveyable>::new(15, AlCasing),)
        },
        tier: HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process {
        name: "Bolted Frame",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, ReinforcedIronPlate), Amount::<Conveyable>::new(56, Screws)),
            output: (Amount::<Conveyable>::new(2, ModularFrame),)
        },
        tier: HardDrive(MainUnlock(Tier2(PartAssembly)))
    },
    Process {
        name: "Bolted Iron Plate",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(18, FePlate), Amount::<Conveyable>::new(50, Screws)),
            output: (Amount::<Conveyable>::new(3, ReinforcedIronPlate),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Caterium Circuit Board",
        time: 48,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, Plastic), Amount::<Conveyable>::new(30, Quickwire)),
            output: (Amount::<Conveyable>::new(7, CircuitBoard),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Caterium Circuit Board",
        time: 48,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, Plastic), Amount::<Conveyable>::new(30, Quickwire)),
            output: (Amount::<Conveyable>::new(7, CircuitBoard),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process {
        name: "Cheap Silica",
        time: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, RawQuartz), Amount::<Conveyable>::new(5, Limestone)),
            output: (Amount::<Conveyable>::new(7, Silica),)
        },
        tier: HardDrive(MAMUnlock(Quartz(SilicaResearch)))
    },
    Process {
        name: "Coated Iron Canister",
        time: 4,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, FePlate), Amount::<Conveyable>::new(1, CuSheet)),
            output: (Amount::<Conveyable>::new(4, EmptyCanister),)
        },
        tier: HardDrive(MainUnlock(Tier5(AlternativeFuelTransport)))
    },
    Process {
        name: "Coated Iron Plate",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, FeIngot), Amount::<Conveyable>::new(2, Plastic)),
            output: (Amount::<Conveyable>::new(15, FePlate),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Compacted Coal",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, Coal), Amount::<Conveyable>::new(5, Sulfur)),
            output: (Amount::<Conveyable>::new(5, CompactedCoal),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Copper Rotor",
        time: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(6, CuSheet), Amount::<Conveyable>::new(52, Screws)),
            output: (Amount::<Conveyable>::new(3, Rotor),)
        },
        tier: HardDrive(MainUnlock(Tier2(PartAssembly)))
    },
    Process {
        name: "Crystal Computer",
        time: 64,
        building: Assembler {
            input: (Amount::<Conveyable>::new(8, CircuitBoard), Amount::<Conveyable>::new(3, CrystalOscillator)),
            output: (Amount::<Conveyable>::new(3, Computer),)
        },
        tier: HardDrive(MainUnlock(Tier5(IndustrialManufacturing)))
    },
    Process {
        name: "Crystal Computer",
        time: 64,
        building: Assembler {
            input: (Amount::<Conveyable>::new(8, CircuitBoard), Amount::<Conveyable>::new(3, CrystalOscillator)),
            output: (Amount::<Conveyable>::new(3, Computer),)
        },
        tier: HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name: "Electric Motor",
        time: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, EMControlRod), Amount::<Conveyable>::new(2, Rotor)),
            output: (Amount::<Conveyable>::new(2, Motor),)
        },
        tier: HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process {
        name: "Electrode Circuit Board",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(6, Rubber), Amount::<Conveyable>::new(9, PetroleumCoke)),
            output: (Amount::<Conveyable>::new(1, CircuitBoard),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Electromagnetic Connection Rod",
        time: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, Stator), Amount::<Conveyable>::new(1, HighSpeedConnector)),
            output: (Amount::<Conveyable>::new(2, EMControlRod),)
        },
        tier: HardDrive(MainUnlock(Tier8(NuclearPower)))
    },
    Process {
        name: "Electromagnetic Connection Rod",
        time: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, Stator), Amount::<Conveyable>::new(1, HighSpeedConnector)),
            output: (Amount::<Conveyable>::new(2, EMControlRod),)
        },
        tier: HardDrive(MAMUnlock(Caterium(AILimiterResearch)))
    },
    Process {
        name: "Encased Industrial Pipe",
        time: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(7, SteelPipe), Amount::<Conveyable>::new(5, Concrete)),
            output: (Amount::<Conveyable>::new(1, IndustrialBeam),)
        },
        tier: HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process {
        name: "Fine Black Powder",
        time: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, Sulfur), Amount::<Conveyable>::new(1, CompactedCoal)),
            output: (Amount::<Conveyable>::new(4, BlackPowder),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Fine Concrete",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Silica), Amount::<Conveyable>::new(12, Limestone)),
            output: (Amount::<Conveyable>::new(10, Concrete),)
        },
        tier: HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name: "Fused Quickwire",
        time: 8,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, CateriumIngot), Amount::<Conveyable>::new(5, CuIngot)),
            output: (Amount::<Conveyable>::new(12, Quickwire),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process {
        name: "Fused Wire",
        time: 20,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, CuIngot), Amount::<Conveyable>::new(1, CateriumIngot)),
            output: (Amount::<Conveyable>::new(30, CuWire),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumResearch)))
    },
    Process {
        name: "Heat Exchanger",
        time: 6,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, AlCasing), Amount::<Conveyable>::new(3, Rubber)),
            output: (Amount::<Conveyable>::new(1, Heatsink),)
        },
        tier: HardDrive(MainUnlock(Tier8(AdvancedAluminumProduction)))
    },
    Process {
        name: "Insulated Cable",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(9, CuWire), Amount::<Conveyable>::new(6, Rubber)),
            output: (Amount::<Conveyable>::new(20, Cable),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "OC Supercomputer",
        time: 20,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, RadioControlUnit), Amount::<Conveyable>::new(3, CoolingSystem)),
            output: (Amount::<Conveyable>::new(1, SuperComputer),)
        },
        tier: HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process {
        name: "OC Supercomputer",
        time: 20,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, RadioControlUnit), Amount::<Conveyable>::new(3, CoolingSystem)),
            output: (Amount::<Conveyable>::new(1, SuperComputer),)
        },
        tier: HardDrive(MainUnlock(Tier8(AdvancedAluminumProduction)))
    },
    Process {
        name: "Plutonium Fuel Unit",
        time: 120,
        building: Assembler {
            input: (Amount::<Conveyable>::new(20, EncasedPuCell), Amount::<Conveyable>::new(1, PressureConversionCube)),
            output: (Amount::<Conveyable>::new(1, PuRod),)
        },
        tier: HardDrive(MainUnlock(Tier8(ParticleEnrichment)))
    },
    Process {
        name: "Quickwire Cable",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Quickwire),
                    Amount::<Conveyable>::new(2, Rubber)),
            output: (Amount::<Conveyable>::new(11, Cable),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Quickwire Cable",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Quickwire),
                    Amount::<Conveyable>::new(2, Rubber)),
            output: (Amount::<Conveyable>::new(11, Cable),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process {
        name: "Quickwire Stator",
        time: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, SteelPipe),
                    Amount::<Conveyable>::new(15, Quickwire)),
            output: (Amount::<Conveyable>::new(2, Stator),)
        },
        tier: HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process {
        name: "Quickwire Stator",
        time: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, SteelPipe),
                    Amount::<Conveyable>::new(15, Quickwire)),
            output: (Amount::<Conveyable>::new(2, Stator),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process {
        name: "Rubber Concrete",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, Limestone),
                    Amount::<Conveyable>::new(2, Rubber)),
            output: (Amount::<Conveyable>::new(9, Concrete),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Silicon Circuit Board",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(11, CuSheet),
                    Amount::<Conveyable>::new(11, Silica)),
            output: (Amount::<Conveyable>::new(5, CircuitBoard),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Silicon Circuit Board",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(11, CuSheet),
                    Amount::<Conveyable>::new(11, Silica)),
            output: (Amount::<Conveyable>::new(5, CircuitBoard),)
        },
        tier: HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name: "Steel Coated Plate",
        time: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, SteelIngot),
                    Amount::<Conveyable>::new(2, Plastic)),
            output: (Amount::<Conveyable>::new(18, FePlate),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Steel Rotor",
        time: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, SteelPipe),
                    Amount::<Conveyable>::new(6, CuWire)),
            output: (Amount::<Conveyable>::new(1, Rotor),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Steeled Frame",
        time: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, ReinforcedIronPlate),
                    Amount::<Conveyable>::new(10, SteelPipe)),
            output: (Amount::<Conveyable>::new(3, ModularFrame),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Stitched Iron Plate",
        time: 32,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, FePlate), Amount::<Conveyable>::new(20, CuWire)),
            output: (Amount::<Conveyable>::new(3, ReinforcedIronPlate),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Adaptive Control Unit",
        time: 120,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(15, AutomatedWiring),
                    Amount::<Conveyable>::new(10, CircuitBoard),
                    Amount::<Conveyable>::new(2, HeavyModularFrame),
                    Some(Amount::<Conveyable>::new(2, Computer))),
            output: (Amount::<Conveyable>::new(2, AdaptiveControlUnit),)
        },
        tier: MainProgression(Tier5(IndustrialManufacturing))
    },
    Process {
        name: "Beacon",
        time: 8,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(3, FePlate),
                    Amount::<Conveyable>::new(1, FeRod),
                    Amount::<Conveyable>::new(15, CuWire),
                    Some(Amount::<Conveyable>::new(2, Cable))),
            output: (Amount::<Conveyable>::new(1, Beacon),)
        },
        tier: MainProgression(Tier1(FieldResearch))
    },
    Process {
        name: "Computer",
        time: 24,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(10, CircuitBoard),
                    Amount::<Conveyable>::new(9, Cable, ),
                    Amount::<Conveyable>::new(18, Plastic, ),
                    Some(Amount::<Conveyable>::new(52, Screws, ))),
            output: (Amount::<Conveyable>::new(1, Computer, ),)
        },
        tier: MainProgression(Tier5(IndustrialManufacturing))
    },
    Process {
        name: "Crystal Oscillator",
        time: 120,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(36, CrushedQuartz),
                    Amount::<Conveyable>::new(28, Cable, ),
                    Amount::<Conveyable>::new(5, ReinforcedIronPlate, ),
                    None),
            output: (Amount::<Conveyable>::new(2, CrystalOscillator, ),)
        },
        tier: MAM(Quartz(CrystalOscillatorResearch))
    },
    Process {
        name: "Crystal Oscillator",
        time: 120,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(36, CrushedQuartz),
                    Amount::<Conveyable>::new(28, Cable, ),
                    Amount::<Conveyable>::new(5, ReinforcedIronPlate, ),
                    None),
            output: (Amount::<Conveyable>::new(2, CrystalOscillator, ),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
];
