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
//! use projectm_rs::*;
//! 
//! let settings = Settings {
//!     mesh_x: 96,
//!     mesh_y: 54,
//!     fps: 30,
//!     texture_size: 512,
//!     window_width: 1280,
//!     window_height: 720,
//!     preset_duration: 15.0,
//!     soft_cut_duration: 15.0,
//!     hard_cut_duration: 60.0,
//!     hard_cut_enabled: false,
//!     hard_cut_sensitivity: 0.0,
//!     beat_sensitivity: 0.5,
//!     aspect_correction: true,
//!     easter_egg: 0.5,
//!     shuffle_enabled: true,
//!     soft_cut_ratings_enabled: true,
//!     preset_path: String::from("./presets"),
//!     texture_path: String::from("./textures"),
//!     data_path: String::from("./"),
//!     ;
//!
//! let projectm_handle = projectm::create(&settings, 0);
//! ```
//! 

#![allow(non_camel_case_types)]

extern crate libc;
extern crate projectm_sys as ffi;

use std::ffi::CStr;
use libc::{c_int, strncpy};

pub enum projectm {}
pub type projectm_handle = *mut ffi::projectm;

#[repr(C)]
#[derive(Debug)]
pub struct Settings {
    pub mesh_x: u32,
    pub mesh_y: u32,
    pub fps: u32,
    pub texture_size: u32,
    pub window_width: u32,
    pub window_height: u32,
    pub texture_path: String,
    pub data_path: String,
    pub preset_duration: f64,
    pub soft_cut_duration: f64,
    pub hard_cut_duration: f64,
    pub hard_cut_enabled: bool,
    pub hard_cut_sensitivity: f32,
    pub beat_sensitivity: f32,
    pub aspect_correction: bool,
    pub easter_egg: f32,
}
pub type settings = Settings;
pub type projectm_settings = ffi::projectm_settings;

