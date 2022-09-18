use crate::guid::PackedGuid;
use crate::movement_flags::{ExtraMovementFlags, MovementFlags};
use crate::position::Vector3d;
use crate::spline::SplineFlag;
use crate::transport::TransportInfo;
use crate::update_flag::UpdateFlag;
use deku::prelude::*;

#[derive(Debug, Clone, DekuWrite, Builder)]
#[builder(derive(Debug))]
#[builder(build_fn(skip))]
pub struct MovementBlock {
    #[builder(setter(custom))]
    #[builder(default = "UpdateFlag::new()")]
    inner: UpdateFlag, //15
    #[builder(setter(custom), field(type = "Option<MovementBlockLivingVariants>"))]
    living: Option<Living>,
    #[builder(setter(skip))]
    position: Option<Position>,
    #[builder(setter(skip))]
    stationary: Option<Stationary>,
    #[deku(endian = "little")]
    high_guid: Option<u32>,
    #[deku(endian = "little")]
    low_guid: Option<u32>,
    // ^^^^^^^^^^^^^^^^^^^
    //     if (flags & UPDATEFLAG_LOWGUID)
    //     {
    //         switch (GetTypeId())
    //         {
    //             case TYPEID_OBJECT:
    //             case TYPEID_ITEM:
    //             case TYPEID_CONTAINER:
    //             case TYPEID_GAMEOBJECT:
    //             case TYPEID_DYNAMICOBJECT:
    //             case TYPEID_CORPSE:
    //                 *data << uint32(GetGUID().GetCounter());              // GetGUID().GetCounter()
    //                 break;
    //             //! Unit, Player and default here are sending wrong values.
    //             /// @todo Research the proper formula
    //             case TYPEID_UNIT:
    //                 *data << uint32(0x0000000B);                // unk
    //                 break;
    //             case TYPEID_PLAYER:
    //                 if (flags & UPDATEFLAG_SELF)
    //                     *data << uint32(0x0000002F);            // unk
    //                 else
    //                     *data << uint32(0x00000008);            // unk
    //                 break;
    //             default:
    //                 *data << uint32(0x00000000);                // unk
    //                 break;
    //         }
    //     }
    has_attacking_target: Option<PackedGuid>, // packed
    #[deku(endian = "little")]
    transport: Option<u32>,
    vehicle: Option<MovementBlockVehicle>,
    #[deku(endian = "little")]
    rotation: Option<i64>,
}

impl MovementBlockBuilder {
    pub fn living(&mut self, value: MovementBlockLivingVariants) -> &mut Self {
        self.living = Some(value);
        self
    }

    pub fn set_self(&mut self) -> &mut Self {
        let mut new = self.inner.clone().unwrap_or_default();
        new.set_self();
        self.inner = Some(new);
        self
    }

