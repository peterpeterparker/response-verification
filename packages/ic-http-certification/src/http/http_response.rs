use crate::HeaderField;
use candid::{
    types::{Serializer, Type, TypeInner},
    CandidType, Deserialize,
};
pub use http::StatusCode;
use serde::Deserializer;
use std::{borrow::Cow, fmt::Debug};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct StatusCodeWrapper(StatusCode);

impl CandidType for StatusCodeWrapper {
    fn _ty() -> Type {
        TypeInner::Nat16.into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        self.0.as_u16().idl_serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StatusCodeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        u16::deserialize(deserializer).and_then(|status_code| {
            StatusCode::from_u16(status_code)
                .map(Into::into)
                .map_err(|_| serde::de::Error::custom("Invalid HTTP Status Code."))
        })
    }
}

impl From<StatusCode> for StatusCodeWrapper {
    fn from(status_code: StatusCode) -> Self {
        Self(status_code)
    }
}

/// A Candid-encodable representation of an HTTP response. This struct is used
/// by the `http_request` method of the HTTP Gateway Protocol's Candid interface.
///
/// # Examples
///
/// ```
/// use ic_http_certification::{HttpResponse, StatusCode};
///
/// let response = HttpResponse::builder()
///     .with_status_code(StatusCode::OK)
///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
///     .with_body(b"Hello, World!")
///     .with_upgrade(false)
///     .build();
///
/// assert_eq!(response.status_code(), StatusCode::OK);
/// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
/// assert_eq!(response.body(), b"Hello, World!");
/// assert_eq!(response.upgrade(), Some(false));
/// ```
///
/// # Helpers
///
/// There are also a number of convenience methods for quickly creating an [HttpResponse] with
/// commonly used HTTP status codes:
///
/// - [OK](HttpResponse::ok)
/// - [CREATED](HttpResponse::created)
/// - [NO_CONTENT](HttpResponse::no_content)
/// - [MOVED_PERMANENTLY](HttpResponse::moved_permanently)
/// - [TEMPORARY_REDIRECT](HttpResponse::temporary_redirect)
/// - [BAD_REQUEST](HttpResponse::bad_request)
/// - [UNAUTHORIZED](HttpResponse::unauthorized)
/// - [FORBIDDEN](HttpResponse::forbidden)
/// - [NOT_FOUND](HttpResponse::not_found)
/// - [METHOD_NOT_ALLOWED](HttpResponse::method_not_allowed)
/// - [TOO_MANY_REQUESTS](HttpResponse::too_many_requests)
/// - [INTERNAL_SERVER_ERROR](HttpResponse::internal_server_error)
///
/// ```
/// use ic_http_certification::{HttpResponse, StatusCode};
///
/// let response = HttpResponse::ok(b"Hello, World!", vec![("Content-Type".into(), "text/plain".into())]).build();
///
/// assert_eq!(response.status_code(), StatusCode::OK);
/// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
/// assert_eq!(response.body(), b"Hello, World!");
/// ```
#[derive(Clone, CandidType, Deserialize)]
pub struct HttpResponse<'a> {
    /// HTTP response status code.
    status_code: StatusCodeWrapper,

    /// HTTP response headers.
    headers: Vec<HeaderField>,

    /// HTTP response body as an array of bytes.
    body: Cow<'a, [u8]>,

    /// Whether the corresponding HTTP request should be upgraded to an update
    /// call.
    upgrade: Option<bool>,
}

