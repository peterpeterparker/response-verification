use crate::{Asset, AssetCertificationError};
use globset::{Glob, GlobMatcher};
use ic_http_certification::StatusCode;
use std::fmt::{Display, Formatter};

/// Certification configuration for [assets](Asset). This configuration
/// is passed alongside the [assets](Asset) to the
/// [AssetRouter](crate::AssetRouter).
///
/// The configuration can target a specific [File](AssetConfig::File) or an
/// array of files using a [Pattern](AssetConfig::Pattern).
///
/// Configuration can specify the content type and
/// headers to include for certification and to be served by the
/// [AssetRouter](crate::AssetRouter) for each asset matching the configuration.
///
/// # Examples
///
/// ## JavaScript file
///
/// This example configures an individual JavaScript file to be served by the
/// [AssetRouter](crate::AssetRouter) on the `/app.js` path. The content type is
/// set to `text/javascript` and a `cache-control` header is added.
///
/// ```
/// use ic_http_certification::StatusCode;
/// use ic_asset_certification::{AssetConfig, AssetEncoding};
///
/// let config = AssetConfig::File {
///     path: "app.js".to_string(),
///     content_type: Some("text/javascript".to_string()),
///     headers: vec![
///         ("Cache-Control".to_string(), "public, max-age=31536000, immutable".to_string()),
///     ],
///     fallback_for: vec![],
///     aliased_by: vec![],
///     encodings: vec![
///         AssetEncoding::Brotli.default_config(),
///         AssetEncoding::Gzip.default_config(),
///     ],
/// };
/// ```
///
/// ## Index HTML file with fallback
///
/// This example configures an individual HTML file to be served by the
/// [AssetRouter](crate::AssetRouter) on the `/index.html` path. In addition,
/// it is configured as the fallback for the `/` scope. This means that any
/// request that does not exactly match an asset, will be given this response.
/// The content type is set to `text/html` and a `cache-control` header is added.
///
/// ```
/// use ic_http_certification::StatusCode;
/// use ic_asset_certification::{AssetConfig, AssetFallbackConfig, AssetEncoding};
///
/// let config = AssetConfig::File {
///     path: "index.html".to_string(),
///     content_type: Some("text/html".to_string()),
///     headers: vec![
///         ("Cache-Control".to_string(), "public, no-cache, no-store".to_string()),
///     ],
///     fallback_for: vec![AssetFallbackConfig {
///         scope: "/".to_string(),
///         status_code: Some(StatusCode::OK),
///     }],
///     aliased_by: vec!["/".to_string()],
///     encodings: vec![
///         AssetEncoding::Brotli.default_config(),
///         AssetEncoding::Gzip.default_config(),
///     ],
/// };
/// ```
///
/// ## 404 HTML file with multiple fallbacks and aliases
///
/// This example configures an individual HTML file to be served by the
/// [AssetRouter](crate::AssetRouter) on the `/404.html` path.
///
/// In addition, it is configured as the fallback for the `/js`, and `/css`
/// scopes. This means that any request that does not exactly match an asset in
/// the `/js` or `/css` directories, will be given this response. The content
/// type is set to `text/html` and a `cache-control` header is added.
///
/// The asset is also aliased by multiple paths. This means that any request
/// made to one of these aliases will be served the asset at `/404.html`.
/// The asset is aliased by the following paths:
///     - `/404`
///     - `/404/`
///     - `/404.html`
///     - `/not-found`
///     - `/not-found/`
///     - `/not-found/index.html`
///
/// ```
/// use ic_http_certification::StatusCode;
/// use ic_asset_certification::{AssetConfig, AssetFallbackConfig, AssetEncoding};
///
/// let config = AssetConfig::File {
///     path: "404.html".to_string(),
///     content_type: Some("text/html".to_string()),
///     headers: vec![
///         ("Cache-Control".to_string(), "public, no-cache, no-store".to_string()),
///     ],
///     fallback_for: vec![
///         AssetFallbackConfig {
///             scope: "/css".to_string(),
///             status_code: Some(StatusCode::NOT_FOUND),
///         },
///         AssetFallbackConfig {
///             scope: "/js".to_string(),
///             status_code: Some(StatusCode::NOT_FOUND),
///         },
///     ],
///     aliased_by: vec![
///         "/404".to_string(),
///         "/404/".to_string(),
///         "/404.html".to_string(),
///         "/not-found".to_string(),
///         "/not-found/".to_string(),
///         "/not-found/index.html".to_string(),
///    ],
///     encodings: vec![
///         AssetEncoding::Brotli.default_config(),
///         AssetEncoding::Gzip.default_config(),
///     ],
/// };
/// ```
///
/// ## CSS files using a glob pattern
///
/// This example configures all CSS files to be served by the
/// [AssetRouter](crate::AssetRouter) using a glob pattern. The content type is
/// set to `text/css` and a `cache-control` header is added.
///
/// ```
/// use ic_http_certification::StatusCode;
/// use ic_asset_certification::{AssetConfig, AssetEncoding};
///
/// let config = AssetConfig::Pattern {
///     pattern: "**/*.css".to_string(),
///     content_type: Some("text/css".to_string()),
///     headers: vec![
///         ("Cache-Control".to_string(), "public, max-age=31536000, immutable".to_string()),
///     ],
///     encodings: vec![
///         AssetEncoding::Brotli.default_config(),
///         AssetEncoding::Gzip.default_config(),
///     ],
/// };
/// ```
///
/// ## Temporary redirect
///
/// This example configures a redirect from `/old` to `/new`. The redirect is
/// configured as a temporary redirect (307).
///
/// ```
/// use ic_asset_certification::{AssetConfig, AssetRedirectKind};
///
/// let config = AssetConfig::Redirect {
///     from: "/old".to_string(),
///     to: "/new".to_string(),
///     kind: AssetRedirectKind::Temporary,
///     headers: vec![(
///         "content-type".to_string(),
///         "text/plain; charset=utf-8".to_string(),
///     )],
/// };
/// ```
///
/// ## Permanent redirect
///
/// This example configures a redirect from `/old` to `/new`. The redirect is
/// configured as a permanent redirect (301).
///
/// ```
/// use ic_asset_certification::{AssetConfig, AssetRedirectKind};
///
/// let config = AssetConfig::Redirect {
///     from: "/old".to_string(),
///     to: "/new".to_string(),
///     kind: AssetRedirectKind::Permanent,
///     headers: vec![(
///         "content-type".to_string(),
///         "text/plain; charset=utf-8".to_string(),
///     )],
/// };
/// ```
#[derive(Debug, Clone)]
pub enum AssetConfig {
    /// Matches a specific file.
    File {
        /// The path to the file. This path must exactly match the
        /// path of an [Asset] provided to the [AssetRouter](crate::AssetRouter)
        /// with this config.
        path: String,

        /// The content type of the file (e.g. "text/javascript").
        ///
        /// Providing this option will auto-insert a `Content-Type` header with
        /// the provided value. If this value is not provided, the
        /// `Content-Type` header will not be inserted.
        ///
        /// If the `Content-Type` header is not sent to the browser, the browser
        /// will try to guess the content type based on the file extension,
        /// unless a `X-Content-Type-Options: nosniff` header is sent.
        ///
        /// Not certifying the `Content-Type` header will also allow a malicious
        /// replica to insert its own `Content-Type` header, which could lead
        /// to a security vulnerability.
        content_type: Option<String>,

        /// Additional headers to be inserted into the response. Each additional
        /// header added will be included in certification and served by the
        /// [AssetRouter](crate::AssetRouter) for matching [Assets](Asset).
        headers: Vec<(String, String)>,

        /// Configure this asset as a fallback for a set of scopes.
        ///
        /// When serving assets, if a requested path does not exactly match any
        /// assets then the [AssetRouter](crate::AssetRouter) will search for an
        /// asset configured with a fallback scope that most closely matches
        /// the requested asset's path.
        ///
        /// For example, if a request is made for `/app.js` and no asset with
        /// that exact path is found, the router will attempt to serve an asset
        /// configured with a fallback scope of `/`.
        ///
        /// This will be done recursively until no more fallback scopes are
        /// possible to find. For example, if a request is made for
        /// `/assets/js/app/core/index.js` and no asset with that exact path is
        /// found, the [AssetRouter](crate::AssetRouter) will search for assets
        /// configured with the following fallback scopes, in order:
        /// - `/assets/js/app/core`
        /// - `/assets/js/app`
        /// - `/assets/js`
        /// - `/assets`
        /// - `/`
        ///
        /// If multiple fallback assets are configured, the first one found will
        /// be used. If no asset is found with any of these fallback scopes, no
        /// response will be returned.
        fallback_for: Vec<AssetFallbackConfig>,

        /// A list of aliases for this asset. If a request is made for one of
        /// these aliases, the asset will be served as if the request was made
        /// for the original path.
        ///
        /// For example, if an asset is configured with the path `index.html` and
        /// the alias `/`, a request for `/` will be served the
        /// asset at `index.html`.
        aliased_by: Vec<String>,

        /// A list of encodings to serve the asset with. Each listing includes
        /// the encoding of an asset, and the file extension for the encoded
        /// asset. The router will search for an asset with the provided file
        /// extension and certify all matching encoded assets, if found.
        ///
        /// A list of alternative encodings that can be used to serve the asset.
        ///
        /// Each entry is a tuple of the [encoding name](AssetEncoding) and the
        /// file extension used in the file path. For example, to include Brotli
        /// and Gzip encodings:
        /// `vec![AssetEncoding::Brotli.default_config(), AssetEncoding::Gzip.default_config()]`
        ///
        /// Each encoding referenced must be provided to the asset router as a
        /// separate file with the same filename as the original file, but with
        /// an additional file extension matching the configuration. For
        /// example, if the current matched file is named `file.html`, then the
        /// asset router will look for `file.html.br` and `file.html.gz`.
        ///
        /// If the file is found, the asset will be certified and served with
        /// the provided encoding according to the `Accept-Encoding`. Encodings
        /// are prioritized in the following order:
        ///     - Brotli
        ///     - Zstd
        ///     - Gzip
        ///     - Deflate
        ///     - Identity
        ///
        /// The asset router will return the highest priority encoding that has
        /// been certified and is supported by the client.
        encodings: Vec<(AssetEncoding, String)>,
    },

