//! A tiny sparkle of love.
//!
//! This is the Rust port of the `@mirql/love-sparkle` npm package: it holds
//! the exact same sparkle text, byte for byte.
//!
//! ```
//! assert_eq!(mirql_love_sparkle::sparkle_of_love(), "✨❤️ Sparkle of Love ❤️✨");
//! ```

#![forbid(unsafe_code)]

/// The sparkle of love text, identical byte for byte to the one produced by
/// the `@mirql/love-sparkle` npm package's `sparkleOfLove()`.
pub const SPARKLE_OF_LOVE: &str = "✨❤️ Sparkle of Love ❤️✨";

/// Returns the sparkle of love text.
///
/// # Examples
///
/// ```
/// use mirql_love_sparkle::sparkle_of_love;
///
/// assert_eq!(sparkle_of_love(), "✨❤️ Sparkle of Love ❤️✨");
/// ```
pub fn sparkle_of_love() -> &'static str {
    SPARKLE_OF_LOVE
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The npm package `@mirql/love-sparkle` (version 0.6.11881) exports
    /// `sparkleOfLove()`, which returns this exact text. Its compiled
    /// `esm/mod.js` contains the literal
    /// `'✨❤️ Sparkle of Love ❤️✨'` with these exact UTF-8 bytes:
    /// `e2 9c a8 e2 9d a4 ef b8 8f 20 .. 20 e2 9d a4 ef b8 8f e2 9c a8`.
    const NPM_SPARKLE_OF_LOVE_BYTES: &[u8] = &[
        0xe2, 0x9c, 0xa8, // ✨
        0xe2, 0x9d, 0xa4, 0xef, 0xb8, 0x8f, // ❤️
        0x20, // space
        b'S', b'p', b'a', b'r', b'k', b'l', b'e', b' ', b'o', b'f', b' ', b'L', b'o', b'v', b'e',
        0x20, // space
        0xe2, 0x9d, 0xa4, 0xef, 0xb8, 0x8f, // ❤️
        0xe2, 0x9c, 0xa8, // ✨
    ];

    #[test]
    fn matches_npm_package_bytes() {
        assert_eq!(SPARKLE_OF_LOVE.as_bytes(), NPM_SPARKLE_OF_LOVE_BYTES);
    }

    #[test]
    fn sparkle_of_love_returns_the_constant() {
        assert_eq!(sparkle_of_love(), SPARKLE_OF_LOVE);
    }
}
