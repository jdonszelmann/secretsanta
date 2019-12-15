use crate::error::SantaError;
use crate::function::{ArgumentList, Function};
use std::fmt::{Display, Error, Formatter};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    String(String),
    Function(Function),
    Boolean(bool),
    List(Vec<Object>),
    Map(HashMap<Object, Object>),
    None,
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Integer(i) => i.hash(state),
            Self::Float(i) => format!("{}", i).hash(state),
            Self::String(i) => i.hash(state),
            Self::None => 0.hash(state),
            Self::Function(_) => unimplemented!("Functions are not a hashable type!"),
            Self::Boolean(i) => i.hash(state),
            Self::List(_) => unimplemented!("Lists are not a hashable type!"),
            Self::Map(_) => unimplemented!("Maps are not a hashable type!"),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
       let res =  self.equals(other).map(|i| {
            if let Self::Boolean(v) = i {
                v
            } else {
                false
            }
        }).unwrap_or(false);

        res
    }
}

impl Eq for Object {}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(i) => write!(f, "{}", i),
            Self::String(i) => write!(f, "{}", i),
            Self::None => write!(f, "None"),
            Self::Function(func) => write!(f, "{:?}", func),
            Self::Boolean(i) => write!(f, "{}", i),
            Self::List(list) => write!(f, "{:?}", list),
            Self::Map(map) => write!(f, "{:?}", map),

        }
    }
}

impl Object {
    pub fn call(&self, arglist: ArgumentList) -> Result<Object, SantaError> {
        match self {
            Self::Function(i) => i.call(arglist),
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

            (Self::Boolean(i), other) => Self::Integer(*i as i64).add(other),
            (other, Self::Boolean(i)) => other.add(&Self::Integer(*i as i64)),

            (Self::String(string), other) => Ok(Self::String(format!("{}{}", string, other))),

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

            (Self::Boolean(i), other) => Self::Integer(*i as i64).subtract(other),
            (other, Self::Boolean(i)) => other.subtract(&Self::Integer(*i as i64)),

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

            (Self::Boolean(i), other) => Self::Integer(*i as i64).multiply(other),
            (other, Self::Boolean(i)) => other.multiply(&Self::Integer(*i as i64)),

            (Self::String(string), Self::Integer(i)) => Ok(Self::String(string.repeat(*i as usize))),

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

            (Self::Boolean(i), other) => Self::Integer(*i as i64).divide(other),
            (other, Self::Boolean(i)) => other.divide(&Self::Integer(*i as i64)),

            _ => Err(SantaError::InvalidOperationError {
                cause: format!("division between {:?} and {:?} not supported", self, other),
            }),
        }
    }

    pub fn negate(&self) -> Result<Object, SantaError> {
        match self {
            Self::Integer(i) => Ok(Self::Integer(-i)),
            Self::Float(i) => Ok(Self::Float(-i)),

            Self::Boolean(i) => Ok(Self::Boolean(!*i)),

            _ => Err(SantaError::InvalidOperationError {
                cause: format!("negation for {:?} not supported", self),
            }),
        }
    }

    pub fn equals(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Boolean(*i == *j)),

            (Self::Integer(i), Self::Boolean(j)) => Ok(Self::Boolean(*i == (*j as i64))),
            (Self::Boolean(i), Self::Integer(j)) => Ok(Self::Boolean((*i as i64) == *j)),

            (Self::Float(i), Self::Float(j)) => Ok(Self::Boolean(*i == *j)),

            (Self::Boolean(i), Self::Boolean(j)) => Ok(Self::Boolean(*i == *j)),
            (Self::String(i), Self::String(j)) => Ok(Self::Boolean(*i == *j)),

