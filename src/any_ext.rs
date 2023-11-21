use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};
use crate::any_ext::CastErr::DowncastFailed;

pub trait AnyExt: Any {}

impl Debug for dyn AnyExt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyExt(trait)").finish()
    }
}

impl<T: 'static + ?Sized> AnyExt for T {}

#[derive(Debug)]
pub enum CastErr {
    TypeIdNotMatch,
    DowncastFailed,
}

#[inline(always)]
pub fn it_is<S: 'static + ?Sized, T: 'static + Sized>(s: &S) -> bool {
    s.type_id() == TypeId::of::<T>()
}

impl dyn AnyExt {
    #[inline(always)]
    pub fn is<T: Sized + 'static>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    #[inline]
    pub fn try_downcast_ref<T>(&self) -> Result<&T, CastErr> where T: Sized + 'static {
        if self.is::<T>() {
            return match (self as &dyn Any).downcast_ref::<T>() {
                None => { Err(DowncastFailed) }
                Some(mpv) => {
                    Ok(mpv)
                }
            };
        }
        Err(CastErr::TypeIdNotMatch)
    }

    #[inline]
    pub fn try_downcast_mut<T>(&mut self) -> Result<&mut T, CastErr> where T: Sized + 'static {
        if self.is::<T>() {
            return match (self as &mut dyn Any).downcast_mut::<T>() {
                None => { Err(DowncastFailed) }
                Some(mpv) => {
                    Ok(mpv)
                }
            };
        }
        Err(CastErr::TypeIdNotMatch)
    }
}

