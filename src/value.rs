use std::collections::HashMap;
use std::fmt::Debug;
use crate::atn_type::ATNType::Parser;
use crate::value::Number::{Float, Int, UInt};
use crate::value::Value::{Bool, Num, Str};

pub type Object = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub enum Number {
    // always greater than zero.
    UInt(u64),
    // always less than zero.
    Int(i64),
    // always finite.
    Float(f64),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Num(Number),
    Str(String),
    Arr(Vec<Value>),
    Obj(Object),
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UInt(l), UInt(r)) => l == r,
            (Int(l), Int(r)) => l == r,
            (Float(l), Float(r)) => l == r,
            (_, _) => false
        }
    }
}

impl Eq for Number {}

impl Value {
    #[inline]
    pub fn to_i64(self) -> Option<i64> {
        match self {
            Num(num) => {
                match num {
                    UInt(n) => {
                        if n <= i64::MAX as u64 {
                            Some(n as i64)
                        } else { None }
                    }
                    Int(n) => Some(n),
                    Float(f) => {
                        if f.fract() == 0.0 {
                            Some(f as i64)
                        } else { None }
                    }
                }
            }
            _ => None
        }
    }

    #[inline]
    pub fn to_u64(self) -> Option<u64> {
        match self {
            Num(num) => match num {
                UInt(n) => Some(n),
                Int(n) => {
                    if n == 0 || n.signum() == 1 {
                        Some(n as u64)
                    } else {
                        None
                    }
                }
                Float(f) => {
                    if f.fract() == 0.0 && f.signum() == 1.0 {
                        Some(f as u64)
                    } else { None }
                }
            }
            _ => None
        }
    }

    #[inline]
    pub fn to_f64(self) -> Option<f64> {
        match self {
            Num(num) => match num {
                Float(f) => Some(f),
                Int(n) => {
                    if n <= f64::MAX as i64 && n >= f64::MIN as i64 {
                        Some(n as f64)
                    } else { None }
                }
                UInt(n) => {
                    if n <= f64::MAX as u64 {
                        Some(n as f64)
                    } else { None }
                }
            }
            _ => None
        }
    }

    #[inline]
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Bool(b) => Some(b),
            _ => None
        }
    }

    #[inline]
    pub fn must_i64(self) -> i64 {
        match self.to_i64() {
            Some(n) => n,
            None => panic!("value can not convert to i64"),
        }
    }

    #[inline]
    pub fn must_u64(self) -> u64 {
        match self.to_u64() {
            Some(n) => n,
            None => panic!("value can not convert to u64")
        }
    }

    #[inline]
    pub fn must_f64(self) -> f64 {
        match self.to_f64() {
            Some(n) => n,
            None => panic!("value can not convert to f64")
        }
    }

    #[inline]
    pub fn must_bool(self) -> bool {
        match self {
            Bool(b) => b,
            _ => panic!("value can not convert to bool")
        }
    }

    /// convert to i64 arbitrary
    #[inline]
    pub fn arb_to_i64(&self) -> Option<i64> {
        match self {
            Num(num) => match num {
                UInt(n) => Some(n.clone() as i64),
                Int(n) => Some(n.clone()),
                Float(n) => {
                    todo!()
                }
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


    #[inline]
    pub fn arb_to_u64(&self) -> Option<u64> {
        todo!()
    }
}
