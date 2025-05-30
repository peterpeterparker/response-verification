use crate::{HeaderField, HttpCertificationError, HttpCertificationResult};
use candid::{
    types::{Serializer, Type, TypeInner},
    CandidType, Deserialize,
};
pub use http::Method;
use http::Uri;
use serde::Deserializer;
use std::{borrow::Cow, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MethodWrapper(Method);

impl CandidType for MethodWrapper {
    fn _ty() -> Type {
        TypeInner::Text.into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.0.as_str().idl_serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MethodWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).and_then(|method| {
            Method::from_str(&method)
                .map(Into::into)
                .map_err(|_| serde::de::Error::custom("Invalid HTTP method"))
        })
    }
}

impl From<Method> for MethodWrapper {
    fn from(method: Method) -> Self {
        Self(method)
    }
}

/// A Candid-encodable representation of an HTTP request. This struct is used by
/// the `http_request` method of the HTTP Gateway Protocol's Candid interface.
///
/// # Examples
///
/// ```
/// use ic_http_certification::{HttpRequest, Method};
///
/// let request = HttpRequest::builder()
///     .with_method(Method::GET)
///     .with_url("/")
///     .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
///     .with_body(&[1, 2, 3])
///     .with_certificate_version(2)
///     .build();
///
/// assert_eq!(request.method(), Method::GET);
/// assert_eq!(request.url(), "/");
/// assert_eq!(request.headers(), &[("X-Custom-Foo".into(), "Bar".into())]);
/// assert_eq!(request.body(), &[1, 2, 3]);
/// assert_eq!(request.certificate_version(), Some(2));
/// ```
///
/// # Helpers
///
/// There are also a number of convenience methods for quickly creating an [HttpRequest] with
/// commonly used HTTP methods:
///
/// - [GET](HttpRequest::get)
/// - [POST](HttpRequest::post)
/// - [PUT](HttpRequest::put)
/// - [PATCH](HttpRequest::patch)
/// - [DELETE](HttpRequest::delete)
///
/// ```
/// use ic_http_certification::HttpRequest;
///
/// let request = HttpRequest::get("/").build();
///
/// assert_eq!(request.method(), "GET");
/// assert_eq!(request.url(), "/");
/// ```
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq)]
pub struct HttpRequest<'a> {
    /// HTTP request method.
    method: MethodWrapper,

    /// HTTP request URL.
    url: String,

    /// HTTP request headers.
    headers: Vec<HeaderField>,

    /// HTTP request body as an array of bytes.
    body: Cow<'a, [u8]>,

    /// The max response verification version to use in the response's
    /// certificate.
    certificate_version: Option<u16>,
}

