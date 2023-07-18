use arrow2::array::PrimitiveArray;
use arrow2::types::NativeType;

/// An enum over primitive types defined by [`arrow2::types::NativeType`]. These include u8, i32,
/// f64, etc.
///
/// [`IntoIterator`] is implemented for this, where it will iterate over the `Array` variant
/// normally but will iterate over the `Scalar` variant forever.
#[derive(Debug, Clone)]
pub enum BroadcastablePrimitive<T>
where
    T: NativeType,
{
    Scalar(T),
    Array(PrimitiveArray<T>),
}

pub enum BroadcastIter<'a, T: NativeType> {
    Scalar(T),
    // TODO: switch this to a ZipValidity that yields option values
    // Array(ZipValidity<&'a T, std::slice::Iter<'a, T>, BitmapIter<'a>>),
    Array(std::slice::Iter<'a, T>),
}

impl<'a, T> IntoIterator for &'a BroadcastablePrimitive<T>
where
    T: NativeType,
{
    type Item = T;
    type IntoIter = BroadcastIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            BroadcastablePrimitive::Array(arr) => BroadcastIter::Array(arr.values_iter()),
            BroadcastablePrimitive::Scalar(val) => BroadcastIter::Scalar(*val),
        }
    }
}

impl<'a, T> Iterator for BroadcastIter<'a, T>
where
    T: NativeType,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BroadcastIter::Array(arr) => arr.next().copied(),
            BroadcastIter::Scalar(val) => Some(val.to_owned()),
        }
    }
}
