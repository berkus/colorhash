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
#[derive(Clone, Debug, PartialEq)]
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
    /// Create a new instance.
    ///
    /// This creates just a default instance that you can adjust with more builder methods.
    ///
    /// See also [`Self::lightness`], [`Self::lightness_vec`], [`Self::saturation`], [`Self::saturation_vec`], [`Self::hue_range`] and [`Self::hue_ranges`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Set a non-default lightness value.
    ///
    /// There will be only one lightness to pick from, so all generated colors will have it.
    ///
    /// See also [`Self::new`], [`Self::lightness_vec`], [`Self::saturation`], [`Self::saturation_vec`], [`Self::hue_range`] and [`Self::hue_ranges`].
    pub fn lightness(self, lightness: f64) -> Self {
        Self {
            s: self.s,
            l: vec![lightness],
            hue_ranges: self.hue_ranges,
        }
    }

    /// Set a non-default array of lightness values.
    ///
    /// The generated colors will pick one of these values for lightness.
    /// For better distribution make sure the vector size is a prime.
    ///
    /// See also [`Self::new`], [`Self::lightness`], [`Self::saturation`], [`Self::saturation_vec`], [`Self::hue_range`] and [`Self::hue_ranges`].
    pub fn lightness_vec(self, lightness: &Vec<f64>) -> Self {
        Self {
            s: self.s,
            l: lightness.to_owned(),
            hue_ranges: self.hue_ranges,
        }
    }

    /// Set a non-default saturation value.
    ///
    /// There will be only one saturation to pick from, so all generated colors will have it.
    ///
    /// See also [`Self::new`], [`Self::lightness`], [`Self::lightness_vec`], [`Self::saturation_vec`], [`Self::hue_range`] and [`Self::hue_ranges`].
    pub fn saturation(self, saturation: f64) -> Self {
        Self {
            s: vec![saturation],
            l: self.l,
            hue_ranges: self.hue_ranges,
        }
    }

    /// Set a non-default array of saturation values.
    ///
    /// The generated colors will pick one of these values for saturation.
    /// For better distribution make sure the vector size is a prime.
    ///
    /// See also [`Self::new`], [`Self::lightness`], [`Self::lightness_vec`], [`Self::saturation`], [`Self::hue_range`] and [`Self::hue_ranges`].
    pub fn saturation_vec(self, saturation: &Vec<f64>) -> Self {
        Self {
            s: saturation.to_owned(),
            l: self.l,
            hue_ranges: self.hue_ranges,
        }
    }

    /// Set a non-default hue range.
    ///
    /// Color hues will be picked from this single range.
    /// Note that hue range is defined in HSL from 0 to 360 degrees.
    ///
    /// See also [`Self::new`], [`Self::lightness`], [`Self::lightness_vec`], [`Self::saturation`], [`Self::saturation_vec`] and [`Self::hue_ranges`].
    pub fn hue_range(self, hue_range: Range<f64>) -> Self {
        Self {
            s: self.s,
            l: self.l,
            hue_ranges: vec![hue_range],
        }
    }

    /// Set a non-default array of hue ranges.
    ///
    /// The generated colors will pick a number from one of these ranges for hue.
    /// Note that each hue range is defined in HSL from 0 to 360 degrees. Ranges can be overlapping,
    /// which range is picked is decided solely by the hash value.
    ///
    /// See also [`Self::new`], [`Self::lightness`], [`Self::lightness_vec`], [`Self::saturation`], [`Self::saturation_vec`] and [`Self::hue_range`].
    pub fn hue_ranges(self, hue_ranges: &Vec<Range<f64>>) -> Self {
        Self {
            s: self.s,
            l: self.l,
            hue_ranges: hue_ranges.to_owned(),
        }
    }

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
    use {super::*, float_eq::assert_float_eq, nanoid::nanoid};

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

    #[test]
    fn should_return_the_hash_color_based_on_default_hue() {
        let hash = ColorHash::default();
        for _ in 0..100 {
            let hue = hash.hsl(&nanoid!()).hue();
            assert!(hue >= 0.0 && hue < 359.0); // hash % 359 means max 358
        }
    }

    #[test]
    fn should_return_the_hash_color_based_on_given_hue_value() {
        let hash = ColorHash::new().hue_range(10.0..10.0);
        for _ in 0..100 {
            let hue = hash.hsl(&nanoid!()).hue();
            assert_float_eq!(hue, 10.0, abs_all <= 0.05);
        }
    }

    #[test]
    fn should_return_the_hash_color_based_on_given_hue_range() {
        for min in (0..361).step_by(60) {
            for max in (min + 1..361).step_by(60) {
                let hash = ColorHash::new().hue_range(min as f64..max as f64);
                for _ in 0..100 {
                    let hue = hash.hsl(&nanoid!()).hue();
                    assert!(hue >= min as f64 && hue < max as f64);
                }
            }
        }
    }

    #[test]
    fn should_work_for_multiple_hue_ranges() {
        let ranges = vec![30.0..90.0, 180.0..210.0, 270.0..285.0];
        let hash = ColorHash::new().hue_ranges(&ranges);
        for _ in 0..100 {
            let hue = hash.hsl(&nanoid!()).hue();
            assert!(ranges.iter().any(|r| hue >= r.start && hue < r.end));
        }
    }

    #[test]
    fn should_return_color_based_on_given_lightness_and_saturation() {
        let hash = ColorHash::new().lightness(50.0).saturation(50.0);
        for _ in 0..100 {
            let hsl = hash.hsl(&nanoid!());
            assert_float_eq!(hsl.saturation(), 50.0, ulps_all <= 1);
            assert_float_eq!(hsl.lightness(), 50.0, ulps_all <= 1);
        }
    }

    #[test]
    fn should_return_the_hash_color_based_on_given_lightness_array_and_saturation_array() {
        let hash = ColorHash::new()
            .lightness_vec(&vec![90.0, 100.0])
            .saturation_vec(&vec![90.0, 100.0]);
        for _ in 0..100 {
            let hsl = hash.hsl(&nanoid!());
            assert!([90.0, 100.0].contains(&hsl.saturation()));
            assert!([90.0, 100.0].contains(&hsl.lightness()));
        }
    }
}