impl<'a> HttpResponse<'a> {
    /// Creates a new [HttpResponseBuilder] initialized with an OK status code and
    /// the given body and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::ok(b"Hello, World!", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::OK);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Hello, World!");
    /// ```
    pub fn ok(
        body: impl Into<Cow<'a, [u8]>>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::OK)
            .with_body(body)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a CREATED status code and
    /// the given body and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::created(b"Hello, World!", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::CREATED);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Hello, World!");
    /// ```
    pub fn created(
        body: impl Into<Cow<'a, [u8]>>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::CREATED)
            .with_body(body)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a NO_CONTENT status code and
    /// the given headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::no_content(vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// ```
    pub fn no_content(headers: Vec<(String, String)>) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::NO_CONTENT)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a MOVED_PERMANENTLY status code and
    /// the given location and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::moved_permanently("https://www.example.com", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::MOVED_PERMANENTLY);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into()), ("Location".into(), "https://www.example.com".into())]);
    /// ```
    pub fn moved_permanently(
        location: impl Into<String>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        let headers = headers
            .into_iter()
            .chain(std::iter::once(("Location".into(), location.into())))
            .collect();

        Self::builder()
            .with_status_code(StatusCode::MOVED_PERMANENTLY)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a NOT_MODIFIED status code and
    /// the given headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::not_modified(vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::NOT_MODIFIED);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// ```
    pub fn not_modified(headers: Vec<(String, String)>) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::NOT_MODIFIED)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a TEMPORARY_REDIRECT status code and
    /// the given location and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::temporary_redirect("https://www.example.com", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::TEMPORARY_REDIRECT);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into()), ("Location".into(), "https://www.example.com".into())]);
    /// ```
    pub fn temporary_redirect(
        location: impl Into<String>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        let headers = headers
            .into_iter()
            .chain(std::iter::once(("Location".into(), location.into())))
            .collect();

        Self::builder()
            .with_status_code(StatusCode::TEMPORARY_REDIRECT)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a BAD_REQUEST status code and
    /// the given body and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::bad_request(b"Bad Request", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Bad Request");
    /// ```
    pub fn bad_request(
        body: impl Into<Cow<'a, [u8]>>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::BAD_REQUEST)
            .with_body(body)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with an UNAUTHORIZED status code and
    /// the given body and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::unauthorized(b"Unauthorized", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Unauthorized");
    /// ```
    pub fn unauthorized(
        body: impl Into<Cow<'a, [u8]>>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::UNAUTHORIZED)
            .with_body(body)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a FORBIDDEN status code and
    /// the given body and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::forbidden(b"Forbidden", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Forbidden");
    /// ```
    pub fn forbidden(
        body: impl Into<Cow<'a, [u8]>>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::FORBIDDEN)
            .with_body(body)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a NOT_FOUND status code and
    /// the given body and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};  
    ///
    /// let response = HttpResponse::not_found(b"Not Found", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Not Found");
    /// ```
    pub fn not_found(
        body: impl Into<Cow<'a, [u8]>>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::NOT_FOUND)
            .with_body(body)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a METHOD_NOT_ALLOWED status code and
    /// the given body and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::method_not_allowed(b"Method Not Allowed", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Method Not Allowed");
    /// ```
    pub fn method_not_allowed(
        body: impl Into<Cow<'a, [u8]>>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::METHOD_NOT_ALLOWED)
            .with_body(body)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a CONFLICT status code and
    /// the given body and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::too_many_requests(b"Too many requests", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::TOO_MANY_REQUESTS);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Too many requests");
    /// ```
    pub fn too_many_requests(
        body: impl Into<Cow<'a, [u8]>>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::TOO_MANY_REQUESTS)
            .with_body(body)
            .with_headers(headers)
    }

    /// Creates a new [HttpResponseBuilder] initialized with a INTERNAL_SERVER_ERROR status code and
    /// the given body and headers.
    ///
    /// This method returns an instance of [HttpResponseBuilder] that can be used to
    /// to create an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::internal_server_error(b"Internal Server Error", vec![("Content-Type".into(), "text/plain".into())]).build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Internal Server Error");
    /// ```
    pub fn internal_server_error(
        body: impl Into<Cow<'a, [u8]>>,
        headers: Vec<(String, String)>,
    ) -> HttpResponseBuilder<'a> {
        Self::builder()
            .with_status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .with_body(body)
            .with_headers(headers)
    }

    /// Creates and returns an instance of [HttpResponseBuilder], a builder-style
    /// object that can be used to construct an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::builder()
    ///     .with_status_code(StatusCode::OK)
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .with_body(b"Hello, World!")
    ///     .with_upgrade(false)
    ///     .build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::OK);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Hello, World!");
    /// assert_eq!(response.upgrade(), Some(false));
    /// ```
    #[inline]
    pub fn builder() -> HttpResponseBuilder<'a> {
        HttpResponseBuilder::new()
    }

    /// Returns the HTTP status code of the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::builder()
    ///     .with_status_code(StatusCode::OK)
    ///     .build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::OK);
    /// ```
    #[inline]
    pub fn status_code(&self) -> StatusCode {
        self.status_code.0
    }

    /// Returns the HTTP headers of the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let response = HttpResponse::builder()
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .build();
    ///
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// ```
    #[inline]
    pub fn headers(&self) -> &[HeaderField] {
        &self.headers
    }

    /// Returns a mutable reference to the HTTP headers of the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let mut response = HttpResponse::builder()
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .build();
    ///
    /// response.headers_mut().push(("Content-Length".into(), "13".into()));
    ///
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into()), ("Content-Length".into(), "13".into())]);
    /// ```
    #[inline]
    pub fn headers_mut(&mut self) -> &mut Vec<HeaderField> {
        &mut self.headers
    }

    /// Adds an additional header to the HTTP response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let mut response = HttpResponse::builder()
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .build();
    ///
    /// response.add_header(("Content-Length".into(), "13".into()));
    ///
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into()), ("Content-Length".into(), "13".into())]);
    /// ```
    #[inline]
    pub fn add_header(&mut self, header: HeaderField) {
        self.headers.push(header);
    }

    /// Returns the HTTP body of the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let response = HttpResponse::builder()
    ///     .with_body(b"Hello, World!")
    ///     .build();
    ///
    /// assert_eq!(response.body(), b"Hello, World!");
    /// ```
    #[inline]
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Returns the upgrade flag of the response. This will determine if the HTTP Gateway will
    /// upgrade the request to an update call.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let response = HttpResponse::builder()
    ///     .with_upgrade(true)
    ///     .build();
    ///
    /// assert_eq!(response.upgrade(), Some(true));
    /// ```
    #[inline]
    pub fn upgrade(&self) -> Option<bool> {
        self.upgrade
    }
}

