use crate::bgs::protocol::game_utilities::v1::ClientRequest;
use crate::bgs::protocol::Variant;

impl ClientRequest {
    pub fn get_param(&self, param_name: &str) -> Option<&Variant> {
        for attr in &self.attribute {
            if attr.name.eq(param_name) {
                return Some(&attr.value);
            }
        }
        None
    }
}
