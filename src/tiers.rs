#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier {
    MainProgression(ProgressTier),
    HardDrive(HardDriveTier),
    MAM(MamTrees),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum HardDriveTier {
    MainUnlock(ProgressTier),
    MAMUnlock(MamTrees),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ProgressTier {
    Tier0(Tier0),
    Tier2(Tier2),
    Tier1(Tier1),
    Tier3(Tier3),
    Tier4(Tier4),
    Tier5(Tier5),
    Tier6(Tier6),
    Tier7(Tier7),
    Tier8(Tier8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier0 {
    Onboarding,
    HubUpgrade2,
    HubUpgrade3,
    HubUpgrade6,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier1 {
    FieldResearch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier2 {
    ResourceSinkBonus,
    PartAssembly,
    ObstacleClearing,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier3 {
    CoalPower,
    BasicSteel,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier4 {
    AdvancedSteel,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier5 {
    OilProcessing,
    AlternativeFuelTransport,
    IndustrialManufacturing,
    GasMask,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier6 {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier7 {
    BauxiteRefinement,
    AeronauticalEngineering,
    HazmatSuit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tier8 {
    NuclearPower,
    ParticleEnrichment,
    AdvancedAluminumProduction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum MamTrees {
    Ficsmas(FicsmasTier),
    Caterium(CateriumTier),
    SulfurTier(SulfurTier),
    Organisms(OrgoTier),
    Fungi(FungusTier),
    Flowers(FlowerTier),
    PowerSlugs(SlugTier),
    Quartz(QuartzTier),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FicsmasTier {
    TreeUpgrade0,
    CandyCaneBasher,
    CandyCaneDecor,
    TreeUpgrade1,
    AFriend,
    FicsmasGiftTree,
    TreeUpgrade2,
    Lights,
    ItsSnowing,
    TreeUpgrade3,
    Wreath,
    Snowfight,
    TreeUpgrade4,
    Fireworks,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CateriumTier {
    CateriumResearch,
    CateriumIngotResearch,
    QuickwireResearch,
    ZiplineResearch,
    CateriumElectronics,
    StunRebarResearch,
    AILimiterResearch,
    PowerPolesMk2,
    HighSpeedConnectorResearch,
    PowerPolesMk3,
    SmartSplitterResearch,
    PowerSwitchResearch,
    SupercomputerResearch,
    PriorityPowerSwitchResearch,
    BulletGuidanceSystem,
    ProgrammableSplitter,
    GeothermalGeneratorResearch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SulfurTier {
    SulfurResearch,
    BlackPowderResearch,
    ExperimentalPower,
    CompactedCoalResearch,
    TurbofuelResearch,
    ExpandedToolbelt,
    NobeliskDetonator,
    SmokelessPowderResearch,
    NuclearDeterrent,
    ClusterNobeliskResearch,
    ExplosiveRebarResearch,
    Rifle,
    TurboRife,
    InflatedPocket,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum OrgoTier {
    HogResearch,
    HatcherResearch,
    StingerResearch,
    SpitterResearch,
    BioOrganicProperties,
    ProteinInhaler,
    StructuralAnalysis,
    RebarGun,
    InflatedPocketDimension,
    ExpandedToolbelt,
    HostileOrganismDetection,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FungusTier {
    MyceliaResearch,
    FabricResearch,
    ToxicCellularModification,
    MedicalProperties,
    Parachute,
    ExpandedToolbelt,
    SyntheticPolyesterFabric,
    VitaminInhaler,
    TherapeuticInhaler,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FlowerTier {
    FlowerPetalsResearch,
    ColorGun,
    ColorCartridges,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SlugTier {
    BluePowerSlugs,
    SlugScanning,
    YellowPowerShards,
    OverclockProduction,
    PurplePowerShards,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum QuartzTier {
    QuartzResearch,
    CrystalResearch,
    SilicaResearch,
    ShatterRebarResearch,
    CrystalOscillatorResearch,
    BladeRunnersResearch,
    InflatedPocketDimension,
    ExplosiveResonanceApplication,
    TheExplorer,
    RadioSignalScanning,
    RadarTechnology,
}
