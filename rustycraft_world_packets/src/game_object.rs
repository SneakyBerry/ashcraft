use crate::guid::TypeMask;
use crate::object::ObjectType;
use crate::update_flag::UpdateFlag;
use crate::update_mask::UpdateFields;

pub struct BaseObject {
    object_type: TypeMask,
    object_type_id: ObjectType,
    update_flag: UpdateFlag,
    update_mask: UpdateFields,
    // ???
    values_count: u16,
    is_in_world: bool,
    is_new_object: bool,
    is_object_updated: bool,
}

pub struct GameObject {
    is_set_flags_count: u8,
    is_set_flags: Vec<u32>,
    object_type: u32,
}