/// An HTTP response builder.
///
/// This type can be used to construct an instance of an [HttpResponse] using a builder-like
/// pattern.
///
/// # Examples
///
/// ```
/// use ic_http_certification::{HttpResponse, StatusCode};
///
/// let response = HttpResponse::builder()
///     .with_status_code(StatusCode::OK)
///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
///     .with_body(b"Hello, World!")
///     .with_upgrade(false)
///     .build();
///
/// assert_eq!(response.status_code(), StatusCode::OK);
/// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
/// assert_eq!(response.body(), b"Hello, World!");
/// assert_eq!(response.upgrade(), Some(false));
/// ```
#[derive(Debug, Clone, Default)]
pub struct HttpResponseBuilder<'a> {
    status_code: Option<StatusCodeWrapper>,
    headers: Vec<HeaderField>,
    body: Cow<'a, [u8]>,
    upgrade: Option<bool>,
}

impl<'a> HttpResponseBuilder<'a> {
    /// Creates a new instance of the [HttpResponseBuilder] that can be used to
    /// constract an [HttpResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::builder()
    ///     .with_status_code(StatusCode::OK)
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .with_body(b"Hello, World!")
    ///     .with_upgrade(false)
    ///     .build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::OK);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Hello, World!");
    /// assert_eq!(response.upgrade(), Some(false));
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the status code of the HTTP response.
    ///
    /// By default, the status code will be set to `200`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::builder()
    ///     .with_status_code(StatusCode::OK)
    ///     .build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::OK);
    /// ```
    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code.into());

        self
    }

    /// Sets the headers of the HTTP response.
    ///
    /// By default, the headers will be set to an empty array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let response = HttpResponse::builder()
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .build();
    ///
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// ```
    pub fn with_headers(mut self, headers: Vec<HeaderField>) -> Self {
        self.headers = headers;

        self
    }

    /// Sets the body of the HTTP response.
    ///
    /// This function will accept both owned and borrowed values. By default,
    /// the body will be set to an empty array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let response = HttpResponse::builder()
    ///     .with_body(b"Hello, World!")
    ///     .build();
    ///
    /// assert_eq!(response.body(), b"Hello, World!");
    /// ```
    pub fn with_body(mut self, body: impl Into<Cow<'a, [u8]>>) -> Self {
        self.body = body.into();

        self
    }

    /// Sets the upgrade flag of the HTTP response. This will determine if the HTTP Gateway will
    /// upgrade the request to an update call.
    ///
    /// By default, the upgrade flag will be set to `None`, which is the same as `Some(false)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let response = HttpResponse::builder()
    ///     .with_upgrade(true)
    ///     .build();
    ///
    /// assert_eq!(response.upgrade(), Some(true));
    /// ```
    pub fn with_upgrade(mut self, upgrade: bool) -> Self {
        self.upgrade = Some(upgrade);

        self
    }

    /// Build an [HttpResponse] from the builder.
    ///
    /// If the status code is not set, it will default to `200`.
    /// If the upgrade flag is not set, it will default to `None`.
    /// If the headers or body are not set, they will default to empty arrays.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::builder()
    ///     .with_status_code(StatusCode::OK)
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .with_body(b"Hello, World!")
    ///     .with_upgrade(false)
    ///     .build();
    ///
    /// assert_eq!(response.status_code(), StatusCode::OK);
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(response.body(), b"Hello, World!");
    /// assert_eq!(response.upgrade(), Some(false));
    /// ```
    pub fn build(self) -> HttpResponse<'a> {
        HttpResponse {
            status_code: self.status_code.unwrap_or(StatusCode::OK.into()),
            headers: self.headers,
            body: self.body,
            upgrade: self.upgrade,
        }
    }

    /// Build an [HttpUpdateResponse] from the builder.
    ///
    /// If the status code is not set, it will default to `200`.
    /// If the headers or body are not set, they will default to empty arrays.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, HttpUpdateResponse, StatusCode};
    ///
    /// let response = HttpResponse::builder()
    ///     .with_status_code(StatusCode::OK)
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .with_body(b"Hello, World!")
    ///     .build();
    ///
    /// let update_response = HttpUpdateResponse::from(response);
    ///
    /// assert_eq!(update_response.status_code(), StatusCode::OK);
    /// assert_eq!(update_response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// assert_eq!(update_response.body(), b"Hello, World!");
    /// ```
    pub fn build_update(self) -> HttpUpdateResponse<'a> {
        HttpUpdateResponse {
            status_code: self.status_code.unwrap_or(StatusCode::OK.into()),
            headers: self.headers,
            body: self.body,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for HttpResponseBuilder<'a> {
    fn from(response: HttpResponse<'a>) -> Self {
        Self {
            status_code: Some(response.status_code),
            headers: response.headers,
            body: response.body,
            upgrade: response.upgrade,
        }
    }
}

