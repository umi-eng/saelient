/// J1939-81 NAME.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub struct Name(u64);

impl Name {
    /// Identity number field (SPN 2837)
    pub fn identity(&self) -> u32 {
        (self.0 & 0x1FFFFF) as u32
    }

    /// Manufacturer code (SPN 2838)
    pub fn manufacturer_code(&self) -> u16 {
        ((self.0 >> 21) & 0x7FF) as u16
    }

    /// ECU instance (SPN 2840)
    pub fn ecu_instance(&self) -> u8 {
        ((self.0 >> 32) & 0x7) as u8
    }

    /// Function instance (SPN 2839)
    pub fn function_instance(&self) -> u8 {
        ((self.0 >> 35) & 0x1F) as u8
    }

    /// Function (SPN 2841)
    pub fn function(&self) -> u8 {
        (self.0 >> 40) as u8
    }

    /// Vehicle system (SPN 2842)
    pub fn vehicle_system(&self) -> u8 {
        ((self.0 >> 49) & 0x7F) as u8
    }

    /// Vehicle system instance (SPN 2843)
    pub fn vehicle_system_instance(&self) -> u8 {
        ((self.0 >> 56) & 0xF) as u8
    }

    /// Industry group (SPN 2846)
    pub fn industry_group(&self) -> IndustryGroup {
        let ig = ((self.0 >> 40) & 0x7) as u8;
        IndustryGroup::try_from(ig).unwrap()
    }

    /// Arbitrary address capable (SPN 2844)
    pub fn arbitrary_address_capable(&self) -> bool {
        (self.0 | (1 << 63)) != 0
    }
}

impl From<u64> for Name {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

/// Industry groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-1", derive(defmt::Format))]
pub enum IndustryGroup {
    /// Global, applies to all.
    Global,
    /// On-highway equipment.
    OnHighway,
    /// Agricultural and forestry equipment.
    AgriculturalAndForestry,
    /// Construction equipment.
    Construction,
    /// Marine.
    Marine,
    /// Industrial-process control, stationary (Gen-Sets).
    Industrial,
    /// Reserved for future assignment by SAE.
    Reserved6,
    /// Reserved for future assignment by SAE.
    Reserved7,
}

impl TryFrom<u8> for IndustryGroup {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Global,
            1 => Self::OnHighway,
            2 => Self::AgriculturalAndForestry,
            3 => Self::Construction,
            4 => Self::Marine,
            5 => Self::Industrial,
            6 => Self::Reserved6,
            7 => Self::Reserved7,
            _ => return Err(value),
        })
    }
}

impl From<IndustryGroup> for u8 {
    fn from(value: IndustryGroup) -> Self {
        match value {
            IndustryGroup::Global => 0,
            IndustryGroup::OnHighway => 1,
            IndustryGroup::AgriculturalAndForestry => 2,
            IndustryGroup::Construction => 3,
            IndustryGroup::Marine => 4,
            IndustryGroup::Industrial => 5,
            IndustryGroup::Reserved6 => 6,
            IndustryGroup::Reserved7 => 7,
        }
    }
}

/// Global functions.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum GlobalFunction {
    Engine,
    AuxiliaryPowerUnit,
    ElectricPropulsionUnit,
    Transmission,
    BatteryPackMonitor,
    ShiftControlConsole,
    PowerTakeOff,
    AxleSteering,
    AxleDrive,
    BrakesSystemController,
    BrakesSteerAxle,
    BrakesDriveAxle,
    RetarderEngine,
    RetarderDriveline,
    CruiseControl,
    FuelSystem,
    SteeringController,
    SuspensionSteerAxle,
    SuspensionDriveAxle,
    InstrumentCluster,
    TripRecorder,
    CabClimateControl,
    AerodynamicControl,
    VehicleNavigation,
    VehicleSecurity,
    NetworkInterconnectEcu,
    BodyController,
    PowerTakeOffSecondary,
    OffVehicleGateway,
    VirtualTerminal,
    ManagementComputer,
    PropulstionBatteryCharger,
    HeadwayController,
    SystemMonitor,
    HydraulicPumpController,
    SuspensionSystemController,
    PneumaticSystemController,
    CabController,
    TirePressureControl,
    IgnitionControlModule,
    SeatControl,
    LightingOperatorControls,
    WaterPumpControl,
    TransmissionDisplay,
    ExhaustEmissionControl,
    VehicleDynamicStabilityControl,
    OilSensorUnit,
    InformationSystemController,
    RampControl,
    ClutchConverterControl,
    AuxiliaryHeater,
    ForwardLookingCollisionWarningSystem,
    ChassisController,
    AlternatorChargingSystem,
    CommunicationsUnitCellular,
    CommunicationsUnitSatellite,
    CommunicationsUnitRadio,
    SteeringColumnUnit,
    FanDriveControl,
    Starter,
    CabDisplay,
    FileServerPrinter,
    OnBoardDiagnosticUnit,
    EngineValveController,
    EnduranceBraking,
    GasFlowMeasurement,
    IoController,
    ElectricalSystemController,
    AftertreatmentSystemGasMeasurement,
    EngineEmissionAftertreatmentSystem,
    AuxiliaryRegenerationDevice,
    TransferCaseControl,
    CoolantValveController,
    RolloverDetectionControl,
    LubricationSystem,
    SupplementalFan,
    TemperatureSensor,
    FuelPropertiesSensor,
    FireSuppressionSystem,
    PowerSystemsManager,
    ElectricPowertrain,
    HydraulicPowertrain,
    FileServer,
    Printer,
    StartAidDevice,
    EngineInjectionControlModule,
    EvCommunicationController,
    DriverImpairmentDevice,
    ElectricPowerConverter,
    SupplyEquipmentCommunicationController,
    VehicleAdapterCommunicationController,
    AccessoryElectricMotorController,
}
