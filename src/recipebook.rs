use crate::objects::{Amount, Building::*, Conveyable, Conveyable::*, Mineable, Mineable::*,
                     Pipeable, Pipeable::*, Process, Pumpable, Pumpable::*};
use crate::tiers::{CateriumTier::*, FicsmasTier::*, FlowerTier::*, FungusTier::*, HardDriveTier::*,
                   MamTrees::*, OrgoTier::*, ProgressTier::*, QuartzTier::*, SlugTier::*,
                   SulfurTier::*, Tier::*, Tier0::*, Tier1::*, Tier2::*, Tier3::*, Tier4::*,
                   Tier5::*, Tier7::*, Tier8::*};


pub static RECIPES: [Process] = *[
    Process {
        name: "Mine Iron",
        time_s: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, FeNode),),
            output: (Amount::<Conveyable>::new(1, FeOre),)
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Mine Copper",
        time_s: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, CuNode),),
            output: (Amount::<Conveyable>::new(1, CuOre),),
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Mine Caterium",
        time_s: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, CateriumNode),),
            output: (Amount::<Conveyable>::new(1, CateriumOre),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Mine Coal",
        time_s: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, CoalNode),),
            output: (Amount::<Conveyable>::new(1, Coal),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Mine Sulfur",
        time_s: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, SulfurNode),),
            output: (Amount::<Conveyable>::new(1, Sulfur),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Mine Raw Quartz",
        time_s: 1,
        building: Miner1 {
            input: (Amount::<Mineable>::new(1, QuartzNode),),
            output: (Amount::<Conveyable>::new(1, RawQuartz),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Blue Ficsmas Ornament",
        time_s: 12,
        building: Smelter {
            input: (Amount::<Conveyable>::new(1, FicsmasGift),),
            output: (Amount::<Conveyable>::new(2, BlueOrnament),)
        },
        tier: MAM(Ficsmas(TreeUpgrade1))
    },
    Process {
        name: "Caterium Ingot",
        time_s: 4,
        building: Smelter {
            input: (Amount::<Conveyable>::new(3, CateriumOre),),
            output: (Amount::<Conveyable>::new(1, CateriumIngot),)
        },
        tier: MAM(Caterium(CateriumIngotResearch))
    },
    Process {
        name: "Copper Ingot",
        time_s: 2,
        building: Smelter {
            input: (Amount::<Conveyable>::new(1, CuOre),),
            output: (Amount::<Conveyable>::new(1, CuIngot),)
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Iron Ingot",
        time_s: 2,
        building: Smelter {
            input: (Amount::<Conveyable>::new(1, FeOre),),
            output: (Amount::<Conveyable>::new(1, FeIngot),),
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Red Ficsmas Ornament",
        time_s: 12,
        building: Smelter {
            input: (Amount::<Conveyable>::new(1, FicsmasGift),),
            output: (Amount::<Conveyable>::new(1, RedOrnament),)
        },
        tier: MAM(Ficsmas(TreeUpgrade1))
    },
    Process {
        name: "Pure Aluminum Ingot",
        time_s: 2,
        building: Smelter {
            input: (Amount::<Conveyable>::new(2, AlScrap),),
            output: (Amount::<Conveyable>::new(1, AlIngot),)
        },
        tier: HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process {
        name: "Extract Water",
        time_s: 1,
        building: WaterExtractor {
            input: (Amount::<Pumpable>::new(1, WaterSource),),
            output: (Amount::<Pipeable>::new(120, Water),)
        },
        tier: MainProgression(Tier3(CoalPower))
    },
    Process {
        name: "Extract Oil",
        time_s: 1,
        building: OilExtractor {
            input: (Amount::<Pumpable>::new(1, OilSource),),
            output: (Amount::<Pipeable>::new(1, CrudeOil),)
        },
        tier: MainProgression(Tier5(OilProcessing))
    },
    Process {
        name: "Aluminum Ingot",
        time_s: 4,
        building: Foundry {
            input: (Amount::<Conveyable>::new(6, AlScrap), Amount::<Conveyable>::new(5, Silica)),
            output: (Amount::<Conveyable>::new(4, AlIngot),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Copper Ficsmas Ornament",
        time_s: 12,
        building: Foundry {
            input: (Amount::<Conveyable>::new(2, RedOrnament), Amount::<Conveyable>::new(2, CuIngot)),
            output: (Amount::<Conveyable>::new(1, CuOrnament),)
        },
        tier: MAM(Ficsmas(FicsmasGiftTree))
    },
    Process {
        name: "Iron Ficsmas Ornament",
        time_s: 12,
        building: Foundry {
            input: (Amount::<Conveyable>::new(3, BlueOrnament), Amount::<Conveyable>::new(3, FeIngot)),
            output: (Amount::<Conveyable>::new(1, FeOrnament),)
        },
        tier: MAM(Ficsmas(FicsmasGiftTree))
    },
    Process {
        name: "Steel Ingot",
        time_s: 4,
        building: Foundry {
            input: (Amount::<Conveyable>::new(3, FeOre), Amount::<Conveyable>::new(3, Coal)),
            output: (Amount::<Conveyable>::new(3, SteelIngot),)
        },
        tier: MainProgression(Tier3(BasicSteel))
    },
    Process {
        name: "Coke Steel Ingot",
        time_s: 12,
        building: Foundry {
            input: (Amount::<Conveyable>::new(15, FeOre), Amount::<Conveyable>::new(15, PetroleumCoke)),
            output: (Amount::<Conveyable>::new(20, SteelIngot),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Compacted Steel Ingot",
        time_s: 16,
        building: Foundry {
            input: (Amount::<Conveyable>::new(15, FeOre), Amount::<Conveyable>::new(3, CompactedCoal)),
            output: (Amount::<Conveyable>::new(10, SteelIngot),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Compacted Steel Ingot",
        time_s: 16,
        building: Foundry {
            input: (Amount::<Conveyable>::new(15, FeOre), Amount::<Conveyable>::new(3, CompactedCoal)),
            output: (Amount::<Conveyable>::new(10, SteelIngot),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Copper Alloy Ingot",
        time_s: 12,
        building: Foundry {
            input: (Amount::<Conveyable>::new(10, CuOre), Amount::<Conveyable>::new(5, FeOre)),
            output: (Amount::<Conveyable>::new(20, CuIngot),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Iron Alloy Ingot",
        time_s: 6,
        building: Foundry {
            input: (Amount::<Conveyable>::new(2, FeOre), Amount::<Conveyable>::new(2, CuOre)),
            output: (Amount::<Conveyable>::new(5, FeIngot),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Solid Steel Ingot",
        time_s: 3,
        building: Foundry {
            input: (Amount::<Conveyable>::new(2, FeIngot), Amount::<Conveyable>::new(2, Coal)),
            output: (Amount::<Conveyable>::new(3, SteelIngot),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Actual Snow",
        time_s: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FicsmasGift),),
            output: (Amount::<Conveyable>::new(2, ActualSnow),)
        },
        tier: MAM(Ficsmas(AFriend))
    },
    Process {
        name: "Alien DNA Capsule",
        time_s: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, AlienProtein),),
            output: (Amount::<Conveyable>::new(1, AlienDNA),)
        },
        tier: MAM(Organisms(BioOrganicProperties))
    },
    Process {
        name: "Aluminum Casing",
        time_s: 2,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, AlIngot),),
            output: (Amount::<Conveyable>::new(2, AlCasing),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Biomass (Alien Protein)",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, AlienProtein),),
            output: (Amount::<Conveyable>::new(100, Biomass),)
        },
        tier: MAM(Organisms(BioOrganicProperties))
    },
    Process {
        name: "Biomass (Mycelia)",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, Mycelia),),
            output: (Amount::<Conveyable>::new(10, Biomass),)
        },
        tier: MAM(Fungi(MyceliaResearch))
    },
    Process {
        name: "Biomass (Wood)",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(4, WoodOrLeaves),),
            output: (Amount::<Conveyable>::new(20, Biomass),)
        },
        tier: MainProgression(Tier0(HubUpgrade6))
    },
    Process {
        name: "Cable",
        time_s: 2,
        building: Constructor {
            input: (Amount::<Conveyable>::new(2, CuWire),),
            output: (Amount::<Conveyable>::new(1, Cable),)
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Candy Cane",
        time_s: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, FicsmasGift),),
            output: (Amount::<Conveyable>::new(5, CandyCane),)
        },
        tier: MAM(Ficsmas(CandyCaneBasher))
    },
    Process {
        name: "Color Cartridge",
        time_s: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FlowerPetals),),
            output: (Amount::<Conveyable>::new(10, ColorCartridge),)
        },
        tier: MAM(Flowers(ColorCartridges))
    },
    Process {
        name: "Color Cartridge",
        time_s: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FlowerPetals),),
            output: (Amount::<Conveyable>::new(10, ColorCartridge),)
        },
        tier: MainProgression(Tier2(ResourceSinkBonus))
    },
    Process {
        name: "Concrete",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, Limestone),),
            output: (Amount::<Conveyable>::new(1, Concrete),)
        },
        tier: MainProgression(Tier0(HubUpgrade3))
    },
    Process {
        name: "Copper Powder",
        time_s: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(30, CuIngot),),
            output: (Amount::<Conveyable>::new(5, CuPowder),)
        },
        tier: MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name: "Copper Sheet",
        time_s: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(2, CuIngot),),
            output: (Amount::<Conveyable>::new(1, CuSheet),)
        },
        tier: MainProgression(Tier2(PartAssembly))
    },
    Process {
        name: "Empty Canister",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(2, Plastic),),
            output: (Amount::<Conveyable>::new(4, EmptyCanister),)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name: "Empty Fluid Tank",
        time_s: 1,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, AlIngot),),
            output: (Amount::<Conveyable>::new(1, EmptyFluidTank),)
        },
        tier: MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process {
        name: "Ficsmas Bow",
        time_s: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(2, FicsmasGift),),
            output: (Amount::<Conveyable>::new(1, FicsmasBow),)
        },
        tier: MAM(Ficsmas(CandyCaneDecor))
    },
    Process {
        name: "Ficsmas Tree Branch",
        time_s: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, FicsmasGift),),
            output: (Amount::<Conveyable>::new(1, FicsmasBranch),)
        },
        tier: MAM(Ficsmas(TreeUpgrade0))
    },
    Process {
        name: "Iron Plate",
        time_s: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, FeIngot),),
            output: (Amount::<Conveyable>::new(2, FePlate),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Iron Rebar",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, FeRod),),
            output: (Amount::<Conveyable>::new(1, BaseRebar),)
        },
        tier: MAM(Organisms(RebarGun))
    },
    Process {
        name: "Iron Rod",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, FeIngot),),
            output: (Amount::<Conveyable>::new(1, FeRod),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process {
        name: "Power Shard (1)",
        time_s: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, PowerSlugBlue),),
            output: (Amount::<Conveyable>::new(1, PowerShard),)
        },
        tier: MAM(PowerSlugs(BluePowerSlugs))
    },
    Process {
        name: "Power Shard (2)",
        time_s: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, PowerSlugYellow),),
            output: (Amount::<Conveyable>::new(2, PowerShard),)
        },
        tier: MAM(PowerSlugs(YellowPowerShards))
    },
    Process {
        name: "Power Shard (5)",
        time_s: 24,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, PowerSlugPurple),),
            output: (Amount::<Conveyable>::new(5, PowerShard),)
        },
        tier: MAM(PowerSlugs(PurplePowerShards))
    },
    Process {
        name: "Quartz Crystal",
        time_s: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, RawQuartz),),
            output: (Amount::<Conveyable>::new(3, CrushedQuartz),)
        },
        tier: MAM(Quartz(QuartzResearch))
    },
    Process {
        name: "Quickwire",
        time_s: 5,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, CateriumIngot),),
            output: (Amount::<Conveyable>::new(5, Quickwire),)
        },
        tier: MAM(Caterium(QuickwireResearch))
    },
    Process {
        name: "Screw",
        time_s: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, FeRod),),
            output: (Amount::<Conveyable>::new(4, Screws),)
        },
        tier: MainProgression(Tier0(HubUpgrade3))
    },
    Process {
        name: "Silica",
        time_s: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, RawQuartz),),
            output: (Amount::<Conveyable>::new(5, Silica),)
        },
        tier: MAM(Quartz(SilicaResearch))
    },
    Process {
        name: "Silica",
        time_s: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, RawQuartz),),
            output: (Amount::<Conveyable>::new(5, Silica),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Snowball",
        time_s: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, ActualSnow),),
            output: (Amount::<Conveyable>::new(1, Snowball),)
        },
        tier: MAM(Ficsmas(Snowfight))
    },
    Process {
        name: "Solid Biofuel",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(8, Biomass),),
            output: (Amount::<Conveyable>::new(4, SolidBiofuel),)
        },
        tier: MainProgression(Tier2(ObstacleClearing))
    },
    Process {
        name: "Steel Beam",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(4, SteelIngot),),
            output: (Amount::<Conveyable>::new(1, SteelBeam),)
        },
        tier: MainProgression(Tier3(BasicSteel))
    },
    Process {
        name: "Steel Pipe",
        time_s: 6,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, SteelIngot),),
            output: (Amount::<Conveyable>::new(2, SteelPipe),)
        },
        tier: MainProgression(Tier3(BasicSteel))
    },
    Process {
        name: "Wire",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, CuIngot),),
            output: (Amount::<Conveyable>::new(2, CuWire),)
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process {
        name: "Biocoal",
        time_s: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, Biomass),),
            output: (Amount::<Conveyable>::new(6, Coal),)
        },
        tier: HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name: "Biocoal",
        time_s: 8,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, Biomass),),
            output: (Amount::<Conveyable>::new(6, Coal),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Cast Screw",
        time_s: 24,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FeIngot),),
            output: (Amount::<Conveyable>::new(20, Screws),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Caterium Wire",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, CateriumIngot),),
            output: (Amount::<Conveyable>::new(8, CuWire),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumResearch)))
    },
    Process {
        name: "Charcoal",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, WoodOrLeaves),),
            output: (Amount::<Conveyable>::new(10, Coal),)
        },
        tier: HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name: "Charcoal",
        time_s: 4,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, WoodOrLeaves),),
            output: (Amount::<Conveyable>::new(10, Coal),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Iron Wire",
        time_s: 24,
        building: Constructor {
            input: (Amount::<Conveyable>::new(5, FeIngot),),
            output: (Amount::<Conveyable>::new(9, CuWire),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Steel Canister",
        time_s: 3,
        building: Constructor {
            input: (Amount::<Conveyable>::new(3, SteelIngot),),
            output: (Amount::<Conveyable>::new(2, EmptyCanister),)
        },
        tier: HardDrive(MainUnlock(Tier5(AlternativeFluidTransport)))
    },
    Process {
        name: "Steel Rod",
        time_s: 5,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, SteelIngot),),
            output: (Amount::<Conveyable>::new(4, FeRod),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Steel Screw",
        time_s: 12,
        building: Constructor {
            input: (Amount::<Conveyable>::new(1, SteelBeam),),
            output: (Amount::<Conveyable>::new(52, Screws),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "AI Limiter",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, CuSheet), Amount::<Conveyable>::new(20, Quickwire)),
            output: (Amount::<Conveyable>::new(5, AILimiter),)
        },
        tier: MAM(Caterium(AILimiterResearch))
    },
    Process {
        name: "AI Limiter",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, CuSheet), Amount::<Conveyable>::new(20, Quickwire)),
            output: (Amount::<Conveyable>::new(5, AILimiter),)
        },
        tier: MainProgression(Tier7(AeronauticalEngineering))
    },
    Process {
        name: "Alclad Aluminum Sheet",
        time_s: 6,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, AlIngot), Amount::<Conveyable>::new(1, CuIngot)),
            output: (Amount::<Conveyable>::new(3, AlcladSheet),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Assembly Director System",
        time_s: 80,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, AdaptiveControlUnit), Amount::<Conveyable>::new(1, SuperComputer)),
            output: (Amount::<Conveyable>::new(1, AssemblyDirectorSystem),)
        },
        tier: MainProgression(Tier7(AeronauticalEngineering))
    },
    Process {
        name: "Automated Wiring",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, Stator), Amount::<Conveyable>::new(20, Cable)),
            output: (Amount::<Conveyable>::new(1, AutomatedWiring),)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process {
        name: "Black Powder",
        time_s: 4,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, Coal), Amount::<Conveyable>::new(1, Sulfur)),
            output: (Amount::<Conveyable>::new(2, BlackPowder),)
        },
        tier: MAM(SulfurTier(BlackPowderResearch))
    },
    Process {
        name: "Circuit Board",
        time_s: 8,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, CuSheet), Amount::<Conveyable>::new(4, Plastic)),
            output: (Amount::<Conveyable>::new(1, CircuitBoard),)
        },
        tier: MainProgression(Tier5(OilProcessing))
    },
    Process {
        name: "Cluster Nobelisk",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Nobelisk), Amount::<Conveyable>::new(4, SmokelessPowder)),
            output: (Amount::<Conveyable>::new(1, ClusterNobelisk),)
        },
        tier: MAM(SulfurTier(ClusterNobeliskResearch))
    },
    Process {
        name: "Electromagnetic Control Rod",
        time_s: 30,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Stator), Amount::<Conveyable>::new(2, AILimiter)),
            output: (Amount::<Conveyable>::new(2, EMControlRod),)
        },
        tier: MainProgression(Tier8(NuclearPower))
    },
    Process {
        name: "Encased Industrial Beam",
        time_s: 10,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, SteelBeam), Amount::<Conveyable>::new(5, Concrete)),
            output: (Amount::<Conveyable>::new(1, IndustrialBeam),)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process {
        name: "Encased Plutonium Cell",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, PuPellet), Amount::<Conveyable>::new(4, Concrete)),
            output: (Amount::<Conveyable>::new(1, EncasedPuCell),)
        },
        tier: MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name: "Ficsmas Decoration",
        time_s: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(15, FicsmasBranch), Amount::<Conveyable>::new(6, OrnamentBundle)),
            output: (Amount::<Conveyable>::new(2, FicsmasDecoration),)
        },
        tier: MAM(Ficsmas(ItsSnowing))
    },
    Process {
        name: "Ficsmas Wonder Star",
        time_s: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, FicsmasDecoration), Amount::<Conveyable>::new(20, CandyCane)),
            output: (Amount::<Conveyable>::new(1, FicsmasStar),)
        },
        tier: MAM(Ficsmas(Wreath))
    },
    Process {
        name: "Ficsmas Ornament Bundle",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, CuOrnament), Amount::<Conveyable>::new(1, FeOrnament)),
            output: (Amount::<Conveyable>::new(1, OrnamentBundle),)
        },
        tier: MAM(Ficsmas(Lights))
    },
    Process {
        name: "Fabric",
        time_s: 4,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, Mycelia), Amount::<Conveyable>::new(5, Biomass)),
            output: (Amount::<Conveyable>::new(1, Fabric),)
        },
        tier: MAM(Fungi(FabricResearch))
    },
    Process {
        name: "Fancy Fireworks",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, FicsmasBranch), Amount::<Conveyable>::new(3, FicsmasBow)),
            output: (Amount::<Conveyable>::new(1, FancyFireworks),)
        },
        tier: MAM(Ficsmas(Fireworks))
    },
    Process {
        name: "Gas Nobelisk",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, Nobelisk), Amount::<Conveyable>::new(10, Biomass)),
            output: (Amount::<Conveyable>::new(1, GasNobelisk),)
        },
        tier: MAM(Fungi(ToxicCellularModification))
    },
    Process {
        name: "Heat Sink",
        time_s: 8,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, AlcladSheet), Amount::<Conveyable>::new(3, CuSheet)),
            output: (Amount::<Conveyable>::new(1, Heatsink),)
        },
        tier: MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process {
        name: "Homing Rifle Ammo",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(20, RifleAmmo), Amount::<Conveyable>::new(1, HighSpeedConnector)),
            output: (Amount::<Conveyable>::new(10, HomingRifleAmmo),)
        },
        tier: MAM(Caterium(BulletGuidanceSystem))
    },
    Process {
        name: "Modular Frame",
        time_s: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, ReinforcedIronPlate), Amount::<Conveyable>::new(12, FeRod)),
            output: (Amount::<Conveyable>::new(2, ModularFrame),)
        },
        tier: MainProgression(Tier2(PartAssembly))
    },
    Process {
        name: "Motor",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, Rotor), Amount::<Conveyable>::new(2, Stator)),
            output: (Amount::<Conveyable>::new(1, Motor),)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process {
        name: "Nobelisk",
        time_s: 6,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, BlackPowder), Amount::<Conveyable>::new(2, SteelPipe)),
            output: (Amount::<Conveyable>::new(1, Nobelisk),)
        },
        tier: MAM(SulfurTier(NobeliskDetonator))
    },
    Process {
        name: "Pressure Conversion Cube",
        time_s: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, FusedModularFrame), Amount::<Conveyable>::new(2, RadioControlUnit)),
            output: (Amount::<Conveyable>::new(1, PressureConversionCube),)
        },
        tier: MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name: "Pulse Nobelisk",
        time_s: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, Nobelisk), Amount::<Conveyable>::new(1, CrystalOscillator)),
            output: (Amount::<Conveyable>::new(5, PulseNobelisk),)
        },
        tier: MAM(Quartz(ExplosiveResonanceApplication))
    },
    Process {
        name: "Reinforced Iron Plate",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(6, FePlate), Amount::<Conveyable>::new(12, Screws)),
            output: (Amount::<Conveyable>::new(1, ReinforcedIronPlate),)
        },
        tier: MainProgression(Tier0(HubUpgrade3))
    },
    Process {
        name: "Rifle Ammo",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, CuSheet), Amount::<Conveyable>::new(2, SmokelessPowder)),
            output: (Amount::<Conveyable>::new(15, RifleAmmo),)
        },
        tier: MAM(SulfurTier(Rifle))
    },
    Process {
        name: "Rotor",
        time_s: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, FeRod), Amount::<Conveyable>::new(25, Screws)),
            output: (Amount::<Conveyable>::new(1, Rotor),)
        },
        tier: MainProgression(Tier2(PartAssembly))
    },
    Process {
        name: "Shatter Rebar",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, BaseRebar), Amount::<Conveyable>::new(3, CrushedQuartz)),
            output: (Amount::<Conveyable>::new(1, ShatterRebar),)
        },
        tier: MAM(Quartz(ShatterRebarResearch))
    },
    Process {
        name: "Smart Plating",
        time_s: 30,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, ReinforcedIronPlate), Amount::<Conveyable>::new(1, Rotor)),
            output: (Amount::<Conveyable>::new(1, SmartPlating),)
        },
        tier: MainProgression(Tier2(PartAssembly))
    },
    Process {
        name: "Sparkly Fireworks",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, FicsmasBranch), Amount::<Conveyable>::new(2, ActualSnow)),
            output: (Amount::<Conveyable>::new(1, SparklyFireworks),)
        },
        tier: MAM(Ficsmas(Fireworks))
    },
    Process {
        name: "Stator",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, SteelPipe), Amount::<Conveyable>::new(8, CuWire)),
            output: (Amount::<Conveyable>::new(1, Stator),)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process {
        name: "Stun Rebar",
        time_s: 6,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, BaseRebar), Amount::<Conveyable>::new(5, Quickwire)),
            output: (Amount::<Conveyable>::new(1, PulseRebar),)
        },
        tier: MAM(Caterium(StunRebarResearch))
    },
    Process {
        name: "Sweet Fireworks",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(6, FicsmasBranch), Amount::<Conveyable>::new(3, CandyCane)),
            output: (Amount::<Conveyable>::new(1, SweetFireworks),)
        },
        tier: MAM(Ficsmas(Fireworks))
    },
    Process {
        name: "Versatile Framework",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, ModularFrame), Amount::<Conveyable>::new(12, SteelBeam)),
            output: (Amount::<Conveyable>::new(2, VersatileFramework),)
        },
        tier: MainProgression(Tier3(BasicSteel))
    },
    Process {
        name: "Adhered Iron Plate",
        time_s: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, FePlate), Amount::<Conveyable>::new(1, Rubber)),
            output: (Amount::<Conveyable>::new(1, ReinforcedIronPlate),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Alclad Casing",
        time_s: 8,
        building: Assembler {
            input: (Amount::<Conveyable>::new(20, AlIngot), Amount::<Conveyable>::new(10, CuIngot)),
            output: (Amount::<Conveyable>::new(15, AlCasing),)
        },
        tier: HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process {
        name: "Bolted Frame",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, ReinforcedIronPlate), Amount::<Conveyable>::new(56, Screws)),
            output: (Amount::<Conveyable>::new(2, ModularFrame),)
        },
        tier: HardDrive(MainUnlock(Tier2(PartAssembly)))
    },
    Process {
        name: "Bolted Iron Plate",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(18, FePlate), Amount::<Conveyable>::new(50, Screws)),
            output: (Amount::<Conveyable>::new(3, ReinforcedIronPlate),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Caterium Circuit Board",
        time_s: 48,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, Plastic), Amount::<Conveyable>::new(30, Quickwire)),
            output: (Amount::<Conveyable>::new(7, CircuitBoard),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Caterium Circuit Board",
        time_s: 48,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, Plastic), Amount::<Conveyable>::new(30, Quickwire)),
            output: (Amount::<Conveyable>::new(7, CircuitBoard),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process {
        name: "Cheap Silica",
        time_s: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, RawQuartz), Amount::<Conveyable>::new(5, Limestone)),
            output: (Amount::<Conveyable>::new(7, Silica),)
        },
        tier: HardDrive(MAMUnlock(Quartz(SilicaResearch)))
    },
    Process {
        name: "Coated Iron Canister",
        time_s: 4,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, FePlate), Amount::<Conveyable>::new(1, CuSheet)),
            output: (Amount::<Conveyable>::new(4, EmptyCanister),)
        },
        tier: HardDrive(MainUnlock(Tier5(AlternativeFluidTransport)))
    },
    Process {
        name: "Coated Iron Plate",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, FeIngot), Amount::<Conveyable>::new(2, Plastic)),
            output: (Amount::<Conveyable>::new(15, FePlate),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Compacted Coal",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(5, Coal), Amount::<Conveyable>::new(5, Sulfur)),
            output: (Amount::<Conveyable>::new(5, CompactedCoal),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Copper Rotor",
        time_s: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(6, CuSheet), Amount::<Conveyable>::new(52, Screws)),
            output: (Amount::<Conveyable>::new(3, Rotor),)
        },
        tier: HardDrive(MainUnlock(Tier2(PartAssembly)))
    },
    Process {
        name: "Crystal Computer",
        time_s: 64,
        building: Assembler {
            input: (Amount::<Conveyable>::new(8, CircuitBoard), Amount::<Conveyable>::new(3, CrystalOscillator)),
            output: (Amount::<Conveyable>::new(3, Computer),)
        },
        tier: HardDrive(MainUnlock(Tier5(IndustrialManufacturing)))
    },
    Process {
        name: "Crystal Computer",
        time_s: 64,
        building: Assembler {
            input: (Amount::<Conveyable>::new(8, CircuitBoard), Amount::<Conveyable>::new(3, CrystalOscillator)),
            output: (Amount::<Conveyable>::new(3, Computer),)
        },
        tier: HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name: "Electric Motor",
        time_s: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, EMControlRod), Amount::<Conveyable>::new(2, Rotor)),
            output: (Amount::<Conveyable>::new(2, Motor),)
        },
        tier: HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process {
        name: "Electrode Circuit Board",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(6, Rubber), Amount::<Conveyable>::new(9, PetroleumCoke)),
            output: (Amount::<Conveyable>::new(1, CircuitBoard),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Electromagnetic Connection Rod",
        time_s: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, Stator), Amount::<Conveyable>::new(1, HighSpeedConnector)),
            output: (Amount::<Conveyable>::new(2, EMControlRod),)
        },
        tier: HardDrive(MainUnlock(Tier8(NuclearPower)))
    },
    Process {
        name: "Electromagnetic Connection Rod",
        time_s: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, Stator), Amount::<Conveyable>::new(1, HighSpeedConnector)),
            output: (Amount::<Conveyable>::new(2, EMControlRod),)
        },
        tier: HardDrive(MAMUnlock(Caterium(AILimiterResearch)))
    },
    Process {
        name: "Encased Industrial Pipe",
        time_s: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(7, SteelPipe), Amount::<Conveyable>::new(5, Concrete)),
            output: (Amount::<Conveyable>::new(1, IndustrialBeam),)
        },
        tier: HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process {
        name: "Fine Black Powder",
        time_s: 16,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, Sulfur), Amount::<Conveyable>::new(1, CompactedCoal)),
            output: (Amount::<Conveyable>::new(4, BlackPowder),)
        },
        tier: HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name: "Fine Concrete",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Silica), Amount::<Conveyable>::new(12, Limestone)),
            output: (Amount::<Conveyable>::new(10, Concrete),)
        },
        tier: HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name: "Fused Quickwire",
        time_s: 8,
        building: Assembler {
            input: (Amount::<Conveyable>::new(1, CateriumIngot), Amount::<Conveyable>::new(5, CuIngot)),
            output: (Amount::<Conveyable>::new(12, Quickwire),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process {
        name: "Fused Wire",
        time_s: 20,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, CuIngot), Amount::<Conveyable>::new(1, CateriumIngot)),
            output: (Amount::<Conveyable>::new(30, CuWire),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumResearch)))
    },
    Process {
        name: "Heat Exchanger",
        time_s: 6,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, AlCasing), Amount::<Conveyable>::new(3, Rubber)),
            output: (Amount::<Conveyable>::new(1, Heatsink),)
        },
        tier: HardDrive(MainUnlock(Tier8(AdvancedAluminumProduction)))
    },
    Process {
        name: "Insulated Cable",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(9, CuWire), Amount::<Conveyable>::new(6, Rubber)),
            output: (Amount::<Conveyable>::new(20, Cable),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "OC Supercomputer",
        time_s: 20,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, RadioControlUnit), Amount::<Conveyable>::new(3, CoolingSystem)),
            output: (Amount::<Conveyable>::new(1, SuperComputer),)
        },
        tier: HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process {
        name: "OC Supercomputer",
        time_s: 20,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, RadioControlUnit), Amount::<Conveyable>::new(3, CoolingSystem)),
            output: (Amount::<Conveyable>::new(1, SuperComputer),)
        },
        tier: HardDrive(MainUnlock(Tier8(AdvancedAluminumProduction)))
    },
    Process {
        name: "Plutonium Fuel Unit",
        time_s: 120,
        building: Assembler {
            input: (Amount::<Conveyable>::new(20, EncasedPuCell), Amount::<Conveyable>::new(1, PressureConversionCube)),
            output: (Amount::<Conveyable>::new(1, PuRod),)
        },
        tier: HardDrive(MainUnlock(Tier8(ParticleEnrichment)))
    },
    Process {
        name: "Quickwire Cable",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Quickwire),
                    Amount::<Conveyable>::new(2, Rubber)),
            output: (Amount::<Conveyable>::new(11, Cable),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Quickwire Cable",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(3, Quickwire),
                    Amount::<Conveyable>::new(2, Rubber)),
            output: (Amount::<Conveyable>::new(11, Cable),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process {
        name: "Quickwire Stator",
        time_s: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, SteelPipe),
                    Amount::<Conveyable>::new(15, Quickwire)),
            output: (Amount::<Conveyable>::new(2, Stator),)
        },
        tier: HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process {
        name: "Quickwire Stator",
        time_s: 15,
        building: Assembler {
            input: (Amount::<Conveyable>::new(4, SteelPipe),
                    Amount::<Conveyable>::new(15, Quickwire)),
            output: (Amount::<Conveyable>::new(2, Stator),)
        },
        tier: HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process {
        name: "Rubber Concrete",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, Limestone),
                    Amount::<Conveyable>::new(2, Rubber)),
            output: (Amount::<Conveyable>::new(9, Concrete),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Silicon Circuit Board",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(11, CuSheet),
                    Amount::<Conveyable>::new(11, Silica)),
            output: (Amount::<Conveyable>::new(5, CircuitBoard),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Silicon Circuit Board",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(11, CuSheet),
                    Amount::<Conveyable>::new(11, Silica)),
            output: (Amount::<Conveyable>::new(5, CircuitBoard),)
        },
        tier: HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name: "Steel Coated Plate",
        time_s: 24,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, SteelIngot),
                    Amount::<Conveyable>::new(2, Plastic)),
            output: (Amount::<Conveyable>::new(18, FePlate),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Steel Rotor",
        time_s: 12,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, SteelPipe),
                    Amount::<Conveyable>::new(6, CuWire)),
            output: (Amount::<Conveyable>::new(1, Rotor),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Steeled Frame",
        time_s: 60,
        building: Assembler {
            input: (Amount::<Conveyable>::new(2, ReinforcedIronPlate),
                    Amount::<Conveyable>::new(10, SteelPipe)),
            output: (Amount::<Conveyable>::new(3, ModularFrame),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process {
        name: "Stitched Iron Plate",
        time_s: 32,
        building: Assembler {
            input: (Amount::<Conveyable>::new(10, FePlate), Amount::<Conveyable>::new(20, CuWire)),
            output: (Amount::<Conveyable>::new(3, ReinforcedIronPlate),)
        },
        tier: HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process {
        name: "Adaptive Control Unit",
        time_s: 120,
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
        time_s: 8,
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
        time_s: 24,
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
        time_s: 120,
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
        time_s: 120,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(36, CrushedQuartz),
                    Amount::<Conveyable>::new(28, Cable, ),
                    Amount::<Conveyable>::new(5, ReinforcedIronPlate, ),
                    None),
            output: (Amount::<Conveyable>::new(2, CrystalOscillator, ),)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Explosive Rebar",
        time_s: 12,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(2, BaseRebar),
                    Amount::<Conveyable>::new(2, SmokelessPowder),
                    Amount::<Conveyable>::new(2, SteelPipe),
                    None),
            output: (Amount::<Conveyable>::new( 1, ExplosiveRebar),)
        },
        tier:MAM(SulfurTier(ExplosiveRebarResearch))
    },
    Process {
        name: "Gas Filter",
        time_s: 8,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, Coal),
                    Amount::<Conveyable>::new(2, Rubber),
                    Amount::<Conveyable>::new(2, Fabric),
                    None),
            output: (Amount::<Conveyable>::new( 1, GasFilter),)
        },
        tier:MainProgression(Tier5(GasMask))
    },
    Process {
        name: "Heavy Modular Frame",
        time_s: 30,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, ModularFrame),
                    Amount::<Conveyable>::new(15, SteelPipe),
                    Amount::<Conveyable>::new(5, IndustrialBeam),
                    Some(Amount::<Conveyable>::new(100, Screws))),
            output: (Amount::<Conveyable>::new(1, HeavyModularFrame),)
        },
        tier:MainProgression(Tier4(AdvancedSteel))
    },
    Process {
        name: "High-Speed Connector",
        time_s: 16,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(56, Quickwire),
                    Amount::<Conveyable>::new(10, Cable),
                    Amount::<Conveyable>::new(1, CircuitBoard),
                    None),
            output: (Amount::<Conveyable>::new(1, HighSpeedConnector ),)
        },
        tier:MainProgression(Tier7(AeronauticalEngineering))
    },
    Process {
        name: "High-Speed Connector",
        time_s: 16,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(56, Quickwire),
                    Amount::<Conveyable>::new(10, Cable),
                    Amount::<Conveyable>::new(1, CircuitBoard),
                    None),
            output: (Amount::<Conveyable>::new(1, HighSpeedConnector ),)
        },
        tier:MAM(Caterium(HighSpeedConnectorResearch))
    },
    Process {
        name: "Iodine Infused Filter",
        time_s: 16,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(1, GasFilter),
                    Amount::<Conveyable>::new(8, Quickwire),
                    Amount::<Conveyable>::new(1, AlCasing),
                    None),
            output: (Amount::<Conveyable>::new(1, IodineFilter ),)
        },
        tier:MainProgression(Tier7(HazmatSuit))
    },
    Process {
        name: "Magnetic Field Generator",
        time_s: 120,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, VersatileFramework),
                    Amount::<Conveyable>::new(2, EMControlRod),
                    Amount::<Conveyable>::new(10, Battery),
                    None),
            output: (Amount::<Conveyable>::new(2, MagneticFieldGenerator ),)
        },
        tier:MainProgression(Tier8(NuclearPower))
    },
    Process {
        name: "Modular Engine",
        time_s: 60,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(2, Motor),
                    Amount::<Conveyable>::new(2, SmartPlating),
                    Amount::<Conveyable>::new(15, Rubber),
                    None),
            output: (Amount::<Conveyable>::new(1, ModularEngine ),)
        },
        tier:MainProgression(Tier5(IndustrialManufacturing))
    },
    Process {
        name: "Nuke Nobelisk",
        time_s: 120,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, Nobelisk),
                    Amount::<Conveyable>::new(20, EncasedUCell),
                    Amount::<Conveyable>::new(10, SmokelessPowder),
                    Some(Amount::<Conveyable>::new(6, AILimiter))),
            output: (Amount::<Conveyable>::new(1, NuclearNobelisk ),)
        },
        tier:MAM(SulfurTier(NuclearDeterrent))
    },
    Process {
        name: "Plutonium Fuel Rod",
        time_s: 240,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(30, EncasedPuCell),
                    Amount::<Conveyable>::new(18, SteelBeam),
                    Amount::<Conveyable>::new(6, EMControlRod),
                    Some(Amount::<Conveyable>::new(10, Heatsink))),
            output: (Amount::<Conveyable>::new(1, PuRod ),)
        },
        tier:MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name: "Radio Control Unit",
        time_s: 48,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(32, AlCasing),
                    Amount::<Conveyable>::new(1, CrystalOscillator),
                    Amount::<Conveyable>::new(1, Computer),
                    None),
            output: (Amount::<Conveyable>::new(2, RadioControlUnit ),)
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name: "Supercomputer",
        time_s: 32,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(2, Computer),
                    Amount::<Conveyable>::new(2, AILimiter),
                    Amount::<Conveyable>::new(3, HighSpeedConnector),
                    Some(Amount::<Conveyable>::new(28, Plastic))),
            output: (Amount::<Conveyable>::new(1, SuperComputer),)
        },
        tier:MAM(Caterium(SupercomputerResearch))
    },
    Process {
        name: "Supercomputer",
        time_s: 32,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(2, Computer),
                    Amount::<Conveyable>::new(2, AILimiter),
                    Amount::<Conveyable>::new(3, HighSpeedConnector),
                    Some(Amount::<Conveyable>::new(28, Plastic))),
            output: (Amount::<Conveyable>::new(1, SuperComputer),)
        },
        tier:MainProgression(Tier7(AeronauticalEngineering))
    },
    Process {
        name: "Thermal Propulsion Rocket",
        time_s: 120,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, ModularEngine),
                    Amount::<Conveyable>::new(2, TurboMotor),
                    Amount::<Conveyable>::new(6, CoolingSystem),
                    Some(Amount::<Conveyable>::new(2, FusedModularFrame))),
            output: (Amount::<Conveyable>::new(2, ThermalPropulsionRocket ),)
        },
        tier:MainProgression(Tier8(LeadingEdgeProduction))
    },
    Process {
        name: "Turbo Motor",
        time_s: 32,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(4, CoolingSystem),
                    Amount::<Conveyable>::new(2, RadioControlUnit),
                    Amount::<Conveyable>::new(4, Motor),
                    Some(Amount::<Conveyable>::new(24, Rubber))),
            output: (Amount::<Conveyable>::new(1, TurboMotor ),)
        },
        tier:MainProgression(Tier8(LeadingEdgeProduction))
    },
    Process {
        name: "Turbo Rifle Ammo",
        time_s: 12,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(25, RifleAmmo),
                    Amount::<Conveyable>::new(3, AlCasing),
                    Amount::<Conveyable>::new(3, PkgdTurbofuel),
                    None),
            output: (Amount::<Conveyable>::new(50, TurboRifleAmmo ),)
        },
        tier:MAM(SulfurTier(TurboRife))
    },
    Process {
        name: "Uranium Fuel Rod",
        time_s: 150,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(50, EncasedUCell),
                    Amount::<Conveyable>::new(3,IndustrialBeam),
                    Amount::<Conveyable>::new(5,EMControlRod),
                    None),
            output: (Amount::<Conveyable>::new(1, URod),)
        },
        tier:MainProgression(Tier8(NuclearPower))
    },
    Process {
        name: "Automated Miner",
        time_s: 60,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(1, Motor),
                    Amount::<Conveyable>::new(4, SteelPipe),
                    Amount::<Conveyable>::new(4, FeRod),
                    Some(Amount::<Conveyable>::new(2, FePlate))),
            output: (Amount::<Conveyable>::new(1 , PortableMiner),)
        },
        tier:HardDrive(MainUnlock(Tier5(IndustrialManufacturing)))
    },
    Process {
        name: "Automated Speed Wiring",
        time_s: 32,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(2, Stator),
                    Amount::<Conveyable>::new(40, CuWire),
                    Amount::<Conveyable>::new(1, HighSpeedConnector),
                    None),
            output: (Amount::<Conveyable>::new(4, AutomatedWiring ),)
        },
        tier:HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process {
        name: "Automated Speed Wiring",
        time_s: 32,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(2, Stator),
                    Amount::<Conveyable>::new(40, CuWire),
                    Amount::<Conveyable>::new(1, HighSpeedConnector),
                    None),
            output: (Amount::<Conveyable>::new(4, AutomatedWiring ),)
        },
        tier:HardDrive(MAMUnlock(Caterium(AILimiterResearch)))
    },
    Process {
        name: "Caterium Computer",
        time_s: 16,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(7, CircuitBoard),
                    Amount::<Conveyable>::new(28, Quickwire),
                    Amount::<Conveyable>::new(12, Rubber),
                    None),
            output: (Amount::<Conveyable>::new(1, Computer),)
        },
        tier:HardDrive(MainUnlock(Tier5(IndustrialManufacturing)))
    },
    Process {
        name: "Caterium Computer",
        time_s: 16,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(7, CircuitBoard),
                    Amount::<Conveyable>::new(28, Quickwire),
                    Amount::<Conveyable>::new(12, Rubber),
                    None),
            output: (Amount::<Conveyable>::new(1, Computer),)
        },
        tier:HardDrive(MAMUnlock(Caterium(AILimiterResearch)))
    },
    Process {
        name: "Classic Battery",
        time_s: 8,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(6, Sulfur),
                    Amount::<Conveyable>::new(7, AlcladSheet),
                    Amount::<Conveyable>::new(8, Plastic),
                    Some(Amount::<Conveyable>::new(12, CuWire))),
            output: (Amount::<Conveyable>::new(4, Battery ),)
        },
        tier:HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process {
        name: "Flexible Framework",
        time_s: 120,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(1, ModularFrame),
                    Amount::<Conveyable>::new(6, SteelBeam),
                    Amount::<Conveyable>::new(8, Rubber),
                    None),
            output: (Amount::<Conveyable>::new( 2, VersatileFramework),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Heavy Encased Frame",
        time_s: 64,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(8, ModularFrame),
                    Amount::<Conveyable>::new(10, IndustrialBeam),
                    Amount::<Conveyable>::new(36, SteelPipe),
                    Some(Amount::<Conveyable>::new(22, Concrete))),
            output: (Amount::<Conveyable>::new(3, HeavyModularFrame ),)
        },
        tier:HardDrive(MainUnlock(Tier5(IndustrialManufacturing)))
    },
    Process {
        name: "Heavy Flexible Frame",
        time_s: 16,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, ModularFrame),
                    Amount::<Conveyable>::new(3, IndustrialBeam),
                    Amount::<Conveyable>::new(20, Rubber),
                    Some(Amount::<Conveyable>::new(104, Screws))),
            output: (Amount::<Conveyable>::new(1, HeavyModularFrame ),)
        },
        tier:HardDrive(MainUnlock(Tier5(IndustrialManufacturing)))
    },
    Process {
        name: "Infused Uranium Cell",
        time_s: 12,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, UOre),
                    Amount::<Conveyable>::new(3, Silica),
                    Amount::<Conveyable>::new(5, Sulfur),
                    Some(Amount::<Conveyable>::new(15,Quickwire))),
            output: (Amount::<Conveyable>::new(4, EncasedUCell ),)
        },
        tier:HardDrive(MainUnlock(Tier8(NuclearPower)))
    },
    Process {
        name: "Infused Uranium Cell",
        time_s: 12,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, UOre),
                    Amount::<Conveyable>::new(3, Silica),
                    Amount::<Conveyable>::new(5, Sulfur),
                    Some(Amount::<Conveyable>::new(15,Quickwire))),
            output: (Amount::<Conveyable>::new(4, EncasedUCell ),)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process {
        name: "Infused Uranium Cell",
        time_s: 12,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, UOre),
                    Amount::<Conveyable>::new(3, Silica),
                    Amount::<Conveyable>::new(5, Sulfur),
                    Some(Amount::<Conveyable>::new(15,Quickwire))),
            output: (Amount::<Conveyable>::new(4, EncasedUCell ),)
        },
        tier:HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name: "Infused Uranium Cell",
        time_s: 12,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(5, UOre),
                    Amount::<Conveyable>::new(3, Silica),
                    Amount::<Conveyable>::new(5, Sulfur),
                    Some(Amount::<Conveyable>::new(15,Quickwire))),
            output: (Amount::<Conveyable>::new(4, EncasedUCell ),)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(SulfurResearch)))
    },
    Process {
        name: "Insulated Crystal Oscillator",
        time_s: 32,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(10, CrushedQuartz),
                    Amount::<Conveyable>::new(7, Rubber),
                    Amount::<Conveyable>::new(1, AILimiter),
                    None),
            output: (Amount::<Conveyable>::new(1, CrystalOscillator),)
        },
        tier:HardDrive(MAMUnlock(Quartz(CrystalOscillatorResearch)))
    },
    Process {
        name: "Insulated Crystal Oscillator",
        time_s: 32,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(10, CrushedQuartz),
                    Amount::<Conveyable>::new(7, Rubber),
                    Amount::<Conveyable>::new(1, AILimiter),
                    None),
            output: (Amount::<Conveyable>::new(1, CrystalOscillator),)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumElectronics)))
    },
    Process {
        name: "Insulated Crystal Oscillator",
        time_s: 32,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(10, CrushedQuartz),
                    Amount::<Conveyable>::new(7, Rubber),
                    Amount::<Conveyable>::new(1, AILimiter),
                    None),
            output: (Amount::<Conveyable>::new(1, CrystalOscillator),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name: "Plastic Smart Plating",
        time_s: 24,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(1, ReinforcedIronPlate),
                    Amount::<Conveyable>::new(1, Rotor),
                    Amount::<Conveyable>::new(3, Plastic),
                    None),
            output: (Amount::<Conveyable>::new(2, SmartPlating),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name:"Radio Connection Unit",
        time_s: 16,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(4, Heatsink),
                    Amount::<Conveyable>::new(2, HighSpeedConnector),
                    Amount::<Conveyable>::new(12, CrushedQuartz),
                    None),
            output: (Amount::<Conveyable>::new(1, RadioControlUnit),)
        },
        tier: HardDrive(MainUnlock(Tier8(AdvancedAluminumProduction)))
    },
    Process {
        name:"Radio Connection Unit",
        time_s: 16,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(4, Heatsink),
                    Amount::<Conveyable>::new(2, HighSpeedConnector),
                    Amount::<Conveyable>::new(12, CrushedQuartz),
                    None),
            output: (Amount::<Conveyable>::new(1, RadioControlUnit),)
        },
        tier: HardDrive(MAMUnlock(Caterium(AILimiterResearch)))
    },
    Process {
        name:"Radio Control System",
        time_s:40,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(1, CrystalOscillator),
                    Amount::<Conveyable>::new(10, CircuitBoard),
                    Amount::<Conveyable>::new(60, AlCasing),
                    Some(Amount::<Conveyable>::new(30, Rubber))),
            output: (Amount::<Conveyable>::new(3, RadioControlUnit),)
        },
        tier: HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process {
        name:"Rigour Motor",
        time_s:48,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(3, Rotor),
                    Amount::<Conveyable>::new(3, Stator),
                    Amount::<Conveyable>::new(1, CrystalOscillator),
                    None),
            output: (Amount::<Conveyable>::new(6, Motor),)
        },
        tier: HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process {
        name:"Rigour Motor",
        time_s:48,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(3, Rotor),
                    Amount::<Conveyable>::new(3, Stator),
                    Amount::<Conveyable>::new(1, CrystalOscillator),
                    None),
            output: (Amount::<Conveyable>::new(6, Motor),)
        },
        tier: HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name:"Silicon High-Speed Connector",
        time_s:40,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(60, Quickwire),
                    Amount::<Conveyable>::new(25, Silica),
                    Amount::<Conveyable>::new(2, CircuitBoard),
                    None),
            output: (Amount::<Conveyable>::new(2, HighSpeedConnector),)
        },
        tier: HardDrive(MAMUnlock(Caterium(HighSpeedConnectorResearch)))
    },
    Process {
        name:"Silicon High-Speed Connector",
        time_s:40,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(60, Quickwire),
                    Amount::<Conveyable>::new(25, Silica),
                    Amount::<Conveyable>::new(2, CircuitBoard),
                    None),
            output: (Amount::<Conveyable>::new(2, HighSpeedConnector),)
        },
        tier: HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name:"Silicon High-Speed Connector",
        time_s:40,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(60, Quickwire),
                    Amount::<Conveyable>::new(25, Silica),
                    Amount::<Conveyable>::new(2, CircuitBoard),
                    None),
            output: (Amount::<Conveyable>::new(2, HighSpeedConnector),)
        },
        tier: HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name:"Super-State Computer",
        time_s:50,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(3,Computer),
                    Amount::<Conveyable>::new(2, EMControlRod),
                    Amount::<Conveyable>::new(20, Battery),
                    Some(Amount::<Conveyable>::new(45, CuWire))),
            output: (Amount::<Conveyable>::new(2, SuperComputer),)
        },
        tier: HardDrive(MainUnlock(Tier8(NuclearPower)))
    },
    Process {
        name:"Super-State Computer",
        time_s:50,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(3,Computer),
                    Amount::<Conveyable>::new(2, EMControlRod),
                    Amount::<Conveyable>::new(20, Battery),
                    Some(Amount::<Conveyable>::new(45, CuWire))),
            output: (Amount::<Conveyable>::new(2, SuperComputer),)
        },
        tier: HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process {
        name:"Turbo Electric Motor",
        time_s:64,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(7, Motor),
                    Amount::<Conveyable>::new(9, RadioControlUnit),
                    Amount::<Conveyable>::new(5, EMControlRod),
                    Some(Amount::<Conveyable>::new(7, Rotor))),
            output: (Amount::<Conveyable>::new(3, TurboMotor),)
        },
        tier: HardDrive(MainUnlock(Tier8(LeadingEdgeProduction)))
    },
    Process {
        name:"Turbo Electric Motor",
        time_s:64,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(7, Motor),
                    Amount::<Conveyable>::new(9, RadioControlUnit),
                    Amount::<Conveyable>::new(5, EMControlRod),
                    Some(Amount::<Conveyable>::new(7, Rotor))),
            output: (Amount::<Conveyable>::new(3, TurboMotor),)
        },
        tier: HardDrive(MainUnlock(Tier8(NuclearPower)))
    },
    Process {
        name:"Turbo Pressure Motor",
        time_s:32,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(4, Motor),
                    Amount::<Conveyable>::new(1, PressureConversionCube),
                    Amount::<Conveyable>::new(24, PkgdN),
                    Some(Amount::<Conveyable>::new(8, Stator))),
            output: (Amount::<Conveyable>::new(2, TurboMotor),)
        },
        tier: HardDrive(MainUnlock(Tier8(ParticleEnrichment)))
    },
    Process {
        name:"Uranium Fuel Unit",
        time_s:300,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(100, EncasedUCell),
                    Amount::<Conveyable>::new(10, EMControlRod),
                    Amount::<Conveyable>::new(3, CrystalOscillator),
                    Some(Amount::<Conveyable>::new(6, Beacon))),
            output: (Amount::<Conveyable>::new(3, URod),)
        },
        tier: HardDrive(MainUnlock(Tier8(NuclearPower)))
    },
    Process {
        name:"Uranium Fuel Unit",
        time_s:300,
        building: Manufacturer {
            input: (Amount::<Conveyable>::new(100, EncasedUCell),
                    Amount::<Conveyable>::new(10, EMControlRod),
                    Amount::<Conveyable>::new(3, CrystalOscillator),
                    Some(Amount::<Conveyable>::new(6, Beacon))),
            output: (Amount::<Conveyable>::new(3, URod),)
        },
        tier: HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name:"Packaged Alumina Solution",
        time_s:1,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, AlSol))),
            output: (Amount::<Conveyable>::new(2, PkgdAlSol),
                    None)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name:"Packaged Fuel",
        time_s:3,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Fuel))),
            output: (Amount::<Conveyable>::new(2, PkgdFuel),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Packaged Heavy Oil Residue",
        time_s:4,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, HeavyOil))),
            output: (Amount::<Conveyable>::new(2, PkgdHOil),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Packaged Liquid Biofuel",
        time_s:3,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, LBioFuel))),
            output: (Amount::<Conveyable>::new(2, PkgdLBiofuel),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Packaged Nitric Acid",
        time_s:2,
        building: Packager {
            input: (Amount::<Conveyable>::new(1, EmptyFluidTank),
                    Some(Amount::<Pipeable>::new(1, NAcid))),
            output: (Amount::<Conveyable>::new(1,PkgdNAcid),
                     None)
        },
        tier: MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name:"Packaged Nitrogen Gas",
        time_s:1,
        building: Packager {
            input: (Amount::<Conveyable>::new(1, EmptyFluidTank),
                    Some(Amount::<Pipeable>::new(4, NGas))),
            output: (Amount::<Conveyable>::new(1, PkgdN),
                     None)
        },
        tier: MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process {
        name:"Packaged Oil",
        time_s:4,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, CrudeOil))),
            output: (Amount::<Conveyable>::new(2, PkgdOil),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Packaged Sulfuric Acid",
        time_s:3,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, SAcid))),
            output: (Amount::<Conveyable>::new(2, PkgdSAcid),
                     None)
        },
        tier: MainProgression(Tier7(AeronauticalEngineering))
    },
    Process {
        name:"Packaged Water",
        time_s:2,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Water))),
            output: (Amount::<Conveyable>::new(2, PkgdWater),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Packaged Turbofuel",
        time_s:6,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Turbofuel))),
            output: (Amount::<Conveyable>::new(2, PkgdTurbofuel),
                     None)
        },
        tier: MainProgression(Tier5(OilProcessing))
    },
    Process {
        name:"Packaged Turbofuel",
        time_s:6,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Turbofuel))),
            output: (Amount::<Conveyable>::new(2, PkgdTurbofuel),
                     None)
        },
        tier: MAM(SulfurTier(TurbofuelResearch))
    },
    Process {
        name:"Packaged Turbofuel",
        time_s:6,
        building: Packager {
            input: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Turbofuel))),
            output: (Amount::<Conveyable>::new(2, PkgdTurbofuel),
                     None)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name:"Unpackage Alumina Solution",
        time_s:1,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, AlSol))),
            input: (Amount::<Conveyable>::new(2, PkgdAlSol),
                     None)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name:"Unpackage Fuel",
        time_s:2,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Fuel))),
            input: (Amount::<Conveyable>::new(2, PkgdFuel),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Unpackage Heavy Oil Residue",
        time_s:6,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, HeavyOil))),
            input: (Amount::<Conveyable>::new(2, PkgdHOil),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Unpackage Liquid Biofuel",
        time_s:2,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, LBioFuel))),
            input: (Amount::<Conveyable>::new(2, PkgdLBiofuel),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Unpackage Nitric Acid",
        time_s:3,
        building: Packager {
            output: (Amount::<Conveyable>::new(1, EmptyFluidTank),
                    Some(Amount::<Pipeable>::new(1, NAcid))),
            input: (Amount::<Conveyable>::new(1,PkgdNAcid),
                     None)
        },
        tier: MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name:"Unpackage Nitrogen Gas",
        time_s:1,
        building: Packager {
            output: (Amount::<Conveyable>::new(1, EmptyFluidTank),
                    Some(Amount::<Pipeable>::new(4, NGas))),
            input: (Amount::<Conveyable>::new(1, PkgdN),
                     None)
        },
        tier: MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process {
        name:"Unpackage Oil",
        time_s:2,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, CrudeOil))),
            input: (Amount::<Conveyable>::new(2, PkgdOil),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Unpackage Sulfuric Acid",
        time_s:1,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, SAcid))),
            input: (Amount::<Conveyable>::new(2, PkgdSAcid),
                     None)
        },
        tier: MainProgression(Tier7(AeronauticalEngineering))
    },
    Process {
        name:"Unpackage Water",
        time_s:1,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Water))),
            input: (Amount::<Conveyable>::new(2, PkgdWater),
                     None)
        },
        tier: MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Unpackage Turbofuel",
        time_s:6,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Turbofuel))),
            input: (Amount::<Conveyable>::new(2, PkgdTurbofuel),
                     None)
        },
        tier: MainProgression(Tier5(OilProcessing))
    },
    Process {
        name:"Unpackage Turbofuel",
        time_s:6,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Turbofuel))),
            input: (Amount::<Conveyable>::new(2, PkgdTurbofuel),
                     None)
        },
        tier: MAM(SulfurTier(TurbofuelResearch))
    },
    Process {
        name:"Unpackage Turbofuel",
        time_s:6,
        building: Packager {
            output: (Amount::<Conveyable>::new(2, EmptyCanister),
                    Some(Amount::<Pipeable>::new(2, Turbofuel))),
            input: (Amount::<Conveyable>::new(2, PkgdTurbofuel),
                     None)
        },
        tier: MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name:"Alumina Solution",
        time_s:6,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(12, Bauxite)),
                     Some(Amount::<Pipeable>::new(18, Water))),
            output: (Some(Amount::<Conveyable>::new(5, Silica)),
                     Some(Amount::<Pipeable>::new(12, AlSol)))
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name:"Aluminum Scrap",
        time_s:1,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(2, Coal)),
                    Some(Amount::<Pipeable>::new(4, AlSol))),
            output: (Some(Amount::<Conveyable>::new(6, AlScrap)),
                     Some(Amount::<Pipeable>::new(2, Water)))
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process {
        name:"Fuel",
        time_s:6,
        building: Refinery {
            input: (None,
                      Some(Amount::<Pipeable>::new(6, CrudeOil))),
            output: (Some(Amount::<Conveyable>::new(3, PolymerResin)),
                       Some(Amount::<Pipeable>::new(4, Fuel)))
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process {
        name:"Liquid Biofuel",
        time_s:4,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(6, SolidBiofuel)),
                      Some(Amount::<Pipeable>::new(3, Water))),
            output: (None,
                       Some(Amount::<Pipeable>::new(4, LBioFuel)))
        },
        tier:MainProgression(Tier5(AlternativeFluidTransport))
    },
    Process {
        name:"Petroleum Coke",
        time_s:6,
        building: Refinery {
            input: (None,
                      Some(Amount::<Pipeable>::new(4, HeavyOil))),
            output: (Some(Amount::<Conveyable>::new(12, PetroleumCoke)),
                       None)
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process {
        name:"Plastic",
        time_s:6,
        building: Refinery {
            input: (None,
                    Some(Amount::<Pipeable>::new(6, CrudeOil))),
            output: (Some(Amount::<Conveyable>::new(2, Plastic)),
                     Some(Amount::<Pipeable>::new(1, HeavyOil)))
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process {
        name:"Residual Plastic",
        time_s:6,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(6, PolymerResin)),
                    Some(Amount::<Pipeable>::new(2, Water))),
            output: (Some(Amount::<Conveyable>::new(2, Plastic)),
                     None)
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process {
        name:"Residual Fuel",
        time_s:6,
        building: Refinery {
            input: (None,
                    Some(Amount::<Pipeable>::new(6, HeavyOil))),
            output: (None,
                     Some(Amount::<Pipeable>::new(4, Fuel)))
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process {
        name:"Residual Rubber",
        time_s:6,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(4, PolymerResin)),
                    Some(Amount::<Pipeable>::new(4, Water))),
            output: (Some(Amount::<Conveyable>::new(2, Rubber)),
                     None)
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process {
        name:"Smokeless Powder",
        time_s:6,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(2, BlackPowder)),
                    Some(Amount::<Pipeable>::new(1, HeavyOil))),
            output: (Some(Amount::<Conveyable>::new(2, SmokelessPowder)),
                     None)
        },
        tier:MAM(SulfurTier(SmokelessPowderResearch))
    },
    Process {
        name:"Sulfuric Acid",
        time_s:6,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(5, Sulfur)),
                    Some(Amount::<Pipeable>::new(5, Water))),
            output: (None,
                     Some(Amount::<Pipeable>::new(5, SAcid)))
        },
        tier:MainProgression(Tier7(AeronauticalEngineering))
    },
    Process {
        name:"Coated Cable",
        time_s:8,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(5, CuWire)),
                    Some(Amount::<Pipeable>::new(2, HeavyOil))),
            output: (Some(Amount::<Conveyable>::new(9, Cable)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name:"Diluted Packaged Fuel",
        time_s:2,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(2, PkgdWater)),
                    Some(Amount::<Pipeable>::new(1, HeavyOil))),
            output: (Some(Amount::<Conveyable>::new(2, PkgdFuel)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name:"Electrode Aluminum Scrap",
        time_s:4,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(4, PetroleumCoke)),
                    Some(Amount::<Pipeable>::new(12, AlSol))),
            output: (Some(Amount::<Conveyable>::new(20, AlScrap)),
                     Some(Amount::<Pipeable>::new(7, Water)))
        },
        tier:HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process {
        name:"Heavy Oil Residue",
        time_s:6,
        building: Refinery {
            input: (None,
                    Some(Amount::<Pipeable>::new(3, CrudeOil))),
            output: (Some(Amount::<Conveyable>::new(2, PolymerResin)),
                     Some(Amount::<Pipeable>::new(4, HeavyOil)))
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name:"Polyester Fabric",
        time_s:2,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(1, PolymerResin)),
                    Some(Amount::<Pipeable>::new(1, Water))),
            output: (Some(Amount::<Conveyable>::new(1, Fabric)),
                     None)
        },
        tier:HardDrive(MAMUnlock(Fungi(SyntheticPolyesterFabric)))
    },
    Process {
        name:"Polymer Resin",
        time_s:6,
        building: Refinery {
            input: (None,
                    Some(Amount::<Pipeable>::new(6, CrudeOil))),
            output: (Some(Amount::<Conveyable>::new(13, PolymerResin)),
                     Some(Amount::<Pipeable>::new(2, HeavyOil)))
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name:"Pure Caterium Ingot",
        time_s:5,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(2, CateriumOre)),
                    Some(Amount::<Pipeable>::new(2, Water))),
            output: (Some(Amount::<Conveyable>::new(1, CateriumIngot)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name:"Pure Caterium Ingot",
        time_s:5,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(2, CateriumOre)),
                    Some(Amount::<Pipeable>::new(2, Water))),
            output: (Some(Amount::<Conveyable>::new(1, CateriumIngot)),
                     None)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumResearch)))
    },
    Process {
        name:"Pure Copper Ingot",
        time_s:24,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(6, CuOre)),
                    Some(Amount::<Pipeable>::new(4, Water))),
            output: (Some(Amount::<Conveyable>::new(15, CuIngot)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name:"Pure Iron Ingot",
        time_s:12,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(7, FeOre)),
                    Some(Amount::<Pipeable>::new(4, Water))),
            output: (Some(Amount::<Conveyable>::new(13, FeIngot)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name:"Pure Quartz Crystal",
        time_s:8,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(2, RawQuartz)),
                    Some(Amount::<Pipeable>::new(5, Water))),
            output: (Some(Amount::<Conveyable>::new(7, CrushedQuartz)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name:"Pure Quartz Crystal",
        time_s:8,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(2, RawQuartz)),
                    Some(Amount::<Pipeable>::new(5, Water))),
            output: (Some(Amount::<Conveyable>::new(7, CrushedQuartz)),
                     None)
        },
        tier:HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process {
        name:"Sloppy Alumina",
        time_s:3,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(10, Bauxite)),
                    Some(Amount::<Pipeable>::new(10, Water))),
            output: (None,
                     Some(Amount::<Pipeable>::new(12, AlSol)))
        },
        tier:HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process {
        name:"Steamed Copper Sheet",
        time_s:8,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(3, CuIngot)),
                    Some(Amount::<Pipeable>::new(3, Water))),
            output: (Some(Amount::<Conveyable>::new(3, CuSheet)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name:"Wet Concrete",
        time_s:3,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(6, Limestone)),
                    Some(Amount::<Pipeable>::new(5, Water))),
            output: (Some(Amount::<Conveyable>::new(4, Concrete)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process {
        name:"Recycled Plastic",
        time_s:12,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(6, Rubber)),
                    Some(Amount::<Pipeable>::new(6, Fuel))),
            output: (Some(Amount::<Conveyable>::new(12, Plastic)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name:"Recycled Rubber",
        time_s:12,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(6, Plastic)),
                    Some(Amount::<Pipeable>::new(6, Fuel))),
            output: (Some(Amount::<Conveyable>::new(12, Rubber)),
                     None)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name:"Turbo Heavy Fuel",
        time_s:8,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(4, CompactedCoal)),
                    Some(Amount::<Pipeable>::new(5, HeavyOil))),
            output: (None,
                     Some(Amount::<Pipeable>::new(4, Turbofuel)))
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process {
        name:"Turbo Heavy Fuel",
        time_s:8,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(4, CompactedCoal)),
                    Some(Amount::<Pipeable>::new(5, HeavyOil))),
            output: (None,
                     Some(Amount::<Pipeable>::new(4, Turbofuel)))
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process {
        name:"Turbofuel",
        time_s:8,
        building: Refinery {
            input: (Some(Amount::<Conveyable>::new(4, CompactedCoal)),
                    Some(Amount::<Pipeable>::new(6, Fuel))),
            output: (None,
                     Some(Amount::<Pipeable>::new(5, Turbofuel)))
        },
        tier:HardDrive(MAMUnlock(SulfurTier(TurbofuelResearch)))
    },
    Process {
        name:,
        time_s:,
        building: Blender{
            input: (Some(Amount::<Conveyable>::new()),
                    Some(Amount::<Conveyable>::new()),
                    Amount::<Pipeable>::new(),
                    Some(Amount::<Pipeable>::new())),
            output: (Some(Amount::<Conveyable>::new()),
                     Some(Amount::<Pipeable>::new())),
        },
        tier:)
    },
];
