#![cfg_attr(target_arch = "spirv", no_std)]
use spirv_std::glam::Vec4;
use spirv_std::spirv;

#[repr(C)]
pub struct MaterialColor {
    color: Vec4,
}

#[spirv(fragment)]
pub fn main_fs( #[spirv(uniform, descriptor_set = 2, binding = 0)] material_color: &MaterialColor,output: &mut Vec4) {
    *output = material_color.color;
}