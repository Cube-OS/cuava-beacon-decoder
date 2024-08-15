use crate::adcs::enums::*;
use serde::*;

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct BootloaderState {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct ProgInfo {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CopyFlashProgress {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Id {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct BootAndRunningProgStatus {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct BootIndexAndStatus {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct LastLoggedEvent {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct SRAMLatchupCounters {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct EDACErrorCounters {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CommStatus {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct TcmdAck {}


#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct FileDownloadBuffer {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct DownloadBlockReady {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct FileInfo {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct InitializeUploadComplete {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct UploadBlockComplete {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct BlockChecksum {}

// Common Configs
#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CacheEnabledState {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct SRAMScrubParam {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CurrentUnixTime {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct UnixTimeSave {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct HoleMap1 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct HoleMap2 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct HoleMap3 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct HoleMap4 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct HoleMap5 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct HoleMap6 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct HoleMap7 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct HoleMap8 {}

// CubeACP
#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsState1 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct JPGConversionProgress {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CubeACPState {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct EstimatedAttitudeAngles {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct EstimatedAngularRates {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct SatellitePositionECI {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct SatelliteVelocity {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct SatellitePositionLLH {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct MagneticFieldVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CoarseSunVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct FineSunVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct NadirVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RateSensorRates {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct WheelSpeed {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct MagnetorquerCmd {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct WheelSpeedCmds {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct IgrfModelledMagneticFieldVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct ModelledSunVector {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct EstimatedGyroBias {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct EstimationInnovationVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct QErrorVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct QCovariance {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AngularRateCovariance {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawCam2Sensor {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawCam1Sensor {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawCss16 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawCss710 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawMagnetometer {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct FineSunSensorCurrents {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct NadirSensorCurrents {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CubeSenseCurrents {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CubeControlCurrents {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct WheelCurrents {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsTemps {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RateSensorTemps {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawGpsStatus {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawGpsTime {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawGpsX {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawGpsY {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawGpsZ {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Star1BodyVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Star2BodyVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Star3BodyVect {}


#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Star1OrbitVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Star2OrbitVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Star3OrbitVect {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct StarMag {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct StarPerf {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct StarTiming {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsStateAll {
    pub attitude_estimation_mode: EstimationMode,
    pub control_mode: ControlMode,
    pub adcs_run_mode: AdcsRunMode,
    pub asgp4_mode: ASGP4Mode,
    pub cube_control_signal_enabled: bool,
    pub cube_control_motor_enabled: bool,
    pub cube_sense1_enabled: bool,
    pub cube_sense2_enabled: bool,
    pub cube_wheel1_enabled: bool,
    pub cube_wheel2_enabled: bool,
    pub cube_wheel3_enabled: bool,
    pub cube_star_enabled: bool,
    pub gps_receiver_enabled: bool,
    pub gps_lna_power_enabled: bool,
    pub motor_driver_enabled: bool,
    pub sun_is_above_local_horizon: bool,
    pub cube_sense1_communication_error: bool,
    pub cube_sense2_communication_error: bool,
    pub cube_control_signal_communications_error: bool,
    pub cube_control_motor_communications_error: bool,
    pub cube_wheel1_communications_error: bool,
    pub cube_wheel2_communications_error: bool,
    pub cube_wheel3_communications_error: bool,
    pub cube_star_communications_error: bool,
    pub magnetometer_range_error: bool,
    pub cam1_sram_overcurrent_detected: bool,
    pub cam1_3v3_overcurrent_detected: bool,
    pub cam1_sensor_busy_error: bool,
    pub cam1_sensor_detection_error: bool,
    pub sun_sensor_range_error: bool,
    pub cam2_sram_overcurrent_detected: bool,
    pub cam2_3v3_overcurrent_detected: bool,
    pub cam2_sensor_busy_error: bool,
    pub cam2_sensor_detection_error: bool,
    pub nadir_sensor_range_error: bool,
    pub rate_sensor_range_error: bool,
    pub wheel_speed_range_error: bool,
    pub coarse_sun_sensor_error: bool,
    pub star_tracker_match_error: bool,
    pub star_tracker_overcurrent_detected: bool,
    pub orbit_parameters_are_invalid: bool,
    pub configuration_is_invalid: bool,
    pub control_mode_change_is_not_allowed: bool,
    pub estimator_change_is_not_allowed: bool,
    pub current_magnetometer_sampling_mode: MagnetometerMode,
    pub modelled_and_measured_magnetic_field_differs_in_size: bool,
    pub node_recovery_error: bool,
    pub cube_sense1_runtime_error: bool,
    pub cube_sense2_runtime_error: bool,
    pub cube_control_signal_runtime_error: bool,
    pub cube_control_motor_runtime_error: bool,
    pub cube_wheel1_runtime_error: bool,
    pub cube_wheel2_runtime_error: bool,
    pub cube_wheel3_runtime_error: bool,
    pub cube_star_runtime_error: bool,
    pub magnetometer_error: bool,
    pub rate_sensor_failure: bool,
    pub estimated_roll_angle: i16,
    pub estimated_pitch_angle: i16,
    pub estimated_yaw_angle: i16,
    pub estimated_q1: i16,
    pub estimated_q2: i16,
    pub estimated_q3: i16,
    pub estimated_x_angular_rate: i16,
    pub estimated_y_angular_rate: i16,
    pub estimated_z_angular_rate: i16,
    pub x_position: i16,
    pub y_position: i16,
    pub z_position: i16,
    pub x_velocity: i16,
    pub y_velocity: i16,
    pub z_velocity: i16,
    pub latitude: i16,
    pub longitude: i16,
    pub altitude: u16,
    pub ecef_position_x: i16,
    pub ecef_position_y: i16,
    pub ecef_position_z: i16,
}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsMeasurements {
    pub mag_x: i16,
    pub mag_y: i16,
    pub mag_z: i16,
    pub coarse_sun_x: i16,
    pub coarse_sun_y: i16,
    pub coarse_sun_z: i16,
    pub sun_x: i16,
    pub sun_y: i16,
    pub sun_z: i16,
    pub nadir_x: i16,
    pub nadir_y: i16,
    pub nadir_z: i16,
    pub ang_rate_x: i16,
    pub ang_rate_y: i16,
    pub ang_rate_z: i16,
    pub wheel_speed_x: i16,
    pub wheel_speed_y: i16,
    pub wheel_speed_z: i16,
    pub star1bx: i16,
    pub star1by: i16,
    pub star1bz: i16,
    pub star1ox: i16,
    pub star1oy: i16,
    pub star1oz: i16,
    pub star2bx: i16,
    pub star2by: i16,
    pub star2bz: i16,
    pub star2ox: i16,
    pub star2oy: i16,
    pub star2oz: i16,
    pub star3bx: i16,
    pub star3by: i16,
    pub star3bz: i16,
    pub star3ox: i16,
    pub star3oy: i16,
    pub star3oz: i16,
}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct ActuatorCommands {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct EstimationData {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawSensors {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct PowerAndTemp {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsExecutionTimes {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsMiscCurrent {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct FineEstimatedAngularRates {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawGps {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawStarTracker {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Star1Raw {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Star2Raw {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Star3Raw {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct SecondaryMagnetometerRaw {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RawRateSensor {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct EstimatedQ {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct ECEFPosition {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct ACPExecState {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsState2 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct ASGP4TLE {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CubeStarEstimatedRates {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CubeStarEstimatedQ {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct StarPerf2 {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct ImageCaptureAndSaveStatus {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct MagnetorquerConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct WheelConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RateGyroConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CSSConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsPowerControl {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CommandedAttitudeAngles {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct TrackingControllerTgtRef {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct StarTrackerConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CubeSenseConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct MagnetometerConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct RedundantMagnetometerConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct OrbitParams {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct DetumblingControlParam {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct YWheelControlParam {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct ReactionWheelControlParam {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct TrackingControllerGainParam {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct MomentOfInertiaMatrix {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct InertialPointingRefVec {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct EstimationParam {}

#[derive(Clone,Copy,Debug,Serialize,Deserialize,PartialEq)]
pub struct UserControllerAndEstimatorParam {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AugmentedSGP4Param {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct SDcardformatprogress {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct SDLog1Config {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct SDLog2Config {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct UARTLogConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsSystemConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AdcsConfig {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct TriggerWithSimulatedData {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct AugmentedSgp4Parameters {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct EstimationParameters {}

#[derive(Copy,Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct CSSAlignmentConfig {}