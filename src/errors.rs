use std::fmt;

pub trait DetailedError: fmt::Display {
    fn details(&self) -> String;
}