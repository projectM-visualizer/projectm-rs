//! ProjectM for Rust
//!
//! This library contains bindings to libprojectm. Its purpose
//! is to read an audio input and to produce mesmerizing visuals
//! by detecting tempo, and rendering advanced equations into a
//! limitless array of user-contributed visualizations.
//!
//! # Example
//!
//! ```
//! use projectm_rs::core::*;
//!
//! let projectm_handle = projectm::create();
//! ```
//!

#![allow(non_camel_case_types)]

extern crate libc;
extern crate projectm_sys as ffi;

use std::ffi::{CString};

pub enum projectm {}
pub type projectm_handle = *mut ffi::projectm;

pub type projectm_channels = u32;
pub const MONO: projectm_channels = 1;
pub const STEREO: projectm_channels = 2;

pub type projectm_touch_type = u32;
pub const TOUCH_TYPE_RANDOM: projectm_touch_type = 0;
pub const TOUCH_TYPE_CIRCLE: projectm_touch_type = 1;
pub const TOUCH_TYPE_RADIAL_BLOB: projectm_touch_type = 2;
pub const TOUCH_TYPE_BLOB2: projectm_touch_type = 3;
pub const TOUCH_TYPE_BLOB3: projectm_touch_type = 4;
pub const TOUCH_TYPE_DERIVATIVE_LINE: projectm_touch_type = 5;
pub const TOUCH_TYPE_BLOB5: projectm_touch_type = 6;
pub const TOUCH_TYPE_LINE: projectm_touch_type = 7;
pub const TOUCH_TYPE_DOUBLE_LINE: projectm_touch_type = 8;

impl projectm {
    // -----------------
    // Core
    // -----------------

    pub fn create() -> *mut ffi::projectm {
        return unsafe { ffi::projectm_create() };
    }

    pub fn destroy(instance: projectm_handle) {
        unsafe { ffi::projectm_destroy(instance) };
    }

    pub fn load_preset_file(instance: projectm_handle, filename: &String, smooth_transition: bool) {
        unsafe {
            ffi::projectm_load_preset_file(
                instance,
                filename.as_ptr() as *mut i8,
                smooth_transition,
            )
        };
    }

    pub fn load_preset_data(instance: projectm_handle, data: &String, smooth_transition: bool) {
        unsafe {
            ffi::projectm_load_preset_data(
                instance,
                data.as_ptr() as *mut i8,
                smooth_transition,
            )
        };
    }

    pub fn reset_textures(instance: projectm_handle) {
        unsafe { ffi::projectm_reset_textures(instance) };
    }

    pub fn get_version_components() -> (i32, i32, i32) {
        #[derive(Debug, Default, Copy, Clone)]
        #[repr(C, packed)]
        struct Version {
            major: i32,
            minor: i32,
            patch: i32,
        }

        let mut version = Version::default();

        unsafe {
            ffi::projectm_get_version_components(
                std::ptr::addr_of_mut!(version.major),
                std::ptr::addr_of_mut!(version.minor),
                std::ptr::addr_of_mut!(version.patch),
            );
        }

        return (version.major, version.minor, version.patch);
    }

    pub fn get_version_string() -> String {
        let get_version = unsafe { ffi::projectm_get_version_string() };
        let version_str = unsafe { std::ffi::CStr::from_ptr(get_version) };
        let version_str_slice = version_str.to_str().unwrap();
        let version = version_str_slice.to_owned();

        unsafe { ffi::projectm_free_string(get_version) };

        return version;
    }

    pub fn get_vcs_version_string() -> String {
        let get_vcs_version = unsafe { ffi::projectm_get_vcs_version_string() };
        let vcs_version_str = unsafe { std::ffi::CStr::from_ptr(get_vcs_version) };
        let vcs_version_str_slice = vcs_version_str.to_str().unwrap();
        let vcs_version = vcs_version_str_slice.to_owned();

        unsafe { ffi::projectm_free_string(get_vcs_version) };

        return vcs_version;
    }

    // -----------------
    // Callbacks
    // -----------------

    pub fn set_preset_switch_requested_event_callback<F: FnMut(bool)>(
        instance: projectm_handle,
        callback: F,
    ) {
        unsafe extern "C" fn trampoline<F: FnMut(bool)>(
            is_hard_cut: bool,
            user_data: *mut std::os::raw::c_void,
        ) {
            unsafe { (*user_data.cast::<F>())(is_hard_cut) }
        }
        unsafe {
            ffi::projectm_set_preset_switch_requested_event_callback(
                instance,
                Some(trampoline::<F>),
                (Box::leak(Box::new(callback)) as *mut F).cast::<std::os::raw::c_void>(),
            )
        }
    }

