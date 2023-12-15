use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};

pub trait AnyExt: Any + 'static {
    // #[inline]
    fn does_impl(&self, tid: &TypeId) -> bool;

    #[inline(always)]
    fn default_basic(&self, tid: &TypeId) -> bool {
        *tid == self.type_id() || *tid == TypeId::of::<dyn AnyExt>() || *tid == TypeId::of::<dyn Any>()
    }
}

#[macro_export]
macro_rules! check_base {
    ($s: expr, $tid: expr) => {
        if $s.default_basic($tid) {
            return true;
        }
    };
}


// impl dyn AnyExt {
//     #[inline]
//     pub fn downcast_trait_ref<T: ?Sized + 'static>(&self) -> Result<&T, CastErr> {
//         // SAFETY: implement the trait can be transmute.
//         if self.impl_it(&TypeId::of::<T>()) {
//             let x: &T = unsafe { mem::transmute(self) };
//             return Ok(x);
//         }
//         Err(CastErr::TraitNotImplement)
//     }
// }
//
impl Debug for dyn AnyExt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyExt(trait)").finish()
    }
}

#[derive(Debug)]
pub enum CastErr {
    TypeIdNotMatch,
    DowncastFailed,
    TraitNotImplement,
}

// #[inline(always)]
// pub fn it_is<S: 'static + ?Sized, T: 'static + Sized>(s: &S) -> bool {
//     s.type_id() == TypeId::of::<T>()
// }

// impl dyn AnyExt {
//     // #[inline(always)]
//     pub fn is<T: Sized + 'static>(&self) -> bool {
//         self.type_id() == TypeId::of::<T>()
//     }
//
//     // #[inline]
//     pub fn try_downcast_ref<T>(&self) -> Result<&T, CastErr> where T: ?Sized + 'static {
//         if self.is::<T>() {
//             return match (self as &dyn Any).downcast_ref::<T>() {
//                 None => { Err(DowncastFailed) }
//                 Some(mpv) => {
//                     Ok(mpv)
//                 }
//             };
//         }
//         Err(CastErr::TypeIdNotMatch)
//     }
//
//     // #[inline]
//     pub fn try_downcast_mut<T>(&mut self) -> Result<&mut T, CastErr> where T: Sized + 'static {
//         if self.is::<T>() {
//             return match (self as &mut dyn Any).downcast_mut::<T>() {
//                 None => { Err(DowncastFailed) }
//                 Some(mpv) => {
//                     Ok(mpv)
//                 }
//             };
//         }
//         Err(CastErr::TypeIdNotMatch)
//     }
// }

