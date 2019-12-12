use crate::error::SantaError;
use crate::function::{ArgumentList, Function};
use failure::_core::fmt::{Error, Formatter};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    String(String),
    Function(Function),
    None,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(i) => write!(f, "{}", i),
            Self::String(i) => write!(f, "{}", i),
            Self::None => write!(f, "None"),
            Self::Function(func) => write!(f, "{:?}", func),
        }
    }
}

impl Object {
    pub fn call(&self, arglist: ArgumentList) -> Result<Object, SantaError> {
        match self {
            Self::Function(i) => Ok(i.call(arglist)),
            _ => Err(SantaError::InvalidOperationError {
                cause: format!("calling type {:?} is not supported", self),
            }),
        }
    }

    pub fn add(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Integer(i + j)),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Float(*i as f64 + j)),
            (Self::Float(i), Self::Integer(j)) => Ok(Self::Float(i + *j as f64)),
            (Self::Float(i), Self::Float(j)) => Ok(Self::Float(i + j)),
            _ => Err(SantaError::InvalidOperationError {
                cause: format!("addition between {:?} and {:?} not supported", self, other),
            }),
        }
    }

    pub fn subtract(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Integer(i - j)),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Float(*i as f64 - j)),
            (Self::Float(i), Self::Integer(j)) => Ok(Self::Float(i - *j as f64)),
            (Self::Float(i), Self::Float(j)) => Ok(Self::Float(i - j)),
            _ => Err(SantaError::InvalidOperationError {
                cause: format!(
                    "subtraction between {:?} and {:?} not supported",
                    self, other
                ),
            }),
        }
    }

    pub fn multiply(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Integer(i * j)),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Float(*i as f64 * j)),
            (Self::Float(i), Self::Integer(j)) => Ok(Self::Float(i * *j as f64)),
            (Self::Float(i), Self::Float(j)) => Ok(Self::Float(i * j)),
            _ => Err(SantaError::InvalidOperationError {
                cause: format!(
                    "multiplication between {:?} and {:?} not supported",
                    self, other
                ),
            }),
        }
    }

    pub fn divide(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Float(*i as f64 / *j as f64)),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Float(*i as f64 / *j as f64)),
            (Self::Float(i), Self::Integer(j)) => Ok(Self::Float(*i as f64 / *j as f64)),
            (Self::Float(i), Self::Float(j)) => Ok(Self::Float(i / j)),
            _ => Err(SantaError::InvalidOperationError {
                cause: format!("division between {:?} and {:?} not supported", self, other),
            }),
        }
    }

    pub fn negate(&self) -> Result<Object, SantaError> {
        match self {
            Self::Integer(i) => Ok(Self::Integer(-i)),
            Self::Float(i) => Ok(Self::Float(-i)),
            _ => Err(SantaError::InvalidOperationError {
                cause: format!("negation for {:?} not supported", self),
            }),
        }
    }
}