    /// Matches files using a glob pattern.
    Pattern {
        /// A glob pattern to match files against.
        ///
        /// Standard Unix-style glob syntax is supported:
        /// - `?` matches any single character.
        /// - `*` matches zero or more characters.
        /// - `**` recursively matches directories but is only legal in three
        ///   situations.
        ///   - If the glob starts with `**/`, then it matches all directories.
        ///     For example, `**/foo` matches `foo` and `bar/foo` but not
        ///     `foo/bar`.
        ///   - If the glob ends with `/**`, then it matches all sub-entries.
        ///     For example, `foo/**` matches `foo/a` and `foo/a/b`, but not
        ///     `foo`.
        ///   - If the glob contains `/**/` anywhere within the pattern, then it
        ///     matches zero or more directories.
        ///   - Using `**` anywhere else is illegal.
        ///   - The glob `**` is allowed and means "match everything".
        /// - `{a,b}` matches `a` or `b` where `a` and `b` are arbitrary glob
        ///   patterns. (N.B. Nesting {...} is not currently allowed.)
        /// - `[ab]` matches `a` or `b` where `a` and `b` are characters.
        /// - `[!ab]` to match any character except for `a` and `b`.
        /// - Metacharacters such as `*` and `?` can be escaped with character
        ///   class notation. e.g., `[*]` matches `*`.
        pattern: String,

        /// The content type of the file (e.g. "text/javascript").
        ///
        /// Providing this option will auto-insert a `Content-Type` header with
        /// the provided value. If this value is not provided, the
        /// `Content-Type` header will not be inserted.
        ///
        /// If the `Content-Type` header is not sent to the browser, the browser
        /// will try to guess the content type based on the file extension,
        /// unless a `X-Content-Type-Options: nosniff` header is sent.
        ///
        /// Not certifying the `Content-Type` header will also allow a malicious
        /// replica to insert its own `Content-Type` header, which could lead
        /// to a security vulnerability.
        content_type: Option<String>,

        /// Additional headers to be inserted into the response. Each additional
        /// header added will be included in certification and served by the
        /// [AssetRouter](crate::AssetRouter) for matching [Assets](Asset).
        headers: Vec<(String, String)>,

        /// A list of encodings to serve the asset with. Each listing includes
        /// the encoding of an asset, and the file extension for the encoded
        /// asset. The router will search for an asset with the provided file
        /// extension and certify all matching encoded assets, if found.
        ///
        /// A list of alternative encodings that can be used to serve the asset.
        ///
        /// Each entry is a tuple of the [encoding name](AssetEncoding) and the
        /// file extension used in the file path. For example, to include Brotli
        /// and Gzip encodings:
        /// `vec![AssetEncoding::Brotli.default_config(), AssetEncoding::Gzip.default_config()]`
        ///
        /// Each encoding referenced must be provided to the asset router as a
        /// separate file with the same filename as the original file, but with
        /// an additional file extension matching the configuration. For
        /// example, if the current matched file is named `file.html`, then the
        /// asset router will look for `file.html.br` and `file.html.gz`.
        ///
        /// If the file is found, the asset will be certified and served with
        /// the provided encoding according to the `Accept-Encoding`. Encodings
        /// are prioritized in the following order:
        ///     - Brotli
        ///     - Zstd
        ///     - Gzip
        ///     - Deflate
        ///     - Identity
        ///
        /// The asset router will return the highest priority encoding that has
        /// been certified and is supported by the client.
        encodings: Vec<(AssetEncoding, String)>,
    },

