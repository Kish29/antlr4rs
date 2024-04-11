use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;
use crate::value::Val::{Str, Bool, Uint64, Int64, Float64};

pub type StructType = HashMap<String, Val>;

/// [Val] is a type that represents some basic types or complex types.
/// Use this type in antlr visitor mode to create result value in program.
/// To reduce dynamic overhead and program complexity, we do not use [Box]<dyn [std::any::Any]>
#[derive(Debug)]
pub enum Val {
    Nil,
    Bool(bool),
    Int(isize),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Uint(usize),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Float32(f32),
    Float64(f64),
    Str(String),
    RcStr(Rc<str>),
    ArcStr(Arc<str>),
    StaticStr(&'static str),
    Arr(Vec<Val>),
    Struct(StructType),
    BoxAny(Box<dyn Any>),
    RcAny(Rc<dyn Any>),
    ArcAny(Arc<dyn Any>),
    StaticAny(&'static dyn Any),
}

impl Val {

    // todo: work out for type, try_into, must_into, try_to, must_to, arb_into, arb_to

    // #[inline]
    pub fn try_into_i64(self) -> Option<i64> {
        match self {
            Uint64(n) => {
                if n <= i64::MAX as u64 {
                    Some(n as i64)
                } else { None }
            }
            Int64(n) => Some(n),
            Float64(f) => {
                if f.fract() == 0.0 {
                    Some(f as i64)
                } else { None }
            }
            _ => todo!()
        }
    }

    // #[inline]
    pub fn to_u64(self) -> Option<u64> {
        match self {
            Uint64(n) => Some(n),
            Int64(n) => {
                if n == 0 || n.signum() == 1 {
                    Some(n as u64)
                } else {
                    None
                }
            }
            Float64(f) => {
                if f.fract() == 0.0 && f.signum() == 1.0 {
                    Some(f as u64)
                } else { None }
            }
            _ => todo!()
        }
    }

    // #[inline]
    pub fn to_f64(self) -> Option<f64> {
        match self {
            Float64(f) => Some(f),
            Int64(n) => {
                if n <= f64::MAX as i64 && n >= f64::MIN as i64 {
                    Some(n as f64)
                } else { None }
            }
            Uint64(n) => {
                if n <= f64::MAX as u64 {
                    Some(n as f64)
                } else { None }
            }
            _ => todo!()
        }
    }

    // #[inline]
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Bool(b) => Some(b),
            _ => None
        }
    }

    // #[inline]
    pub fn must_i64(self) -> i64 {
        match self.try_into_i64() {
            Some(n) => n,
            None => panic!("value can not convert to i64"),
        }
    }

    // #[inline]
    pub fn must_u64(self) -> u64 {
        match self.to_u64() {
            Some(n) => n,
            None => panic!("value can not convert to u64")
        }
    }

    // #[inline]
    pub fn must_f64(self) -> f64 {
        match self.to_f64() {
            Some(n) => n,
            None => panic!("value can not convert to f64")
        }
    }

    // #[inline]
    pub fn must_bool(self) -> bool {
        match self {
            Bool(b) => b,
            _ => panic!("value can not convert to bool")
        }
    }

    /// convert to i64 arbitrary
    // #[inline]
    pub fn arb_to_i64(&self) -> Option<i64> {
        match self {
            Uint64(n) => Some(n.clone() as i64),
            Int64(n) => Some(n.clone()),
            Float64(n) => {
                todo!()
            }
            Bool(b) => {
                if *b { Some(1) } else { Some(0) }
            }
            Str(s) => {
                todo!()
            }
            _ => None
        }
    }


    // #[inline]
    pub fn arb_to_u64(&self) -> Option<u64> {
        todo!()
    }
}
