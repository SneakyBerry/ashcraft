use crate::guid::Guid;
use crate::objects::private;

enum DynamicObjectType {
    Portal = 0x0, // unused
    AreaSpell = 0x1,
    FarSightFocus = 0x2,
}

pub trait DynamicObject: private::Object<0x0006> {
    fn set_dynamic_object_caster(&mut self, dynamic_object_caster: Guid) -> &mut Self {
        self.set_value(dynamic_object_caster, 0x0000)
    }
    fn get_dynamic_object_caster(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }
    fn set_dynamic_object_bytes(&mut self, dynamic_object_bytes: [u8; 4]) -> &mut Self {
        self.set_value(dynamic_object_bytes, 0x0002)
    }
    fn get_dynamic_object_bytes(&self) -> Option<u32> {
        self.get_value(0x0002)
    }
    fn set_dynamic_object_spell_id(&mut self, dynamic_object_spell_id: u32) -> &mut Self {
        self.set_value(dynamic_object_spell_id, 0x0003)
    }
    fn get_dynamic_object_spell_id(&self) -> Option<u32> {
        self.get_value(0x0003)
    }
    fn set_dynamic_object_radius(&mut self, dynamic_object_radius: f32) -> &mut Self {
        self.set_value(dynamic_object_radius, 0x0004)
    }
    fn get_dynamic_object_radius(&self) -> Option<f32> {
        self.get_value(0x0004)
    }
    fn set_dynamic_object_cast_time(&mut self, dynamic_object_cast_time: u32) -> &mut Self {
        self.set_value(dynamic_object_cast_time, 0x0005)
    }
    fn get_dynamic_object_cast_time(&self) -> Option<u32> {
        self.get_value(0x0005)
    }
}