impl<'a> HttpRequest<'a> {
    /// Creates a new [HttpRequestBuilder] initialized with a GET method and
    /// the given URL.
    ///
    /// This method returns an instance of [HttpRequestBuilder] which can be
    /// used to create an [HttpRequest].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequest, Method};
    ///
    /// let request = HttpRequest::get("/").build();
    ///
    /// assert_eq!(request.method(), Method::GET);
    /// ```
    pub fn get(url: impl Into<String>) -> HttpRequestBuilder<'a> {
        HttpRequestBuilder::new()
            .with_method(Method::GET)
            .with_url(url)
    }

    /// Creates a new [HttpRequestBuilder] initialized with a POST method and
    /// the given URL.
    ///
    /// This method returns an instance of [HttpRequestBuilder] which can be
    /// used to create an [HttpRequest].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequest, Method};
    ///
    /// let request = HttpRequest::post("/").build();
    ///
    /// assert_eq!(request.method(), Method::POST);
    /// ```
    pub fn post(url: impl Into<String>) -> HttpRequestBuilder<'a> {
        HttpRequestBuilder::new()
            .with_method(Method::POST)
            .with_url(url)
    }

    /// Creates a new [HttpRequestBuilder] initialized with a PUT method and
    /// the given URL.
    ///
    /// This method returns an instance of [HttpRequestBuilder] which can be
    /// used to create an [HttpRequest].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequest, Method};
    ///
    /// let request = HttpRequest::put("/").build();
    ///
    /// assert_eq!(request.method(), Method::PUT);
    /// ```
    pub fn put(url: impl Into<String>) -> HttpRequestBuilder<'a> {
        HttpRequestBuilder::new()
            .with_method(Method::PUT)
            .with_url(url)
    }

    /// Creates a new [HttpRequestBuilder] initialized with a PATCH method and
    /// the given URL.
    ///
    /// This method returns an instance of [HttpRequestBuilder] which can be
    /// used to create an [HttpRequest].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequest, Method};
    ///
    /// let request = HttpRequest::patch("/").build();
    ///
    /// assert_eq!(request.method(), Method::PATCH);
    /// ```
    pub fn patch(url: impl Into<String>) -> HttpRequestBuilder<'a> {
        HttpRequestBuilder::new()
            .with_method(Method::PATCH)
            .with_url(url)
    }

    /// Creates a new [HttpRequestBuilder] initialized with a DELETE method and
    /// the given URL.
    ///
    /// This method returns an instance of [HttpRequestBuilder] which can be
    /// used to create an [HttpRequest].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequest, Method};
    ///
    /// let request = HttpRequest::delete("/").build();
    ///
    /// assert_eq!(request.method(), Method::DELETE);
    /// ```
    pub fn delete(url: impl Into<String>) -> HttpRequestBuilder<'a> {
        HttpRequestBuilder::new()
            .with_method(Method::DELETE)
            .with_url(url)
    }

    /// Creates and returns an instance of [HttpRequestBuilder], a builder-style object
    /// which can be used to create an [HttpRequest].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequest, Method};
    ///
    /// let request = HttpRequest::builder()
    ///     .with_method(Method::GET)
    ///     .with_url("/")
    ///     .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
    ///     .with_body(&[1, 2, 3])
    ///     .with_certificate_version(2)
    ///     .build();
    ///
    /// assert_eq!(request.method(), Method::GET);
    /// assert_eq!(request.url(), "/");
    /// assert_eq!(request.headers(), &[("X-Custom-Foo".into(), "Bar".into())]);
    /// assert_eq!(request.body(), &[1, 2, 3]);
    /// assert_eq!(request.certificate_version(), Some(2));
    /// ```
    #[inline]
    pub fn builder() -> HttpRequestBuilder<'a> {
        HttpRequestBuilder::new()
    }

    /// Returns the HTTP method of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("/").build();
    ///
    /// assert_eq!(request.method(), "GET");
    /// ```
    #[inline]
    pub fn method(&self) -> &Method {
        &self.method.0
    }

    /// Returns the URL of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("/").build();
    ///
    /// assert_eq!(request.url(), "/");
    /// ```
    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the headers of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("/")
    ///     .with_headers(vec![("Accept".into(), "text/plain".into())])
    ///     .build();
    ///
    /// assert_eq!(request.headers(), &[("Accept".into(), "text/plain".into())]);
    /// ```
    #[inline]
    pub fn headers(&self) -> &[HeaderField] {
        &self.headers
    }

    /// Returns a mutable reference to the HTTP headers of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let mut request = HttpRequest::get("/")
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .build();
    ///
    /// request.headers_mut().push(("Content-Length".into(), "13".into()));
    ///
    /// assert_eq!(request.headers(), &[("Content-Type".into(), "text/plain".into()), ("Content-Length".into(), "13".into())]);
    /// ```
    #[inline]
    pub fn headers_mut(&mut self) -> &mut Vec<HeaderField> {
        &mut self.headers
    }

    /// Returns the body of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("/")
    ///     .with_body(&[1, 2, 3])
    ///     .build();
    ///
    /// assert_eq!(request.body(), &[1, 2, 3]);
    /// ```
    #[inline]
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Returns the max response verification version to use in the response's
    /// certificate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("/")
    ///     .with_certificate_version(2)
    ///     .build();
    ///
    /// assert_eq!(request.certificate_version(), Some(2));
    /// ```
    #[inline]
    pub fn certificate_version(&self) -> Option<u16> {
        self.certificate_version
    }

    /// Returns the path of the request URL, without domain, query parameters or fragments.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("https://canister.com/sample-asset.txt").build();
    ///
    /// assert_eq!(request.get_path().unwrap(), "/sample-asset.txt");
    /// ```
    pub fn get_path(&self) -> HttpCertificationResult<String> {
        let uri = self
            .url
            .parse::<Uri>()
            .map_err(|_| HttpCertificationError::MalformedUrl(self.url.to_string()))?;

        let decoded_path = urlencoding::decode(uri.path()).map(|path| path.into_owned())?;
        Ok(decoded_path)
    }

    /// Returns the query parameters of the request URL, if any, as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("https://canister.com/sample-asset.txt?foo=bar").build();
    ///
    /// assert_eq!(request.get_query().unwrap(), Some("foo=bar".to_string()));
    /// ```
    pub fn get_query(&self) -> HttpCertificationResult<Option<String>> {
        self.url
            .parse::<Uri>()
            .map(|uri| uri.query().map(|uri| uri.to_owned()))
            .map_err(|_| HttpCertificationError::MalformedUrl(self.url.to_string()))
    }
}

