use crate::{hash::HashInto, marker::*};
use core::marker::PhantomData;
use digest::Digest;
use subtle::ConstantTimeEq;

/// `Slice` represents some potentially secret bytes of arbitrary length.  It
/// exists so you can mark some bytes with a [`Secrecy`]. The only automatic
/// effect this has is that equality comparison runs in constant time if either
/// `Slice` is marked [`Secret`].
///
/// [`Secrecy`]: crate::marker::Secrecy
/// [`Secret`]: crate::marker::Secret
///
/// # Example
///
/// To crate a new `Slice` just [`mark`] any `&[u8]`
///
/// ```
/// use secp256kfun::marker::*;
/// let bytes = b"a secret message";
/// let slice = bytes.as_ref().mark::<Secret>();
/// ```

#[derive(Debug, Clone, Copy)]
pub struct Slice<'a, S> {
    pub(crate) inner: &'a [u8],
    secrecy: PhantomData<S>,
}

impl<'a, 'b, S1, S2> PartialEq<Slice<'b, S2>> for Slice<'a, S1> {
    default fn eq(&self, rhs: &Slice<'b, S2>) -> bool {
        // by default do comparison constant time
        self.inner.ct_eq(rhs.inner).into()
    }
}

impl<'a, 'b> PartialEq<Slice<'b, Public>> for Slice<'a, Public> {
    fn eq(&self, rhs: &Slice<'b, Public>) -> bool {
        // if both are public do variable time
        self.inner == rhs.inner
    }
}

impl<'a, S> From<Slice<'a, S>> for &'a [u8] {
    fn from(msg: Slice<'a, S>) -> Self {
        msg.inner
    }
}

impl<'a, S> From<&'a [u8]> for Slice<'a, S> {
    fn from(slice: &'a [u8]) -> Self {
        Slice {
            inner: slice,
            secrecy: PhantomData::<S>,
        }
    }
}

impl<'a, S> HashInto for Slice<'a, S> {
    fn hash_into(&self, hash: &mut impl Digest) {
        hash.input(self.inner)
    }
}

impl<S> core::fmt::Display for Slice<'_, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for byte in self.inner.iter() {
            write!(f, "{:02x}", byte)?
        }
        Ok(())
    }
}