impl PartialEq for HttpResponse<'_> {
    fn eq(&self, other: &Self) -> bool {
        let mut a_headers = self.headers().to_vec();
        a_headers.sort();

        let mut b_headers = other.headers().to_vec();
        b_headers.sort();

        self.status_code == other.status_code
            && a_headers == b_headers
            && self.body == other.body
            && self.upgrade == other.upgrade
    }
}

impl Debug for HttpResponse<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Truncate body to 100 characters for debug output
        let max_body_len = 100;
        let formatted_body = if self.body.len() > max_body_len {
            format!("{:?}...", &self.body[..max_body_len])
        } else {
            format!("{:?}", &self.body)
        };

        f.debug_struct("HttpResponse")
            .field("status_code", &self.status_code)
            .field("headers", &self.headers)
            .field("body", &formatted_body)
            .field("upgrade", &self.upgrade)
            .finish()
    }
}

/// A Candid-encodable representation of an HTTP update response. This struct is used
/// by the `http_update_request` method of the HTTP Gateway Protocol.
///
/// This is the same as [HttpResponse], excluding the
/// [upgrade](HttpResponse::upgrade) field.
///
/// # Examples
///
/// ```
/// use ic_http_certification::{HttpResponse, HttpUpdateResponse, StatusCode};
///
/// let response = HttpResponse::builder()
///     .with_status_code(StatusCode::OK)
///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
///     .with_body(b"Hello, World!")
///     .build_update();
///
/// let update_response = HttpUpdateResponse::from(response);
///
/// assert_eq!(update_response.status_code(), StatusCode::OK);
/// assert_eq!(update_response.headers(), &[("Content-Type".into(), "text/plain".into())]);
/// assert_eq!(update_response.body(), b"Hello, World!");
/// ```
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq)]
pub struct HttpUpdateResponse<'a> {
    /// HTTP response status code.
    status_code: StatusCodeWrapper,

    /// HTTP response headers.
    headers: Vec<HeaderField>,

    /// HTTP response body as an array of bytes.
    body: Cow<'a, [u8]>,
}

