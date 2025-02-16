use super::wasmedge;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub enum WasmEdgeString<'a> {
    Owned(StringBuf),
    Borrowed(StringRef<'a>),
}

impl Drop for WasmEdgeString<'_> {
    fn drop(&mut self) {
        if let Self::Owned(buf) = self {
            unsafe { wasmedge::WasmEdge_StringDelete(buf.inner) };
        }
    }
}

impl WasmEdgeString<'_> {
    pub fn into_owned(self) -> Self {
        match self {
            Self::Owned(_) => self,
            Self::Borrowed(b) => Self::Owned(b.to_owned()),
        }
    }
}

impl From<StringBuf> for WasmEdgeString<'_> {
    fn from(str_buf: StringBuf) -> Self {
        Self::Owned(str_buf)
    }
}

impl<'a, T: Into<StringRef<'a>>> From<T> for WasmEdgeString<'a> {
    fn from(str_ref: T) -> Self {
        Self::Borrowed(str_ref.into())
    }
}

impl Default for WasmEdgeString<'_> {
    fn default() -> Self {
        Self::Owned(Default::default())
    }
}

impl Default for wasmedge::WasmEdge_String {
    fn default() -> Self {
        wasmedge::WasmEdge_String {
            Length: 0,
            Buf: std::ptr::null(),
        }
    }
}

#[derive(Debug, Default)]
pub struct StringBuf {
    pub(crate) inner: wasmedge::WasmEdge_String,
}

impl<'a> StringBuf {
    pub fn as_ref(&'a self) -> StringRef<'a> {
        StringRef {
            inner: unsafe { wasmedge::WasmEdge_StringWrap(self.inner.Buf, self.inner.Length) },
            lifetime: PhantomData,
        }
    }
}

impl From<String> for StringBuf {
    fn from(s: String) -> Self {
        Self {
            // Safety:  String is valid utf-8 encoding in Rust
            inner: unsafe {
                wasmedge::WasmEdge_StringWrap(s.as_ptr() as *const i8, s.len() as u32)
            },
        }
    }
}

impl Clone for StringBuf {
    fn clone(&self) -> Self {
        Self {
            inner: unsafe {
                wasmedge::WasmEdge_StringCreateByBuffer(self.inner.Buf, self.inner.Length)
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StringRef<'a> {
    pub(crate) inner: wasmedge::WasmEdge_String,
    lifetime: PhantomData<&'a ()>,
}

impl StringRef<'_> {
    pub fn to_owned(self) -> StringBuf {
        StringBuf {
            inner: unsafe {
                wasmedge::WasmEdge_StringCreateByBuffer(self.inner.Buf, self.inner.Length)
            },
        }
    }
}

impl<'a> From<&'a str> for StringRef<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            inner: unsafe {
                wasmedge::WasmEdge_StringWrap(s.as_ptr() as *const i8, s.len() as u32)
            },
            lifetime: PhantomData,
        }
    }
}

impl From<StringRef<'_>> for wasmedge::WasmEdge_String {
    fn from(s: StringRef<'_>) -> Self {
        s.inner
    }
}
