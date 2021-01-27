//! # Data Objects
use derive_more::*;
use downcast_rs::{impl_downcast, DowncastSync};

pub trait DataObject: DowncastSync + Send + Sync + 'static {}

impl_downcast!(sync DataObject);

/// Empty data object
#[derive(Debug, PartialEq)]
pub struct Empty;
impl DataObject for Empty {}

/// Collection data object
#[derive(Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
pub struct Collection(pub Vec<Box<dyn DataObject>>);
impl DataObject for Collection {}

// Support for JSON data structures
impl DataObject for serde_json::Value {}