    /// Redirects the request to another URL. This config type is not matched
    /// against any assets.
    Redirect {
        /// The URL to redirect from.
        from: String,

        /// The URL to redirect to.
        to: String,

        /// The kind redirect to configure.
        kind: AssetRedirectKind,

        /// Additional headers to be inserted into the response. Each additional
        /// header added will be included in certification and served by the
        /// [AssetRouter](crate::AssetRouter) for matching [Assets](Asset).
        ///
        /// Note that the `Location` header will be automatically added to the
        /// response with the value of the `to` field.
        headers: Vec<(String, String)>,
    },
}

/// Configuration for an asset to be used as a fallback for a specific scope.
///
/// See the [fallback_for](AssetConfig::File::fallback_for) configuration
/// of the [AssetConfig] interface for more information.
#[derive(Debug, Clone)]
pub struct AssetFallbackConfig {
    /// The scope to use this asset as a fallback for.
    ///
    /// See the [fallback_for](AssetConfig::File::fallback_for)
    /// configuration of the [AssetConfig] interface for more information.
    pub scope: String,

    /// The HTTP status code to return when serving the asset.
    /// If this value is not provided, the default status code will be 200.
    pub status_code: Option<StatusCode>,
}

/// The type of redirect to use. Redirects can be either
/// [permanent](AssetRedirectKind::Permanent) or
/// [temporary](AssetRedirectKind::Temporary).
#[derive(Debug, Clone)]
pub enum AssetRedirectKind {
    /// A permanent redirect (301).
    ///
    /// The browser will cache this redirect and will not make a request to the
    /// old location again. This is useful when the resource has permanently
    /// moved to a new location. The browser will update its bookmarks and
    /// search engine results.
    ///
    /// See the
    /// [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/301)
    /// for more information.
    Permanent,

