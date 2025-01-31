// build-pass
// compile-flags: -C target-feature=+ImageQuery

use spirv_std::{arch, Image, Sampler};

#[spirv(fragment)]
pub fn main(
    #[spirv(descriptor_set = 0, binding = 0)] image: &Image!(2D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    output: &mut glam::Vec2,
) {
    *output = image.query_lod(*sampler, glam::Vec2::new(0.0, 1.0));
}
