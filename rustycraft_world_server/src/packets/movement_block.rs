use crate::packets::guid::Guid;
use crate::packets::movement_flags::{ExtraMovementFlags, MovementFlags};
use crate::packets::spline::SplineFlag;
use crate::packets::transport::TransportInfo;
use crate::packets::vector3d::Vector3d;
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
pub struct MovementBlock {
    pub update_flag: MovementBlockUpdateFlag,
}

#[derive(Debug, DekuWrite)]
pub struct MovementBlockUpdateFlag {
    inner: u16,
    living: Option<MovementBlockUpdateFlagLiving>,
    high_guid: Option<u32>,
    low_guid: Option<u32>,
    has_attacking_target: Option<Guid>,
    transport: Option<u32>,
    vehicle: Option<MovementBlockUpdateFlagVehicle>,
    rotation: Option<u64>,
}

#[derive(DekuWrite)]
pub union MovementBlockUpdateFlagLiving {
    living: Living,
    position: Position,
    has_position: HasPosition,
}

#[derive(Debug, DekuWrite)]
pub struct HasPosition {
    orientation2: f32,
    position2: Vector3d,
}

#[derive(Debug, DekuWrite)]
pub struct Position {
    corpse_orientation: f32,
    orientation1: f32,
    position1: Vector3d,
    transport_guid: Guid,
}

#[derive(Debug, DekuWrite)]
pub struct Living {
    flags: MovementFlags,
    extra_flags: ExtraMovementFlags,
    timestamp: u32,
    living_position: Vector3d,
    living_orientation: f32,

    transport: Option<TransportInfo>,
    swimming_pitch: Option<f32>,
    fall_time: f32,
    falling: Option<MovementBlockMovementFlagsFalling>,
    spline_elevation: Option<f32>,

    walking_speed: f32,
    running_speed: f32,
    backwards_running_speed: f32,
    swimming_speed: f32,
    backwards_swimming_speed: f32,
    flight_speed: f32,
    backwards_flight_speed: f32,
    pitch_rate: f32,
    turn_rate: f32,

    spline_enabled: Option<MovementBlockMovementFlagsSplineEnabled>,
}

#[derive(Debug, DekuWrite)]
pub struct MovementBlockUpdateFlagVehicle {
    pub vehicle_id: u32,
    pub vehicle_orientation: f32,
}

#[derive(Debug, DekuWrite)]
pub struct MovementBlockMovementFlagsFalling {
    pub cos_angle: f32,
    pub sin_angle: f32,
    pub xy_speed: f32,
    pub z_speed: f32,
}

#[derive(Debug, Clone, DekuWrite, Builder)]
pub struct MovementBlockMovementFlagsSplineEnabled {
    spline_flags: MovementBlockSplineFlag,
    time_passed: u32,
    duration: u32,
    id: u32,
    #[builder(setter(skip))]
    #[builder(default = "self.nodes.as_ref().map(|v| v.len()).unwrap_or_else(|| 0) as u32")]
    amount_of_nodes: u32,
    #[builder(default)]
    nodes: Vec<Vector3d>,
    final_node: Vector3d,
}

#[derive(Debug, Clone, DekuWrite)]
pub struct MovementBlockSplineFlag {
    inner: SplineFlag,
    angle: Option<f32>,
    target: Option<f32>,
    point: Option<f32>,
}

pub enum FinalMovement {
    Angle(f32),
    Target(f32),
    Point(f32),
}

impl MovementBlockSplineFlag {
    pub fn new(final_type: Option<FinalMovement>) -> MovementBlockSplineFlag {
        let mut inner = SplineFlag::new(0);
        match final_type {
            None => MovementBlockSplineFlag {
                inner,
                angle: None,
                target: None,
                point: None,
            },
            Some(FinalMovement::Angle(angle)) => {
                inner.set_final_angle();
                MovementBlockSplineFlag {
                    inner,
                    angle: Some(angle),
                    target: None,
                    point: None,
                }
            }
            Some(FinalMovement::Target(target)) => {
                inner.set_final_target();
                MovementBlockSplineFlag {
                    inner,
                    angle: None,
                    target: Some(target),
                    point: None,
                }
            }
            Some(FinalMovement::Point(point)) => {
                inner.set_final_point();
                MovementBlockSplineFlag {
                    inner,
                    angle: None,
                    target: None,
                    point: Some(point),
                }
            }
        }
    }
}