/// An HTTP request builder.
///
/// This type can be used to construct an instance of an [HttpRequest] using a builder-like
/// pattern.
///
/// # Examples
///
/// ```
/// use ic_http_certification::{HttpRequestBuilder, Method};
///
/// let request = HttpRequestBuilder::new()
///     .with_method(Method::GET)
///     .with_url("/")
///     .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
///     .with_body(&[1, 2, 3])
///     .with_certificate_version(2)
///     .build();
///
/// assert_eq!(request.method(), Method::GET);
/// assert_eq!(request.url(), "/");
/// assert_eq!(request.headers(), &[("X-Custom-Foo".into(), "Bar".into())]);
/// assert_eq!(request.body(), &[1, 2, 3]);
/// assert_eq!(request.certificate_version(), Some(2));
/// ```
#[derive(Debug, Clone, Default)]
pub struct HttpRequestBuilder<'a> {
    method: Option<MethodWrapper>,
    url: Option<String>,
    headers: Vec<HeaderField>,
    body: Cow<'a, [u8]>,
    certificate_version: Option<u16>,
}

impl<'a> HttpRequestBuilder<'a> {
    /// Creates a new instance of the [HttpRequestBuilder] that can be used to
    /// construct an [HttpRequest].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequestBuilder, Method};
    ///
    /// let request = HttpRequestBuilder::new()
    ///     .with_method(Method::GET)
    ///     .with_url("/")
    ///     .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
    ///     .with_body(&[1, 2, 3])
    ///     .with_certificate_version(2)
    ///     .build();
    ///
    /// assert_eq!(request.method(), Method::GET);
    /// assert_eq!(request.url(), "/");
    /// assert_eq!(request.headers(), &[("X-Custom-Foo".into(), "Bar".into())]);
    /// assert_eq!(request.body(), &[1, 2, 3]);
    /// assert_eq!(request.certificate_version(), Some(2));
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the HTTP method of the [HttpRequest].
    ///
    /// This function will accept both owned and borrowed values. By default,
    /// the method will be set to `"GET"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequestBuilder, Method};
    ///
    /// let request = HttpRequestBuilder::new()
    ///     .with_method(Method::GET)
    ///     .build();
    ///
    /// assert_eq!(request.method(), Method::GET);
    /// ```
    #[inline]
    pub fn with_method(mut self, method: Method) -> Self {
        self.method = Some(method.into());

        self
    }

