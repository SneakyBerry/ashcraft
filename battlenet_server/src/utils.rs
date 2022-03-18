use axum::http::header::HeaderName;
use bytes::Bytes;

#[allow(dead_code)]
struct ByteStr {
    bytes: Bytes,
}

#[allow(dead_code)]
pub struct Http1Header {
    inner: Repr<Custom>,
}

#[allow(dead_code)]
enum Repr<T> {
    Standard(String),
    Custom(T),
}

#[allow(dead_code)]
struct Custom(ByteStr);

impl Http1Header {
    #[allow(unused_doc_comments)]
    pub fn unsafe_cast(value: &'static str) -> HeaderName {
        /// World of Warcraft client accept only headers following HTTP/1 spec
        /// and we need to set header with value Content-Type case sensitive.
        let my_header = Http1Header {
            inner: Repr::Custom(Custom(ByteStr {
                bytes: Bytes::from(value),
            })),
        };
        let header: HeaderName = unsafe { std::mem::transmute(my_header) };
        header
    }
}

#[cfg(test)]
mod test {
    use crate::utils::{Http1Header};
    use axum::http::header::HeaderName;

    #[test]
    fn test_header_cast() {
        let header_name = Http1Header::unsafe_cast("Content-Type");
        assert_eq!(header_name.as_str(), "Content-Type");
    }
}
