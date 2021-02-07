//! # Data Objects
use derive_more::*;
use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::{clone_trait_object, DynClone};

pub trait DataObject: DowncastSync + Send + Sync + DynClone + 'static {
    /// Try sending in a data object into this data object
    ///
    /// If implementation is able to handle it, it should return `Ok(())`, otherwise
    /// it should return `value` through `Err(value)`.
    ///
    /// This function is used primarily by [`DataObjectExt`] implementation for `Box<dyn
    /// DataObject>`. If `try_send` returns `Err`, [`DataObjectExt::send`] will overwrite
    /// the box's value with the given value.
    ///
    /// Default implementation returns `Err(value)`.
    fn try_send(&mut self, value: Box<dyn DataObject>) -> Result<(), Box<dyn DataObject>> {
        Err(value)
    }
}
clone_trait_object!(DataObject);

impl_downcast!(sync DataObject);

pub trait DataObjectExt {
    /// Sends a data object into another data object
    ///
    /// If the target data object has special handling for `send` (`Collection`, for example,
    /// appends the value to its own list), then that's the behaviour that will be chosen.
    ///
    /// Otherwise, `send` will replace target data object with `value`
    fn send(&mut self, value: Box<dyn DataObject>);
}

impl DataObjectExt for Box<dyn DataObject> {
    fn send(&mut self, value: Box<dyn DataObject>) {
        if let Err(value) = self.try_send(value) {
            *self = value;
        }
    }
}

/// Empty data object
#[derive(Debug, PartialEq, Clone)]
pub struct Empty;
impl DataObject for Empty {}

/// Primitive data container
///
/// ## Note
///
/// This type is meant to store data types that aren't [`DataObject`] by themselves,
/// so if, for example, you need to store a collection, use [`Collection`] instead of
/// [`Container`]`<`[`std::vec::Vec`]`<T>>`
#[derive(Clone, PartialEq)]
pub struct Container<T: Send + Sync + Clone + 'static>(pub T);
impl<T: Send + Sync + Clone + 'static> DataObject for Container<T> {}

/// Collection data object
#[derive(Deref, DerefMut, Clone)]
#[deref(forward)]
#[deref_mut(forward)]
pub struct Collection(pub Vec<Box<dyn DataObject>>);
impl DataObject for Collection {
    fn try_send(&mut self, value: Box<dyn DataObject>) -> Result<(), Box<dyn DataObject>> {
        self.0.push(value);
        Ok(())
    }
}

impl Collection {
    /// Creates an empty [`Collection`]
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Default for Collection {
    fn default() -> Self {
        Collection::new()
    }
}

// Support for JSON data structures
impl DataObject for serde_json::Value {}

#[cfg(test)]
mod tests {
    use super::*;
    use bpxe_internal_macros as bpxe_im;

    #[bpxe_im::test]
    fn send_to_empty_replaces() {
        let mut data_object: Box<dyn DataObject> = Box::new(Empty);
        data_object.send(Box::new(Container(1u8)));
        assert!(matches!(
            data_object.downcast::<Container<u8>>().map(|v| *v),
            Ok(Container(1u8))
        ));
    }

    #[bpxe_im::test]
    fn send_to_container_replaces() {
        let mut data_object: Box<dyn DataObject> = Box::new(Container(true));
        data_object.send(Box::new(Empty));
        assert!(matches!(
            data_object.downcast::<Empty>().map(|v| *v),
            Ok(Empty)
        ));
    }

    #[bpxe_im::test]
    fn send_to_collection_appends() {
        let mut data_object: Box<dyn DataObject> = Box::new(Collection::new());
        data_object.send(Box::new(Empty));
        assert!(matches!(
            data_object.downcast_ref::<Collection>(),
            Some(Collection(v)) if v[0].downcast_ref::<Empty>().is_some()
        ));
        data_object.send(Box::new(Container(1u8)));
        assert!(matches!(
            data_object.downcast_ref::<Collection>(),
            Some(Collection(v)) if matches!(v[1].downcast_ref::<Container<u8>>(), Some(Container(1)))
        ));
    }
}
