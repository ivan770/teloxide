//! Various serializers for memory storages.

use serde::{de::DeserializeOwned, ser::Serialize};

/// A serializer for memory storages.
pub trait Serializer<D> {
    type Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error>;
    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error>;
}

/// The JSON serializer for memory storages.
pub struct JSON;

impl<D> Serializer<D> for JSON
where
    D: Serialize + DeserializeOwned,
{
    type Error = serde_json::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        serde_json::from_slice(data)
    }
}

/// The [CBOR] serializer for memory storages.
///
/// [CBOR]: https://en.wikipedia.org/wiki/CBOR
#[cfg(feature = "cbor-serializer")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "cbor-serializer")))]
pub struct CBOR;

#[cfg(feature = "cbor-serializer")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "cbor-serializer")))]
impl<D> Serializer<D> for CBOR
where
    D: Serialize + DeserializeOwned,
{
    type Error = serde_cbor::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        serde_cbor::to_vec(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        serde_cbor::from_slice(data)
    }
}

/// The [Bincode] serializer for memory storages.
///
/// [Bincode]: https://github.com/servo/bincode
#[cfg(feature = "bincode-serializer")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "bincode-serializer")))]
pub struct Bincode;

#[cfg(feature = "bincode-serializer")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "bincode-serializer")))]
impl<D> Serializer<D> for Bincode
where
    D: Serialize + DeserializeOwned,
{
    type Error = bincode::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        bincode::serialize(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        bincode::deserialize(data)
    }
}