    /// Set the HTTP URL of the [HttpRequest].
    ///
    /// This function will accept both owned and borrowed values. By default,
    /// the URL will be set to `"/"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequestBuilder;
    ///
    /// let request = HttpRequestBuilder::new()
    ///     .with_url("/")
    ///     .build();
    ///
    /// assert_eq!(request.url(), "/");
    /// ```
    #[inline]
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());

        self
    }

    /// Set the HTTP headers of the [HttpRequest].
    ///
    /// By default the headers will be an empty array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequestBuilder, HeaderField};
    ///
    /// let request = HttpRequestBuilder::new()
    ///     .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
    ///     .build();
    ///
    /// assert_eq!(request.headers(), &[("X-Custom-Foo".into(), "Bar".into())]);
    /// ```
    #[inline]
    pub fn with_headers(mut self, headers: Vec<HeaderField>) -> Self {
        self.headers = headers;

        self
    }

    /// Set the HTTP body of the [HttpRequest].
    ///
    /// This function will accept both owned and borrowed values. By default,
    /// the body will be an empty array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequestBuilder;
    ///
    /// let request = HttpRequestBuilder::new()
    ///     .with_body(&[1, 2, 3])
    ///     .build();
    ///
    /// assert_eq!(request.body(), &[1, 2, 3]);
    /// ```
    #[inline]
    pub fn with_body(mut self, body: impl Into<Cow<'a, [u8]>>) -> Self {
        self.body = body.into();

        self
    }

    /// Set the max response verification vwersion to use in the
    /// [crate::HttpResponse] certificate.
    ///
    /// By default, the certificate version will be `None`, which
    /// is equivalent to setting it to version `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequestBuilder;
    ///
    /// let request = HttpRequestBuilder::new()
    ///     .with_certificate_version(2)
    ///     .build();
    ///
    /// assert_eq!(request.certificate_version(), Some(2));
    /// ```
    #[inline]
    pub fn with_certificate_version(mut self, certificate_version: u16) -> Self {
        self.certificate_version = Some(certificate_version);

        self
    }

    /// Build an [HttpRequest] from the builder.
    ///
    /// If the method is not set, it will default to `"GET"`.
    /// If the URL is not set, it will default to `"/"`.
    /// If the certificate version is not set, it will default to `1`.
    /// If the headers or body are not set, they will default to empty arrays.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequestBuilder, Method};
    ///
    /// let request = HttpRequestBuilder::new()
    ///     .with_method(Method::GET)
    ///     .with_url("/")
    ///     .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
    ///     .with_body(&[1, 2, 3])
    ///     .with_certificate_version(2)
    ///     .build();
    ///
    /// assert_eq!(request.method(), Method::GET);
    /// assert_eq!(request.url(), "/");
    /// assert_eq!(request.headers(), &[("X-Custom-Foo".into(), "Bar".into())]);
    /// assert_eq!(request.body(), &[1, 2, 3]);
    /// assert_eq!(request.certificate_version(), Some(2));
    /// ```
    #[inline]
    pub fn build(self) -> HttpRequest<'a> {
        HttpRequest {
            method: self.method.unwrap_or(Method::GET.into()),
            url: self.url.unwrap_or("/".to_string()),
            headers: self.headers,
            body: self.body,
            certificate_version: self.certificate_version,
        }
    }

    /// Build an [HttpUpdateRequest] from the builder.
    ///
    /// If the method is not set, it will default to `"GET"`.
    /// If the URL is not set, it will default to `"/"`.
    /// If the headers or body are not set, they will default to empty arrays.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpRequestBuilder, Method};
    ///
    /// let update_request = HttpRequestBuilder::new()
    ///     .with_method(Method::GET)
    ///     .with_url("/")
    ///     .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
    ///     .with_body(&[1, 2, 3])
    ///     .build_update();
    ///
    /// assert_eq!(update_request.method(), Method::GET);
    /// assert_eq!(update_request.url(), "/");
    /// assert_eq!(update_request.headers(), &[("X-Custom-Foo".into(), "Bar".into())]);
    /// assert_eq!(update_request.body(), &[1, 2, 3]);
    /// ```
    #[inline]
    pub fn build_update(self) -> HttpUpdateRequest<'a> {
        HttpUpdateRequest {
            method: self.method.unwrap_or(Method::GET.into()),
            url: self.url.unwrap_or("/".to_string()),
            headers: self.headers,
            body: self.body,
        }
    }
}

/// A Candid-encodable representation of an HTTP update request. This struct is
/// used by the `http_update_request` method of the HTTP Gateway Protocol.
///
/// This is the same as [HttpRequest], excluding the
/// [certificate_version](HttpRequest::certificate_version) property.
///
/// # Examples
///
/// ```
/// use ic_http_certification::{HttpUpdateRequest, HttpRequest, Method};
///
/// let request = HttpRequest::get("/")
///     .with_method(Method::GET)
///     .with_url("/")
///     .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
///     .with_body(&[1, 2, 3])
///     .with_certificate_version(2)
///     .build();
/// let update_request = HttpUpdateRequest::from(request);
///
/// assert_eq!(update_request.method(), Method::GET);
/// assert_eq!(update_request.url(), "/");
/// assert_eq!(update_request.headers(), &[("X-Custom-Foo".into(), "Bar".into())]);
/// assert_eq!(update_request.body(), &[1, 2, 3]);
/// ```
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq)]
pub struct HttpUpdateRequest<'a> {
    /// HTTP request method.
    method: MethodWrapper,

    /// HTTP request URL.
    url: String,

    /// HTTP request headers.
    headers: Vec<HeaderField>,

    /// HTTP request body as an array of bytes.
    body: Cow<'a, [u8]>,
}

