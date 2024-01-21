//
// SPDX: MIT OR Apache-2.0
//
pub use colorsys::{Hsl, Rgb};
use {
    byteorder::{BigEndian, ReadBytesExt},
    hmac_sha256::Hash,
    std::{io::Cursor, ops::Range},
};

fn rgb_hash(key: &str) -> usize {
    Cursor::new(Hash::hash(key.as_bytes()))
        .read_u32::<BigEndian>()
        .expect("Hash is too small") // always succeds for sha256!
         as usize
}

/// Convert a string to its color representation using a hash function.
pub struct ColorHash {
    s: Vec<f64>,
    l: Vec<f64>,
    hue_ranges: Vec<Range<f64>>,
}

impl Default for ColorHash {
    /// Create a default instance.
    ///
    /// In the default variant, there are no hue ranges, standard saturation and lightness ranges.
    fn default() -> Self {
        Self {
            s: vec![35., 50., 65.], // note that length 3 is a prime
            l: vec![35., 50., 65.], // note that length 3 is a prime
            hue_ranges: vec![],
        }
    }
}

impl ColorHash {

    /// Returns the hash in HSL.
    ///
    /// Note that H ∈ [0, 360); S ∈ [0, 100]; L ∈ [0, 100];
    pub fn hsl(&self, input: &str) -> Hsl {
        let hash = rgb_hash(input);
        let hue_resolution = 727; // note that 727 is a prime

        let h = if self.hue_ranges.len() > 0 {
            let range = &self.hue_ranges[hash % self.hue_ranges.len()];
            ((hash / self.hue_ranges.len()) % hue_resolution) as f64 * (range.end - range.start)
                / hue_resolution as f64
                + range.start
        } else {
            (hash % 359) as f64 // note that 359 is a prime
        };
        let s = self.s[(hash / 360) % self.s.len()];
        let l = self.l[(hash / 360 / self.s.len()) % self.l.len()];

        Hsl::new(h as f64, s, l, None)
    }

    /// Returns the hash in RGB.
    ///
    /// Note that R ∈ [0, 255); G ∈ [0, 255); B ∈ [0, 255];
    pub fn rgb(&self, input: &str) -> Rgb {
        self.hsl(input).into()
    }

    /// Returns the hash in HTML-style hex string.
    ///
    /// You could also generate CSS style RGB string using `rgb(input).to_css_string().`
    pub fn hex(&self, input: &str) -> String {
        self.rgb(input).to_hex_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashing() {
        assert_eq!(rgb_hash("hello world"), 3108841401);
        assert_eq!(rgb_hash("a"), 3398926610);
        assert_eq!(rgb_hash("b"), 1042540566);
        assert_eq!(rgb_hash("c"), 779955203);
    }

    #[test]
    fn hsl_colors() {
        let ch = ColorHash::new();
        assert_eq!(ch.hsl("hello world"), Hsl::new(126.0, 65., 65., None));
        assert_eq!(ch.hsl("a"), Hsl::new(52.0, 35., 50., None));
        assert_eq!(ch.hsl("b"), Hsl::new(258.0, 50., 65., None));
        assert_eq!(ch.hsl("c"), Hsl::new(60.0, 65., 65., None));
    }
}