    /// A temporary redirect (307).
    ///
    /// The browser will not cache this redirect and will make a request to the
    /// old location again. This is useful when the resource has temporarily
    /// moved to a new location. The browser will not update its bookmarks and
    /// search engine results.
    ///
    /// See the
    /// [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/307)
    /// for more information.
    Temporary,
}

/// The encoding of an asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetEncoding {
    /// The asset is not encoded.
    Identity,

    /// The asset is encoded with the Brotli algorithm.
    Brotli,

    /// The asset is encoded with the Zstd algorithm.
    Zstd,

    /// The asset is encoded with the Gzip algorithm.
    Gzip,

    /// The asset is encoded with the Deflate algorithm.
    Deflate,
}

impl AssetEncoding {
    /// Returns the default encoding and file extension for the encoding.
    /// The default encoding is the encoding that is used when the client
    /// does not specify an encoding in the `Accept-Encoding` header.
    ///
    /// The default encoding and file extension are:
    /// - [Brotli](AssetEncoding::Brotli): `br`
    /// - [Zstd](AssetEncoding::Zstd): `zst`
    /// - [Gzip](AssetEncoding::Gzip): `gz`
    /// - [Deflate](AssetEncoding::Deflate): `zz`
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_asset_certification::AssetEncoding;
    ///
    /// let (encoding, extension) = AssetEncoding::Brotli.default_config();
    /// assert_eq!(encoding, AssetEncoding::Brotli);
    /// assert_eq!(extension, ".br");
    ///
    /// let (encoding, extension) = AssetEncoding::Zstd.default_config();
    /// assert_eq!(encoding, AssetEncoding::Zstd);
    /// assert_eq!(extension, ".zst");
    ///
    /// let (encoding, extension) = AssetEncoding::Gzip.default_config();
    /// assert_eq!(encoding, AssetEncoding::Gzip);
    /// assert_eq!(extension, ".gz");
    ///
    /// let (encoding, extension) = AssetEncoding::Deflate.default_config();
    /// assert_eq!(encoding, AssetEncoding::Deflate);
    /// assert_eq!(extension, ".zz");
    ///
    /// let (encoding, extension) = AssetEncoding::Identity.default_config();
    /// assert_eq!(encoding, AssetEncoding::Identity);
    /// assert_eq!(extension, "");
    /// ```
    pub fn default_config(self) -> (AssetEncoding, String) {
        let file_extension = match self {
            AssetEncoding::Identity => "".to_string(),
            AssetEncoding::Brotli => ".br".to_string(),
            AssetEncoding::Zstd => ".zst".to_string(),
            AssetEncoding::Gzip => ".gz".to_string(),
            AssetEncoding::Deflate => ".zz".to_string(),
        };

        (self, file_extension)
    }