impl<'a> HttpUpdateRequest<'a> {
    /// Returns the HTTP method of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("/").build_update();
    ///
    /// assert_eq!(request.method(), "GET");
    /// ```
    #[inline]
    pub fn method(&self) -> &Method {
        &self.method.0
    }

    /// Returns the URL of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("/").build_update();
    ///
    /// assert_eq!(request.url(), "/");
    /// ```
    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the headers of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("/")
    ///     .with_headers(vec![("Accept".into(), "text/plain".into())])
    ///     .build_update();
    ///
    /// assert_eq!(request.headers(), &[("Accept".into(), "text/plain".into())]);
    /// ```
    #[inline]
    pub fn headers(&self) -> &[HeaderField] {
        &self.headers
    }

    /// Returns the body of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("/")
    ///     .with_body(&[1, 2, 3])
    ///     .build_update();
    ///
    /// assert_eq!(request.body(), &[1, 2, 3]);
    /// ```
    #[inline]
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Returns the path of the request URL, without domain, query parameters or fragments.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("https://canister.com/sample-asset.txt").build();
    ///
    /// assert_eq!(request.get_path().unwrap(), "/sample-asset.txt");
    /// ```
    pub fn get_path(&self) -> HttpCertificationResult<String> {
        let uri = self
            .url
            .parse::<Uri>()
            .map_err(|_| HttpCertificationError::MalformedUrl(self.url.to_string()))?;

        let decoded_path = urlencoding::decode(uri.path()).map(|path| path.into_owned())?;
        Ok(decoded_path)
    }

    /// Returns the query parameters of the request URL, if any, as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpRequest;
    ///
    /// let request = HttpRequest::get("https://canister.com/sample-asset.txt?foo=bar").build();
    ///
    /// assert_eq!(request.get_query().unwrap(), Some("foo=bar".to_string()));
    /// ```
    pub fn get_query(&self) -> HttpCertificationResult<Option<String>> {
        self.url
            .parse::<Uri>()
            .map(|uri| uri.query().map(|uri| uri.to_owned()))
            .map_err(|_| HttpCertificationError::MalformedUrl(self.url.to_string()))
    }
}

impl<'a> From<HttpRequest<'a>> for HttpUpdateRequest<'a> {
    fn from(req: HttpRequest<'a>) -> Self {
        HttpUpdateRequest {
            method: req.method,
            url: req.url,
            headers: req.headers,
            body: req.body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_get_uri() {
        let req = HttpRequest::get("https://canister.com/sample-asset.txt").build();

        let path = req.get_path().unwrap();
        let query = req.get_query().unwrap();

        assert_eq!(path, "/sample-asset.txt");
        assert!(query.is_none());
    }

    #[test]
    fn request_get_encoded_uri() {
        let test_requests = [
            (
                HttpRequest::get("https://canister.com/%73ample-asset.txt").build(),
                "/sample-asset.txt",
                "",
            ),
            (
                HttpRequest::get("https://canister.com/path/123?foo=test%20component&bar=1").build(),
                "/path/123",
                "foo=test%20component&bar=1",
            ),
            (
                HttpRequest::get("https://canister.com/a%20file.txt").build(),
                "/a file.txt",
                "",
            ),
            (
                HttpRequest::get("https://canister.com/mujin0722/3888-zjfrd-tqaaa-aaaaf-qakia-cai/%E6%97%A0%E8%AE%BA%E7%BE%8E%E8%81%94%E5%82%A8%E6%98%AF%E5%90%A6%E5%8A%A0%E6%81%AFbtc%E4%BB%8D%E5%B0%86%E5%9B%9E%E5%88%B07%E4%B8%87%E5%88%80").build(),
                "/mujin0722/3888-zjfrd-tqaaa-aaaaf-qakia-cai/无论美联储是否加息btc仍将回到7万刀",
                "",
            ),
        ];

        for (req, expected_path, expected_query) in test_requests.iter() {
            let path = req.get_path().unwrap();
            let query = req.get_query().unwrap();

            assert_eq!(path, *expected_path);
            assert_eq!(query.unwrap_or_default(), *expected_query);
        }
    }
}