            (Self::Float(i), Self::Integer(j)) => Ok(Self::Boolean(*i == (*j as f64))),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Boolean((*i as f64) == *j)),

            (Self::List(i), Self::List(j)) => Ok(Self::Boolean(i == j)),
            (Self::Map(i), Self::Map(j)) => Ok(Self::Boolean(i == j)),

            (Self::None, Self::None) => Ok(Self::Boolean(true)),

            (Self::Function(i), Self::Function(j)) => Ok(Self::Boolean(i == j)),

            _ => Err(SantaError::InvalidOperationError {
                cause: format!(
                    "comparison between {:?} and {:?} not supported",
                    self, other
                ),
            }),
        }
    }

    pub fn notequals(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Boolean(*i != *j)),

            (Self::Integer(i), Self::Boolean(j)) => Ok(Self::Boolean(*i != (*j as i64))),
            (Self::Boolean(i), Self::Integer(j)) => Ok(Self::Boolean((*i as i64) != *j)),

            (Self::Float(i), Self::Float(j)) => Ok(Self::Boolean(*i != *j)),

            (Self::Boolean(i), Self::Boolean(j)) => Ok(Self::Boolean(*i != *j)),
            (Self::String(i), Self::String(j)) => Ok(Self::Boolean(*i != *j)),

            (Self::Float(i), Self::Integer(j)) => Ok(Self::Boolean(*i != (*j as f64))),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Boolean((*i as f64) != *j)),

            (Self::List(i), Self::List(j)) => Ok(Self::Boolean(i != j)),
            (Self::Map(i), Self::Map(j)) => Ok(Self::Boolean(i != j)),

            (Self::None, Self::None) => Ok(Self::Boolean(true)),

            (Self::Function(i), Self::Function(j)) => Ok(Self::Boolean(i != j)),

            (i, j) => i.equals(j)?.negate()
        }
    }

    pub fn less(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Boolean(*i < *j)),

            (Self::Integer(i), Self::Boolean(j)) => Ok(Self::Boolean(*i < (*j as i64))),
            (Self::Boolean(i), Self::Integer(j)) => Ok(Self::Boolean((*i as i64) < *j)),

            (Self::Float(i), Self::Float(j)) => Ok(Self::Boolean(*i < *j)),

            (Self::Float(i), Self::Integer(j)) => Ok(Self::Boolean(*i < (*j as f64))),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Boolean((*i as f64) < *j)),

            _ => Err(SantaError::InvalidOperationError {
                cause: format!(
                    "comparison between {:?} and {:?} not supported",
                    self, other
                ),
            }),
        }
    }

    pub fn greater(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Boolean(*i > *j)),

            (Self::Integer(i), Self::Boolean(j)) => Ok(Self::Boolean(*i > *j as i64)),
            (Self::Boolean(i), Self::Integer(j)) => Ok(Self::Boolean(*i as i64 > *j)),

            (Self::Float(i), Self::Float(j)) => Ok(Self::Boolean(*i > *j)),

            (Self::Float(i), Self::Integer(j)) => Ok(Self::Boolean(*i > *j as f64)),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Boolean(*i as f64 > *j)),

            _ => Err(SantaError::InvalidOperationError {
                cause: format!(
                    "comparison between {:?} and {:?} not supported",
                    self, other
                ),
            }),
        }
    }

    pub fn lessequals(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Boolean(*i <= *j)),

            (Self::Integer(i), Self::Boolean(j)) => Ok(Self::Boolean(*i <= *j as i64)),
            (Self::Boolean(i), Self::Integer(j)) => Ok(Self::Boolean(*i as i64 <= *j)),

            (Self::Float(i), Self::Float(j)) => Ok(Self::Boolean(*i <= *j)),

            (Self::Float(i), Self::Integer(j)) => Ok(Self::Boolean(*i <= *j as f64)),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Boolean(*i as f64 <= *j)),

            _ => Err(SantaError::InvalidOperationError {
                cause: format!(
                    "comparison between {:?} and {:?} not supported",
                    self, other
                ),
            }),
        }
    }

    pub fn greaterequals(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::Integer(i), Self::Integer(j)) => Ok(Self::Boolean(*i >= *j)),

            (Self::Integer(i), Self::Boolean(j)) => Ok(Self::Boolean(*i >= *j as i64)),
            (Self::Boolean(i), Self::Integer(j)) => Ok(Self::Boolean(*i as i64 >= *j)),

            (Self::Float(i), Self::Float(j)) => Ok(Self::Boolean(*i >= *j)),

            (Self::Float(i), Self::Integer(j)) => Ok(Self::Boolean(*i >= *j as f64)),
            (Self::Integer(i), Self::Float(j)) => Ok(Self::Boolean(*i as f64 >= *j)),

            _ => Err(SantaError::InvalidOperationError {
                cause: format!(
                    "comparison between {:?} and {:?} not supported",
                    self, other
                ),
            }),
        }
    }

    pub fn index(&self, other: &Object) -> Result<Object, SantaError> {
        match (self, other) {
            (Self::String(i), Self::Integer(j)) => Ok(Self::String(i.chars().nth(*j as usize).ok_or(SantaError::IndexOutOfBounds)?.to_string())),
            (Self::List(i), Self::Integer(j)) => Ok(i.get(*j as usize).ok_or(SantaError::IndexOutOfBounds)?.clone()),

            (Self::Map(i), j) => Ok(i.get(j).ok_or(SantaError::KeyError)?.clone()),


            // Blanked impl for booleans to work as integers
            (i, Self::Boolean(j)) => i.index(&Self::Integer(*j as i64)),

            _ => Err(SantaError::InvalidOperationError {
                cause: format!(
                    "indexing {:?} with {:?} not supported",
                    self, other
                ),
            }),
        }
    }
}