    /// Returns an encoding with a custom file extension. This is useful
    /// when the default file extension assigned by [default_config](AssetEncoding::default_config)
    /// is not desired.
    ///
    /// # Examples
    ///
    /// ```
    /// use ic_asset_certification::AssetEncoding;
    ///
    /// let (encoding, extension) = AssetEncoding::Brotli.custom_config("brotli".to_string());
    ///
    /// assert_eq!(encoding, AssetEncoding::Brotli);
    /// assert_eq!(extension, "brotli");
    /// ```
    pub fn custom_config(self, extension: String) -> (AssetEncoding, String) {
        (self, extension)
    }
}

impl Display for AssetEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AssetEncoding::Identity => "identity".to_string(),
            AssetEncoding::Brotli => "br".to_string(),
            AssetEncoding::Zstd => "zstd".to_string(),
            AssetEncoding::Gzip => "gzip".to_string(),
            AssetEncoding::Deflate => "deflate".to_string(),
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NormalizedAssetConfig {
    File {
        path: String,
        content_type: Option<String>,
        headers: Vec<(String, String)>,
        fallback_for: Vec<AssetFallbackConfig>,
        aliased_by: Vec<String>,
        encodings: Vec<(AssetEncoding, String)>,
    },
    Pattern {
        pattern: GlobMatcher,
        content_type: Option<String>,
        headers: Vec<(String, String)>,
        encodings: Vec<(AssetEncoding, String)>,
    },
    Redirect {
        from: String,
        to: String,
        kind: AssetRedirectKind,
        headers: Vec<(String, String)>,
    },
}

