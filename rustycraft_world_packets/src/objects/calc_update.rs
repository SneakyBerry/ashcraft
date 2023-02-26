use crate::objects::size_helper::FieldSize;
use crate::objects::UpdateFields;
use deku::bitvec::BitVec;
use deku::DekuWrite;

pub trait CalcUpdate<const BASE_OFFSET: usize = 0>: Default {
    fn get_diff(&self, old: Option<&Self>) -> UpdateFields;
}

/// In general in UpdateObject packet on client zeroed field means that there is no value
/// If field is not presented it assumed as field not updated at all and you need to
/// use current state of it
impl<T, const BASE_OFFSET: usize> CalcUpdate<BASE_OFFSET> for T
where
    T: DekuWrite + FieldSize + Default,
{
    fn get_diff(&self, old: Option<&Self>) -> UpdateFields {
        let mut update_fields = UpdateFields::new();
        let default_old = Default::default();
        let old = old.unwrap_or(&default_old);

        let mut old_buffer = BitVec::new();
        let mut new_buffer = BitVec::new();

        old.write(&mut old_buffer, ()).expect("Write failed");
        self.write(&mut new_buffer, ()).expect("Write failed");

        for (i, (old_chunk, new_chunk)) in old_buffer
            .as_raw_slice()
            .chunks(T::EL_SIZE * 4)
            .zip(new_buffer.as_raw_slice().chunks(T::EL_SIZE * 4))
            .enumerate()
        {
            if old_chunk != new_chunk {
                update_fields.set_value::<BASE_OFFSET>(new_chunk, i * T::EL_SIZE);
            }
        }
        update_fields
    }
}

#[cfg(test)]
mod tests {
    use crate::guid::{Guid, PlayerGuid};
    use crate::objects::calc_update::CalcUpdate;

    #[test]
    fn test() {
        let a = 0xFFFFFFFFFFFFFFFFu64;
        let b = 0x0000000000000000u64;
        println!("{:?}", &<_ as CalcUpdate<0>>::get_diff(&a, Some(&b)));
        let a = [0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64];
        let b = [0xFFFFFFFFFFFFFFFFu64, 0x0000000000000000u64];
        println!("{:?}", &<_ as CalcUpdate<0>>::get_diff(&a, Some(&b)));
        let a = Guid::Player(PlayerGuid::new(0, 1));
        let b = Guid::Empty;
        println!("{:?}", &<_ as CalcUpdate<0>>::get_diff(&a, Some(&b)));
    }
}