    pub fn set_preset_switch_failed_event_callback<F: FnMut(String, String)>(
        instance: projectm_handle,
        callback: F,
    ) {
        unsafe extern "C" fn trampoline<F: FnMut(String, String)>(
            preset_filename: *const i8,
            message: *const i8,
            user_data: *mut std::os::raw::c_void,
        ) {
            let preset_filename_str = unsafe { std::ffi::CStr::from_ptr(preset_filename) };
            let preset_filename_str_slice = preset_filename_str.to_str().unwrap();
            let preset_filename = preset_filename_str_slice.to_owned();

            let message_str = unsafe { std::ffi::CStr::from_ptr(message) };
            let message_str_slice = message_str.to_str().unwrap();
            let message = message_str_slice.to_owned();
            unsafe { (*user_data.cast::<F>())(preset_filename, message) }
        }
        unsafe {
            ffi::projectm_set_preset_switch_failed_event_callback(
                instance,
                Some(trampoline::<F>),
                (Box::leak(Box::new(callback)) as *mut F).cast::<std::os::raw::c_void>(),
            )
        }
    }

    // -----------------
    // Parameters
    // -----------------

    pub fn set_texture_search_paths(
        instance: projectm_handle,
        texture_search_paths: Vec<String>,
        count: usize,
    ) {
        let texture_search_paths_cstr: Vec<_> = texture_search_paths
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();

        let mut texture_search_paths_pointer: Vec<_> = texture_search_paths_cstr
            .iter() // do NOT into_iter()
            .map(|arg| arg.as_ptr())
            .collect();

        texture_search_paths_pointer.push(std::ptr::null());

        unsafe {
            ffi::projectm_set_texture_search_paths(
                instance,
                texture_search_paths_pointer.as_ptr() as *mut *const ::std::os::raw::c_char,
                count,
            )
        };
    }

    pub fn get_beat_sensitivity(instance: projectm_handle) -> f32 {
        return unsafe { ffi::projectm_get_beat_sensitivity(instance) };
    }

    pub fn set_beat_sensitivity(instance: projectm_handle, sensitivity: f32) {
        unsafe { ffi::projectm_set_beat_sensitivity(instance, sensitivity) };
    }

    pub fn get_hard_cut_duration(instance: projectm_handle) -> f64 {
        return unsafe { ffi::projectm_get_hard_cut_duration(instance) };
    }

    pub fn set_hard_cut_duration(instance: projectm_handle, seconds: f64) {
        unsafe { ffi::projectm_set_hard_cut_duration(instance, seconds) };
    }

    pub fn get_hard_cut_enabled(instance: projectm_handle) -> bool {
        return unsafe { ffi::projectm_get_hard_cut_enabled(instance) };
    }

    pub fn set_hard_cut_enabled(instance: projectm_handle, enabled: bool) {
        unsafe { ffi::projectm_set_hard_cut_enabled(instance, enabled) }
    }

    pub fn get_hard_cut_sensitivity(instance: projectm_handle) -> f32 {
        return unsafe { ffi::projectm_get_hard_cut_sensitivity(instance) };
    }

    pub fn set_hard_cut_sensitivity(instance: projectm_handle, sensitivity: f32) {
        unsafe { ffi::projectm_set_hard_cut_sensitivity(instance, sensitivity) }
    }

    pub fn get_soft_cut_duration(instance: projectm_handle) -> f64 {
        return unsafe { ffi::projectm_get_soft_cut_duration(instance) };
    }

    pub fn set_soft_cut_duration(instance: projectm_handle, seconds: f64) {
        unsafe { ffi::projectm_set_soft_cut_duration(instance, seconds) }
    }

    pub fn get_preset_duration(instance: projectm_handle) -> f64 {
        unsafe { ffi::projectm_get_preset_duration(instance) }
    }

    pub fn set_preset_duration(instance: projectm_handle, seconds: f64) {
        unsafe { ffi::projectm_set_preset_duration(instance, seconds) }
    }

    pub fn get_mesh_size(instance: projectm_handle) -> (usize, usize) {
        #[derive(Debug, Default, Copy, Clone)]
        #[repr(C, packed)]
        struct Mesh {
            mesh_x: usize,
            mesh_y: usize,
        }

        let mut mesh = Mesh::default();

        unsafe {
            ffi::projectm_get_mesh_size(
                instance,
                std::ptr::addr_of_mut!(mesh.mesh_x),
                std::ptr::addr_of_mut!(mesh.mesh_y),
            );
        }

        return (mesh.mesh_x, mesh.mesh_y);
    }