impl TryFrom<AssetConfig> for NormalizedAssetConfig {
    type Error = AssetCertificationError;

    fn try_from(config: AssetConfig) -> Result<Self, Self::Error> {
        match config {
            AssetConfig::File {
                path,
                content_type,
                headers,
                fallback_for,
                aliased_by,
                encodings,
            } => Ok(NormalizedAssetConfig::File {
                path,
                content_type,
                headers,
                fallback_for,
                aliased_by,
                encodings,
            }),
            AssetConfig::Pattern {
                pattern,
                content_type,
                headers,
                encodings,
            } => Ok(NormalizedAssetConfig::Pattern {
                pattern: Glob::new(&pattern)?.compile_matcher(),
                content_type,
                headers,
                encodings,
            }),
            AssetConfig::Redirect {
                from,
                to,
                kind,
                headers,
            } => Ok(NormalizedAssetConfig::Redirect {
                from,
                to,
                kind,
                headers,
            }),
        }
    }
}

impl NormalizedAssetConfig {
    pub(crate) fn matches_asset(&self, asset: &Asset) -> bool {
        match self {
            Self::File { path, .. } => path == asset.path.as_ref(),
            Self::Pattern { pattern, .. } => pattern.is_match(asset.path.as_ref()),
            Self::Redirect { .. } => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Asset;
    use rstest::*;

    #[rstest]
    #[case("index.html", "index.html", true)]
    #[case("app.js", "app.js", true)]
    #[case("index.js", "app.css", false)]
    #[case("index.css", "index.js", false)]
    fn matches_asset_file(
        #[case] asset_path: &str,
        #[case] config_path: &str,
        #[case] expected: bool,
    ) {
        let asset = Asset::new(asset_path, vec![]);
        let config: NormalizedAssetConfig = AssetConfig::File {
            path: config_path.to_string(),
            content_type: None,
            headers: vec![],
            fallback_for: vec![],
            aliased_by: vec![],
            encodings: vec![],
        }
        .try_into()
        .unwrap();

        assert_eq!(config.matches_asset(&asset), expected);
    }

    #[rstest]
    // index.html, *
    #[case("index.html", "*", true)]
    #[case("index.html", "**", true)]
    #[case("index.html", "**/*", true)]
    #[case("index.html", "**/**", true)]
    // app.js, *
    #[case("app.js", "*", true)]
    #[case("app.js", "**", true)]
    #[case("app.js", "**/*", true)]
    #[case("app.js", "**/**", true)]
    // index.html, *.html
    #[case("index.html", "*.html", true)]
    #[case("index.html", "**.html", true)]
    #[case("index.html", "**/*.html", true)]
    #[case("index.html", "**/**.html", true)]
    // app.js, *.html
    #[case("app.js", "*.html", false)]
    #[case("app.js", "**.html", false)]
    #[case("app.js", "**/*.html", false)]
    #[case("app.js", "**/**.html", false)]
    // app.js, *.js
    #[case("app.js", "*.js", true)]
    #[case("app.js", "**.js", true)]
    #[case("app.js", "**/*.js", true)]
    #[case("app.js", "**/**.js", true)]
    // index.html, *.{js,html}
    #[case("index.html", "*.{js,html}", true)]
    #[case("index.html", "**.{js,html}", true)]
    #[case("index.html", "**/*.{js,html}", true)]
    #[case("index.html", "**/**.{js,html}", true)]
    // app.js, *.{js,html}
    #[case("app.js", "*.{js,html}", true)]
    #[case("app.js", "**.{js,html}", true)]
    #[case("app.js", "**/*.{js,html}", true)]
    #[case("app.js", "**/**.{js,html}", true)]
    // index.html, assets/*.html
    #[case("index.html", "assets/*.html", false)]
    #[case("index.html", "assets/**.html", false)]
    #[case("index.html", "assets/**/*.html", false)]
    // app.js, assets/*.js
    #[case("app.js", "assets/*.js", false)]
    #[case("app.js", "assets/**.js", false)]
    #[case("app.js", "assets/**/*.js", false)]
    // assets/index.html, *
    #[case("assets/index.html", "*", true)]
    #[case("assets/index.html", "**", true)]
    #[case("assets/index.html", "**/*", true)]
    #[case("assets/index.html", "**/**", true)]
    // assets/app.js, *
    #[case("assets/app.js", "*", true)]
    #[case("assets/app.js", "**", true)]
    #[case("assets/app.js", "**/*", true)]
    #[case("assets/app.js", "**/**", true)]
    // assets/index.html, *.html
    #[case("assets/index.html", "*.html", true)]
    #[case("assets/index.html", "**.html", true)]
    #[case("assets/index.html", "**/*.html", true)]
    // assets/app.js, *.js
    #[case("assets/app.js", "*.js", true)]
    #[case("assets/app.js", "**.js", true)]
    #[case("assets/app.js", "**/*.js", true)]
    // assets/index.html, assets/*.html
    #[case("assets/index.html", "assets/*.html", true)]
    #[case("assets/index.html", "assets/**.html", true)]
    #[case("assets/index.html", "assets/**/*.html", true)]
    // assets/app.js, assets/*.js
    #[case("assets/app.js", "assets/*.js", true)]
    #[case("assets/app.js", "assets/**.js", true)]
    #[case("assets/app.js", "assets/**/*.js", true)]
    // assets/index.html, assets/*.{js,html}
    #[case("assets/index.html", "assets/*.{js,html}", true)]
    #[case("assets/index.html", "assets/**.{js,html}", true)]
    #[case("assets/index.html", "assets/**/*.{js,html}", true)]
    #[case("assets/index.html", "assets/**/**.{js,html}", true)]
    // assets/app.js, assets/*.{js,html}
    #[case("assets/app.js", "assets/*.{js,html}", true)]
    #[case("assets/app.js", "assets/**.{js,html}", true)]
    #[case("assets/app.js", "assets/**/*.{js,html}", true)]
    #[case("assets/app.js", "assets/**/**.{js,html}", true)]
    fn matches_asset_pattern(
        #[case] asset_path: &str,
        #[case] config_pattern: &str,
        #[case] expected: bool,
    ) {
        let asset = Asset::new(asset_path, vec![]);
        let config: NormalizedAssetConfig = AssetConfig::Pattern {
            pattern: config_pattern.to_string(),
            content_type: None,
            headers: vec![],
            encodings: vec![],
        }
        .try_into()
        .unwrap();

        assert_eq!(config.matches_asset(&asset), expected);
    }

    #[rstest]
    #[case("index.html")]
    #[case("app.js")]
    #[case("index.js")]
    #[case("index.css")]
    fn does_not_match_asset_redirect(#[case] asset_path: &str) {
        let asset = Asset::new(asset_path, vec![]);
        let config: NormalizedAssetConfig = AssetConfig::Redirect {
            from: asset_path.to_string(),
            to: asset_path.to_string(),
            kind: AssetRedirectKind::Permanent,
            headers: vec![(
                "content-type".to_string(),
                "text/plain; charset=utf-8".to_string(),
            )],
        }
        .try_into()
        .unwrap();

        assert!(!config.matches_asset(&asset));
    }

    #[rstest]
    fn asset_encoding_to_string() {
        assert_eq!(AssetEncoding::Brotli.to_string(), "br");
        assert_eq!(AssetEncoding::Zstd.to_string(), "zstd");
        assert_eq!(AssetEncoding::Gzip.to_string(), "gzip");
        assert_eq!(AssetEncoding::Deflate.to_string(), "deflate");
        assert_eq!(AssetEncoding::Identity.to_string(), "identity");
    }
}
