use std::any::{Any, TypeId};
use std::ops::Deref;
use crate::any_ext::CastErr::DowncastFailed;

#[derive(Debug)]
pub enum CastErr {
    TypeIdNotMatch,
    DowncastFailed,
}

#[inline]
pub fn try_downcast_ref<T>(r: &dyn Any) -> Result<&T, CastErr> where T: Sized + 'static {
    if r.type_id() == TypeId::of::<T>() {
        return match r.downcast_ref::<T>() {
            None => { Err(DowncastFailed) }
            Some(mpv) => {
                Ok(mpv)
            }
        };
    }
    Err(CastErr::TypeIdNotMatch)
}

#[inline]
pub fn try_downcast_mut<T>(r: &mut dyn Any) -> Result<&mut T, CastErr> where T: Sized + 'static {
    if r.deref().type_id() == TypeId::of::<T>() {
        return match r.downcast_mut::<T>() {
            None => { Err(DowncastFailed) }
            Some(mpv) => {
                Ok(mpv)
            }
        };
    }
    Err(CastErr::TypeIdNotMatch)
}