    pub fn set_mesh_size(instance: projectm_handle, mesh_x: usize, mesh_y: usize) {
        unsafe {
            ffi::projectm_set_mesh_size(instance, mesh_x, mesh_y);
        }
    }

    pub fn get_fps(instance: projectm_handle) -> usize {
        return unsafe { ffi::projectm_get_fps(instance) } as usize;
    }

    pub fn set_fps(instance: projectm_handle, fps: u32) {
        unsafe { ffi::projectm_set_fps(instance, fps as i32) };
    }

    pub fn get_aspect_correction(instance: projectm_handle) -> bool {
        return unsafe { ffi::projectm_get_aspect_correction(instance) };
    }

    pub fn set_aspect_correction(instance: projectm_handle, enabled: bool) {
        unsafe { ffi::projectm_set_aspect_correction(instance, enabled) };
    }

    pub fn get_easter_egg(instance: projectm_handle) -> f32 {
        return unsafe { ffi::projectm_get_easter_egg(instance) };
    }

    pub fn set_easter_egg(instance: projectm_handle, sensitivity: f32) {
        unsafe { ffi::projectm_set_easter_egg(instance, sensitivity) };
    }

    pub fn get_preset_locked(instance: projectm_handle) -> bool {
        return unsafe { ffi::projectm_get_preset_locked(instance) };
    }

    pub fn set_preset_locked(instance: projectm_handle, lock: bool) {
        unsafe { ffi::projectm_set_preset_locked(instance, lock) };
    }

    pub fn get_window_size(instance: projectm_handle) -> (usize, usize) {
        #[derive(Debug, Default, Copy, Clone)]
        #[repr(C, packed)]
        struct Mesh {
            width: usize,
            height: usize,
        }

        let mut window = Mesh::default();

        unsafe {
            ffi::projectm_get_window_size(
                instance,
                std::ptr::addr_of_mut!(window.width),
                std::ptr::addr_of_mut!(window.height),
            );
        }

        return (window.width, window.height);
    }

    pub fn set_window_size(instance: projectm_handle, width: usize, height: usize) {
        unsafe { ffi::projectm_set_window_size(instance, width, height) };
    }

    // -----------------
    // Render OpenGL
    // -----------------

    pub fn render_frame(instance: projectm_handle) {
        unsafe { ffi::projectm_opengl_render_frame(instance) };
    }

    pub fn init_render_to_texture(instance: projectm_handle) -> u32 {
        return unsafe { ffi::projectm_opengl_init_render_to_texture(instance) };
    }

    // -----------------
    // Touch
    // -----------------

    pub fn touch(
        instance: projectm_handle,
        x: f32,
        y: f32,
        pressure: i32,
        touch_type: projectm_touch_type,
    ) {
        unsafe { ffi::projectm_touch(instance, x, y, pressure, touch_type) };
    }

    pub fn touch_drag(instance: projectm_handle, x: f32, y: f32, pressure: i32) {
        unsafe { ffi::projectm_touch_drag(instance, x, y, pressure) };
    }

    pub fn touch_destroy(instance: projectm_handle, x: f32, y: f32) {
        unsafe { ffi::projectm_touch_destroy(instance, x, y) };
    }

    pub fn touch_destroy_all(instance: projectm_handle) {
        unsafe { ffi::projectm_touch_destroy_all(instance) };
    }

    // -----------------
    // Audio
    // -----------------

    pub fn pcm_get_max_samples() -> u32 {
        return unsafe { ffi::projectm_pcm_get_max_samples() };
    }

    pub fn pcm_add_float(
        instance: projectm_handle,
        samples: *const f32,
        count: u32,
        channels: projectm_channels,
    ) {
        unsafe { ffi::projectm_pcm_add_float(instance, samples, count, channels) }
    }

    pub fn pcm_add_int16(
        instance: projectm_handle,
        samples: *const i16,
        count: u32,
        channels: projectm_channels,
    ) {
        unsafe { ffi::projectm_pcm_add_int16(instance, samples, count, channels) }
    }

    pub fn pcm_add_uint8(
        instance: projectm_handle,
        samples: *const u8,
        count: u32,
        channels: projectm_channels,
    ) {
        unsafe { ffi::projectm_pcm_add_uint8(instance, samples, count, channels) }
    }

    // -----------------
    // Debug
    // -----------------

    // Figure out how to make an argument optional
    pub fn write_debug_image_on_next_frame(instance: projectm_handle, output_file: Option<&String>) {
        unsafe { ffi::projectm_write_debug_image_on_next_frame(instance, output_file.unwrap().as_ptr() as *mut i8) };
    }
}