    pub fn build(&self) -> Result<MovementBlock, MovementBlockBuilderError> {
        let mut res = MovementBlock {
            inner: self.inner.as_ref().unwrap().clone(),
            living: None,
            position: None,
            stationary: None,
            high_guid: self.high_guid.unwrap_or_default().clone(),
            low_guid: self.low_guid.unwrap_or_default().clone(),
            has_attacking_target: self.has_attacking_target.clone().unwrap_or_default(),
            transport: self.transport.unwrap_or_default().clone(),
            vehicle: self.vehicle.clone().unwrap_or_default(),
            rotation: self.rotation.unwrap_or_default().clone(),
        };
        match self.living {
            Some(MovementBlockLivingVariants::Living(ref living)) => {
                res.inner.set_living();
                res.living = Some(living.clone());
            }
            Some(MovementBlockLivingVariants::Position(ref position)) => {
                res.inner.set_position();
                res.position = Some(position.clone());
            }
            Some(MovementBlockLivingVariants::Stationary(ref has_position)) => {
                res.inner.set_has_position();
                res.stationary = Some(has_position.clone());
            }
            _ => {}
        }
        if res.high_guid.is_some() {
            res.inner.set_high_guid();
        }
        if res.low_guid.is_some() {
            res.inner.set_low_guid();
        }
        if res.has_attacking_target.is_some() {
            res.inner.set_has_attacking_target();
        }
        if res.transport.is_some() {
            res.inner.set_transport();
        }
        if res.vehicle.is_some() {
            res.inner.set_vehicle();
        }
        if res.rotation.is_some() {
            res.inner.set_rotation();
        }
        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub enum MovementBlockLivingVariants {
    Living(Living),
    Position(Position),
    Stationary(Stationary),
}

#[derive(Debug, Clone, DekuWrite)]
pub struct Stationary {
    pub position: Vector3d,
    #[deku(endian = "little")]
    pub orientation: f32,
}

// TODO: FIX IT
#[derive(Debug, Clone, DekuWrite)]
pub struct Position {
    #[deku(endian = "little")]
    pub corpse_orientation: f32,
    pub position1: Vector3d,
    pub transport_guid: PackedGuid,
}

#[derive(Debug, Clone, DekuWrite, Builder)]
pub struct Living {
    flags: MovementFlags,            // 19
    extra_flags: ExtraMovementFlags, // 21
    #[deku(endian = "little")]
    timestamp: u32, // 25
    living_position: Vector3d,       // 37

    #[builder(default)]
    // MOVEMENTFLAG_ONTRANSPORT
    transport: Option<TransportInfo>,
    #[builder(default)]
    // MOVEMENTFLAG_SWIMMING | MOVEMENTFLAG_FLYING
    #[deku(endian = "little")]
    swimming_pitch: Option<f32>,
    #[deku(endian = "little")]
    fall_time: f32, // 41
    #[builder(default)]
    // MOVEMENTFLAG_FALLING
    falling: Option<MovementBlockMovementFlagsFalling>,
    #[builder(default)]
    // MOVEMENTFLAG_SPLINE_ELEVATION
    #[deku(endian = "little")]
    spline_elevation: Option<f32>,

    #[deku(endian = "little")]
    walking_speed: f32, // 45
    #[deku(endian = "little")]
    running_speed: f32, // 49
    #[deku(endian = "little")]
    backwards_running_speed: f32,
    #[deku(endian = "little")]
    swimming_speed: f32, // 53
    #[deku(endian = "little")]
    backwards_swimming_speed: f32, // 57
    #[deku(endian = "little")]
    flight_speed: f32, // 61
    #[deku(endian = "little")]
    backwards_flight_speed: f32, // 65
    #[deku(endian = "little")]
    turn_rate: f32, // 69
    #[deku(endian = "little")]
    pitch_rate: f32, // 73

    #[builder(default)]
    spline_enabled: Option<MovementBlockMovementFlagsSplineEnabled>,
}

#[derive(Debug, Clone, DekuWrite)]
pub struct MovementBlockVehicle {
    #[deku(endian = "little")]
    pub vehicle_id: u32,
    #[deku(endian = "little")]
    pub vehicle_orientation: f32,
}

#[derive(Debug, Clone, DekuWrite)]
pub struct MovementBlockMovementFlagsFalling {
    #[deku(endian = "little")]
    pub z_speed: f32,
    #[deku(endian = "little")]
    pub sin_angle: f32,
    #[deku(endian = "little")]
    pub cos_angle: f32,
    #[deku(endian = "little")]
    pub xy_speed: f32,
}

#[derive(Debug, Clone, DekuWrite, Builder)]
pub struct MovementBlockMovementFlagsSplineEnabled {
    spline_flags: MovementBlockSplineFlag,
    #[deku(endian = "little")]
    time_passed: u32,
    #[deku(endian = "little")]
    duration: u32,
    #[deku(endian = "little")]
    id: u32,
    #[deku(update = "self.nodes.len() as u32")]
    #[deku(endian = "little")]
    amount_of_nodes: u32,
    nodes: Vec<Vector3d>,
    final_node: Vector3d,
}

#[derive(Debug, Clone, DekuWrite)]
pub struct MovementBlockSplineFlag {
    inner: SplineFlag,
    #[deku(endian = "little")]
    angle: Option<f32>,
    #[deku(endian = "little")]
    target: Option<f32>,
    #[deku(endian = "little")]
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
