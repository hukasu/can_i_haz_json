//! Attempt to parse each of the glTF 2.0 models provided by Khronos Groups
//! on the https://github.com/KhronosGroup/glTF-Sample-Models repository.

use can_i_haz_gltf::Gltf;
use can_i_haz_gltf_test_macro::make_gltf_tests;

make_gltf_tests!("gltf-sample-models/2.0");
