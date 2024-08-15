use serde::*;

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum AcpType {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq,Eq)]
pub enum AdcsRunMode {
#[default]
    Off,
    Enabled,
    Triggered,
    Simulation,
}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum ASGP4Error {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum Asgp4Filter {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq,Eq)]
pub enum ASGP4Mode {
#[default]
    Off,
    Trigger,
    Background,
    Augment,
}
#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum AxisSelect {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum BootCause {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum BootProgram {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum BootStatus {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum CameraSelect {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum CamType {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum CaptureResult {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq,Eq)]
pub enum ControlMode {
#[default]
    /// No control
    None,
    /// Detumbling control
    Detumbling,
    /// Y-Thomson spin
    YThompson,
    /// Y-Wheel momentum stabilised - Initial Pitch Acquisition
    YWheelInitialPitch,
    /// Y-Wheel momentum stabilised - Steady State
    YWheelSteadyState,
    /// XYZ-Wheel control
    XYZWheel,
    /// Rwheel sun tracking control
    RWheelSun,
    /// Rwheel target tracking control
    RWheelTarget,
    /// Very Fast-spin Detumbling control
    VeryFastSpinDetumbling,
    /// Fast-spin Detumbling control
    FastSpinDetumbling,
    /// User Specific Control Mode 1
    UserSpec1,
    /// User Specific Control Mode 2
    UserSpec2,
    /// Stop R-wheels
    StopRWheels,
    /// User Coded Control Mode
    UserCoded,
    /// Sun-tracking yaw-or roll-only wheel control mode
    SunTrackYaw,
    /// Target-tracking yaw-only wheel control mode
    TargetTrackYaw,
}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum DetectResult {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq,Eq)]
pub enum EstimationMode {
#[default]
    /// No attitude estimation
    None,
    /// MEMS rate sensing
    MEMSRate,
    /// Magnetometer rate filter
    Magmeter,
    /// Magnetometer rate filter with pitch estimation
    MagmeterPitch,
    /// Magnetometer and Fine-sun TRIAD algorithm
    MagmeterFineSun,
    /// Full-state EKF
    FullEKF,
    /// MEMS gyro EKF
    MEMSgyroEKF,
    /// User Coded Estimation Mode
    User,
}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum ExecutionPoint {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum FileType {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum GPIOPort {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum GPIOPortPin {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum GPSSolutionStatus {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum GpsType {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum HoleMapID {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum IdentMode {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum ImageSize {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum ImSaveStatus {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum JpgConvertResult {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq,Eq)]
pub enum MagnetometerMode {
#[default]
    /// Main MTM Sampled Through Signal
    MainSignal,
    /// Redundant MTM Sampled Through Signal
    RedSignal,
    /// Main MTM Sampled Through Motor
    MainMotor,
    /// None
    None,
}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum PowerSelect {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum ProgramList {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum ResetCause {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum SdLog {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum SpecialCon {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum StarIDModeVal {}

#[derive(Copy,Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum TCErrorStatus {}