impl<'a> HttpUpdateResponse<'a> {
    /// Returns the HTTP status code of the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::{HttpResponse, StatusCode};
    ///
    /// let response = HttpResponse::builder()
    ///     .with_status_code(StatusCode::OK)
    ///     .build_update();
    ///
    /// assert_eq!(response.status_code(), StatusCode::OK);
    /// ```
    #[inline]
    pub fn status_code(&self) -> StatusCode {
        self.status_code.0
    }

    /// Returns the HTTP headers of the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let response = HttpResponse::builder()
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .build_update();
    ///
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into())]);
    /// ```
    #[inline]
    pub fn headers(&self) -> &[HeaderField] {
        &self.headers
    }

    /// Returns a mutable reference to the HTTP headers of the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let mut response = HttpResponse::builder()
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .build_update();
    ///
    /// response.headers_mut().push(("Content-Length".into(), "13".into()));
    ///
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into()), ("Content-Length".into(), "13".into())]);
    /// ```
    #[inline]
    pub fn headers_mut(&mut self) -> &mut Vec<HeaderField> {
        &mut self.headers
    }

    /// Adds an additional header to the HTTP response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let mut response = HttpResponse::builder()
    ///     .with_headers(vec![("Content-Type".into(), "text/plain".into())])
    ///     .build_update();
    ///
    /// response.add_header(("Content-Length".into(), "13".into()));
    ///
    /// assert_eq!(response.headers(), &[("Content-Type".into(), "text/plain".into()), ("Content-Length".into(), "13".into())]);
    /// ```
    #[inline]
    pub fn add_header(&mut self, header: HeaderField) {
        self.headers.push(header);
    }

    /// Returns the HTTP body of the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_http_certification::HttpResponse;
    ///
    /// let response = HttpResponse::builder()
    ///     .with_body(b"Hello, World!")
    ///     .build_update();
    ///
    /// assert_eq!(response.body(), b"Hello, World!");
    /// ```
    #[inline]
    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

impl<'a> From<HttpResponse<'a>> for HttpUpdateResponse<'a> {
    fn from(response: HttpResponse<'a>) -> Self {
        Self {
            status_code: response.status_code,
            headers: response.headers,
            body: response.body,
        }
    }
}
