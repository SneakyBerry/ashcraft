use crate::guid::PackedGuid;
use crate::movement_flags::{ExtraMovementFlags, MovementFlags};
use crate::position::{MovementInfo, Vector3d};
use crate::spline::SplineFlag;
use crate::transport::TransportInfo;
use crate::update_flag::UpdateFlag;
use deku::prelude::*;

#[derive(Debug, Clone, DekuWrite, Valuable, Builder)]
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
    high_guid: Option<u32>,
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
    has_attacking_target: Option<PackedGuid>,
    transport: Option<u32>,
    vehicle: Option<MovementBlockVehicle>,
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
            inner: self.inner.as_ref().expect("Has default.").clone(),
            living: None,
            position: None,
            stationary: None,
            high_guid: self.high_guid.unwrap_or_default(),
            low_guid: self.low_guid.unwrap_or_default(),
            has_attacking_target: self.has_attacking_target.clone().unwrap_or_default(),
            transport: self.transport.unwrap_or_default(),
            vehicle: self.vehicle.clone().unwrap_or_default(),
            rotation: self.rotation.unwrap_or_default(),
        };
        match self.living {
            Some(MovementBlockLivingVariants::Living(ref living)) => {
                res.inner.set_living();
                res.living = Some(*living.clone());
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
    Living(Box<Living>),
    Position(Position),
    Stationary(Stationary),
}

#[derive(Debug, Clone, DekuWrite, Valuable)]
pub struct Stationary {
    pub position: Vector3d,
    pub orientation: f32,
}

// TODO: FIX IT
#[derive(Debug, Clone, DekuWrite, Valuable)]
pub struct Position {
    pub corpse_orientation: f32,
    pub position1: Vector3d,
    pub transport_guid: PackedGuid,
}

#[derive(Debug, Clone, DekuWrite, Builder, Valuable)]
pub struct Living {
    movement_data: MovementInfo,
    walking_speed: f32,
    running_speed: f32,
    backwards_running_speed: f32,
    swimming_speed: f32,
    backwards_swimming_speed: f32,
    flight_speed: f32,
    backwards_flight_speed: f32,
    turn_rate: f32,
    pitch_rate: f32,

    #[builder(default)]
    spline_enabled: Option<MovementBlockMovementFlagsSplineEnabled>,
}

#[derive(Debug, Clone, DekuWrite, Valuable)]
pub struct MovementBlockVehicle {
    pub vehicle_id: u32,
    pub vehicle_orientation: f32,
}

#[derive(Debug, Clone, DekuWrite, Valuable)]
pub struct MovementBlockMovementFlagsFalling {
    pub z_speed: f32,
    pub sin_angle: f32,
    pub cos_angle: f32,
    pub xy_speed: f32,
}

#[derive(Debug, Clone, DekuWrite, Valuable, Builder)]
pub struct MovementBlockMovementFlagsSplineEnabled {
    spline_flags: MovementBlockSplineFlag,
    time_passed: u32,
    duration: u32,
    id: u32,
    #[deku(update = "self.nodes.len() as u32")]
    amount_of_nodes: u32,
    nodes: Vec<Vector3d>,
    final_node: Vector3d,
}

#[derive(Debug, Clone, DekuWrite, Valuable)]
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