// TODO: Callbacks - Prefered types
// pub type preset_switch_requested_event = Option<fn(is_hard_cut: bool, user_data: *mut ())>;
// pub type preset_switch_failed_event = Option<fn(filename: &String, message: &String, user_data: *mut ())>;

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
    pub fn create(settings: &settings) -> *mut ffi::projectm {

        let projectm_settings = projectm_settings {
            mesh_x: settings.mesh_x as c_int,
            mesh_y: settings.mesh_y as c_int,
            fps: settings.fps as c_int,
            texture_size: settings.texture_size as c_int,
            window_width: settings.window_width as c_int,
            window_height: settings.window_height as c_int,
            texture_path: unsafe { ffi::projectm_alloc_string((settings.texture_path.len() + 1).try_into().unwrap()) },
            data_path: unsafe { ffi::projectm_alloc_string((settings.data_path.len() + 1).try_into().unwrap()) },
            preset_duration: settings.preset_duration,
            soft_cut_duration: settings.soft_cut_duration,
            hard_cut_duration: settings.hard_cut_duration,
            hard_cut_enabled: settings.hard_cut_enabled,
            hard_cut_sensitivity: settings.hard_cut_sensitivity,
            beat_sensitivity: settings.beat_sensitivity,
            aspect_correction: settings.aspect_correction,
            easter_egg: settings.easter_egg,
        };

        unsafe {
            strncpy(projectm_settings.texture_path, settings.texture_path.as_bytes().as_ptr() as *mut i8, settings.texture_path.len());
            strncpy(projectm_settings.data_path, settings.data_path.as_bytes().as_ptr() as *mut i8, settings.data_path.len());
        }
        

        return unsafe { ffi::projectm_create_settings(&projectm_settings) };
    }
    
    pub fn create_from_file(settings_path: &String) -> *mut ffi::projectm {
        return unsafe { ffi::projectm_create(settings_path.as_bytes().as_ptr() as *mut i8) };
    }

    pub fn destroy(instance: projectm_handle) {
        unsafe { ffi::projectm_destroy(instance) };
    }

    pub fn get_settings(instance: projectm_handle) -> Settings {
        let projectm_settings_buf = unsafe { ffi::projectm_get_settings(instance) };
        let projectm_settings_ref = unsafe{ projectm_settings_buf.as_ref().unwrap() };
            
        let projectm_settings = Settings {
            mesh_x: projectm_settings_ref.mesh_x as u32,
            mesh_y: projectm_settings_ref.mesh_y as u32,
            fps: projectm_settings_ref.fps as u32,
            texture_size: projectm_settings_ref.texture_size as u32,
            window_width: projectm_settings_ref.window_width as u32,
            window_height: projectm_settings_ref.window_height as u32,
            texture_path: projectm::get_texture_path(instance),
            data_path: projectm::get_data_path(instance),
            preset_duration: projectm_settings_ref.preset_duration,
            soft_cut_duration: projectm_settings_ref.soft_cut_duration,
            hard_cut_duration: projectm_settings_ref.hard_cut_duration,
            hard_cut_enabled: projectm_settings_ref.hard_cut_enabled,
            hard_cut_sensitivity: projectm_settings_ref.hard_cut_sensitivity,
            beat_sensitivity: projectm_settings_ref.beat_sensitivity,
            aspect_correction: projectm_settings_ref.aspect_correction,
            easter_egg: projectm_settings_ref.easter_egg,
        };
        
        return projectm_settings;
    }

    // -----------------

    pub fn load_preset_file(instance: projectm_handle, filename: &String, smooth_transition: bool) {
        unsafe { ffi::projectm_load_preset_file(instance, filename.as_bytes().as_ptr() as *mut i8, smooth_transition) };
    }

    pub fn load_preset_data(instance: projectm_handle, data: &String, smooth_transition: bool) {
        unsafe { ffi::projectm_load_preset_data(instance, data.as_bytes().as_ptr() as *mut i8, smooth_transition) };
    }

    pub fn set_preset_switch_requested_event_callback(instance: projectm_handle, callback: ffi::projectm_preset_switch_requested_event, user_data: *mut ()) {
        // TODO: Needs improving
        unsafe { ffi::projectm_set_preset_switch_requested_event_callback(instance, callback, user_data as *mut ::std::os::raw::c_void) };
    }

    pub fn set_preset_switch_failed_event_callback(instance: projectm_handle, callback: ffi::projectm_preset_switch_failed_event, user_data: *mut ()) {
        // TODO: Needs improving
        unsafe { ffi::projectm_set_preset_switch_failed_event_callback(instance, callback, user_data as *mut ::std::os::raw::c_void) };
    }

    pub fn is_preset_locked(instance: projectm_handle) -> bool {
        return unsafe { ffi::projectm_is_preset_locked(instance) };
    }

    pub fn lock_preset(instance: projectm_handle, lock: bool) {
        unsafe { ffi::projectm_lock_preset(instance, lock) };
    }

    // -----------------

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
        
    pub fn get_mesh_size(instance: projectm_handle) -> [usize; 2] {
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
    
        return [mesh.mesh_x, mesh.mesh_y];
    }

    pub fn set_mesh_size(instance: projectm_handle, mesh_x: usize, mesh_y: usize) {    
        unsafe {
            ffi::projectm_set_mesh_size(
                instance,
                mesh_x,
                mesh_y,
            );
        }
    }
    
    pub fn get_fps(instance: projectm_handle) -> usize {
        return unsafe { ffi::projectm_get_fps(instance) } as usize;
    }

    pub fn set_fps(instance: projectm_handle, fps: u32) {
        unsafe { ffi::projectm_set_fps(instance, fps as i32) };
    }

    pub fn get_texture_path(instance: projectm_handle) -> String {
        let texture_path_buf = unsafe { ffi::projectm_get_texture_path(instance) };
        let texture_path_str = unsafe { CStr::from_ptr(texture_path_buf) };
        let texture_path_str_slice = texture_path_str.to_str().unwrap();
        let texture_path = texture_path_str_slice.to_owned();

        unsafe { ffi::projectm_free_string(texture_path_buf) };

        return texture_path;
    }

    pub fn get_data_path(instance: projectm_handle) -> String {
        let data_path_buf = unsafe { ffi::projectm_get_data_path(instance) };
        let data_path_str = unsafe { CStr::from_ptr(data_path_buf) };
        let data_path_str_slice = data_path_str.to_str().unwrap();
        let data_path = data_path_str_slice.to_owned();

        unsafe { ffi::projectm_free_string(data_path_buf) };

        return data_path;
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

    pub fn get_window_size(instance: projectm_handle) -> [usize; 2] {
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
    
        return [window.width, window.height];
    }

    pub fn set_window_size(instance: projectm_handle, width: usize, height: usize) {
        unsafe { ffi::projectm_set_window_size(instance, width, height) };
    }

    // -----------------

    pub fn reset_textures(instance: projectm_handle) {
        unsafe { ffi::projectm_reset_textures(instance) };
    }

    pub fn render_frame(instance: projectm_handle) {
        unsafe { ffi::projectm_render_frame(instance) };
    }

    pub fn init_render_to_texture(instance: projectm_handle) -> u32 {
        return unsafe { ffi::projectm_init_render_to_texture(instance) };
    }
    
    // -----------------

    pub fn touch(instance: projectm_handle, x: f32, y: f32, pressure: i32, touch_type: projectm_touch_type) {
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

    pub fn write_debug_image_on_next_frame(instance: projectm_handle) {
        unsafe { ffi::projectm_write_debug_image_on_next_frame(instance) };
    }
}
