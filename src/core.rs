//! ProjectM for Rust
//!
//! This library contains bindings to libprojectm. Its purpose
//! is to read an audio input and to produce mesmerizing visuals
//! by detecting tempo, and rendering advanced equations into a
//! limitless array of user-contributed visualizations.
//!
//! # Example
//!
// ! use projectm_rs::core::*;
// !
// ! let ProjectMHandle = Projectm::create();
//!

extern crate libc;
extern crate projectm_sys as ffi;

use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;

pub type ProjectMHandle = *mut ffi::projectm;

pub type ProjectMChannels = u32;
pub const MONO: ProjectMChannels = 1;
pub const STEREO: ProjectMChannels = 2;

pub type ProjectMTouchType = u32;
pub const TOUCH_TYPE_RANDOM: ProjectMTouchType = 0;
pub const TOUCH_TYPE_CIRCLE: ProjectMTouchType = 1;
pub const TOUCH_TYPE_RADIAL_BLOB: ProjectMTouchType = 2;
pub const TOUCH_TYPE_BLOB2: ProjectMTouchType = 3;
pub const TOUCH_TYPE_BLOB3: ProjectMTouchType = 4;
pub const TOUCH_TYPE_DERIVATIVE_LINE: ProjectMTouchType = 5;
pub const TOUCH_TYPE_BLOB5: ProjectMTouchType = 6;
pub const TOUCH_TYPE_LINE: ProjectMTouchType = 7;
pub const TOUCH_TYPE_DOUBLE_LINE: ProjectMTouchType = 8;

struct Projectm {}

impl Projectm {
    // -----------------
    // Core
    // -----------------

    fn create() -> *mut ffi::projectm {
        unsafe { ffi::projectm_create() }
    }

    fn destroy(instance: ProjectMHandle) {
        unsafe { ffi::projectm_destroy(instance) };
    }

    fn load_preset_file(instance: ProjectMHandle, filename: &str, smooth_transition: bool) {
        unsafe {
            ffi::projectm_load_preset_file(
                instance,
                filename.as_ptr() as *mut i8,
                smooth_transition,
            )
        };
    }

    fn load_preset_data(instance: ProjectMHandle, data: &str, smooth_transition: bool) {
        unsafe {
            ffi::projectm_load_preset_data(instance, data.as_ptr() as *mut i8, smooth_transition)
        };
    }

    fn reset_textures(instance: ProjectMHandle) {
        unsafe { ffi::projectm_reset_textures(instance) };
    }

    fn get_version_components() -> (i32, i32, i32) {
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

        (version.major, version.minor, version.patch)
    }

    fn get_version_string() -> String {
        let get_version = unsafe { ffi::projectm_get_version_string() };
        let version_str = unsafe { std::ffi::CStr::from_ptr(get_version) };
        let version_str_slice = version_str.to_str().unwrap();
        let version = version_str_slice.to_owned();

        unsafe { ffi::projectm_free_string(get_version) };

        version
    }

    fn get_vcs_version_string() -> String {
        let get_vcs_version = unsafe { ffi::projectm_get_vcs_version_string() };
        let vcs_version_str = unsafe { std::ffi::CStr::from_ptr(get_vcs_version) };
        let vcs_version_str_slice = vcs_version_str.to_str().unwrap();
        let vcs_version = vcs_version_str_slice.to_owned();

        unsafe { ffi::projectm_free_string(get_vcs_version) };

        vcs_version
    }

    // -----------------
    // Callbacks
    // -----------------

    fn set_preset_switch_requested_event_callback<F: FnMut(bool)>(
        instance: ProjectMHandle,
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

    fn set_preset_switch_failed_event_callback<F: FnMut(String, String)>(
        instance: ProjectMHandle,
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

    fn set_texture_search_paths(
        instance: ProjectMHandle,
        texture_search_paths: &[String],
        count: usize,
    ) {
        let texture_search_paths_cstr: Vec<_> = texture_search_paths
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();

        let mut texture_search_paths_pointer: Vec<_> = texture_search_paths_cstr
            .iter()
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

    fn get_beat_sensitivity(instance: ProjectMHandle) -> f32 {
        unsafe { ffi::projectm_get_beat_sensitivity(instance) }
    }

    fn set_beat_sensitivity(instance: ProjectMHandle, sensitivity: f32) {
        unsafe { ffi::projectm_set_beat_sensitivity(instance, sensitivity) };
    }

    fn get_hard_cut_duration(instance: ProjectMHandle) -> f64 {
        unsafe { ffi::projectm_get_hard_cut_duration(instance) }
    }

    fn set_hard_cut_duration(instance: ProjectMHandle, seconds: f64) {
        unsafe { ffi::projectm_set_hard_cut_duration(instance, seconds) };
    }

    fn get_hard_cut_enabled(instance: ProjectMHandle) -> bool {
        unsafe { ffi::projectm_get_hard_cut_enabled(instance) }
    }

    fn set_hard_cut_enabled(instance: ProjectMHandle, enabled: bool) {
        unsafe { ffi::projectm_set_hard_cut_enabled(instance, enabled) }
    }

    fn get_hard_cut_sensitivity(instance: ProjectMHandle) -> f32 {
        unsafe { ffi::projectm_get_hard_cut_sensitivity(instance) }
    }

    fn set_hard_cut_sensitivity(instance: ProjectMHandle, sensitivity: f32) {
        unsafe { ffi::projectm_set_hard_cut_sensitivity(instance, sensitivity) }
    }

    fn get_soft_cut_duration(instance: ProjectMHandle) -> f64 {
        unsafe { ffi::projectm_get_soft_cut_duration(instance) }
    }

    fn set_soft_cut_duration(instance: ProjectMHandle, seconds: f64) {
        unsafe { ffi::projectm_set_soft_cut_duration(instance, seconds) }
    }

    fn get_preset_duration(instance: ProjectMHandle) -> f64 {
        unsafe { ffi::projectm_get_preset_duration(instance) }
    }

    fn set_preset_duration(instance: ProjectMHandle, seconds: f64) {
        unsafe { ffi::projectm_set_preset_duration(instance, seconds) }
    }

    fn get_mesh_size(instance: ProjectMHandle) -> (usize, usize) {
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

        (mesh.mesh_x, mesh.mesh_y)
    }

    fn set_mesh_size(instance: ProjectMHandle, mesh_x: usize, mesh_y: usize) {
        unsafe {
            ffi::projectm_set_mesh_size(instance, mesh_x, mesh_y);
        }
    }

    fn get_fps(instance: ProjectMHandle) -> u32 {
        unsafe { ffi::projectm_get_fps(instance).try_into().unwrap() }
    }

    // FIXME: shouldn't it also be a usize?
    fn set_fps(instance: ProjectMHandle, fps: u32) {
        unsafe { ffi::projectm_set_fps(instance, fps as i32) };
    }

    fn get_aspect_correction(instance: ProjectMHandle) -> bool {
        unsafe { ffi::projectm_get_aspect_correction(instance) }
    }

    fn set_aspect_correction(instance: ProjectMHandle, enabled: bool) {
        unsafe { ffi::projectm_set_aspect_correction(instance, enabled) };
    }

    fn get_easter_egg(instance: ProjectMHandle) -> f32 {
        unsafe { ffi::projectm_get_easter_egg(instance) }
    }

    fn set_easter_egg(instance: ProjectMHandle, sensitivity: f32) {
        unsafe { ffi::projectm_set_easter_egg(instance, sensitivity) };
    }

    fn get_preset_locked(instance: ProjectMHandle) -> bool {
        unsafe { ffi::projectm_get_preset_locked(instance) }
    }

    fn set_preset_locked(instance: ProjectMHandle, lock: bool) {
        unsafe { ffi::projectm_set_preset_locked(instance, lock) };
    }

    fn get_window_size(instance: ProjectMHandle) -> (usize, usize) {
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

        (window.width, window.height)
    }

    fn set_window_size(instance: ProjectMHandle, width: usize, height: usize) {
        unsafe { ffi::projectm_set_window_size(instance, width, height) };
    }

    // -----------------
    // Render OpenGL
    // -----------------

    fn render_frame(instance: ProjectMHandle) {
        unsafe { ffi::projectm_opengl_render_frame(instance) };
    }

    // -----------------
    // Touch
    // -----------------

    fn touch(
        instance: ProjectMHandle,
        x: f32,
        y: f32,
        pressure: i32,
        touch_type: ProjectMTouchType,
    ) {
        unsafe { ffi::projectm_touch(instance, x, y, pressure, touch_type.try_into().unwrap()) };
    }

    fn touch_drag(instance: ProjectMHandle, x: f32, y: f32, pressure: i32) {
        unsafe { ffi::projectm_touch_drag(instance, x, y, pressure) };
    }

    fn touch_destroy(instance: ProjectMHandle, x: f32, y: f32) {
        unsafe { ffi::projectm_touch_destroy(instance, x, y) };
    }

    fn touch_destroy_all(instance: ProjectMHandle) {
        unsafe { ffi::projectm_touch_destroy_all(instance) };
    }

    // -----------------
    // Audio
    // -----------------

    fn pcm_get_max_samples() -> u32 {
        unsafe { ffi::projectm_pcm_get_max_samples() }
    }

    fn pcm_add_float(instance: ProjectMHandle, samples: &[f32], channels: ProjectMChannels) {
        assert!(
            samples.len() <= Self::pcm_get_max_samples() as usize,
            "Number of samples is greater than max samples"
        );
        let samples_per_channel = samples.len() / channels as usize;
        unsafe {
            ffi::projectm_pcm_add_float(
                instance,
                samples.as_ptr(),
                samples_per_channel as u32,
                channels.try_into().unwrap(),
            )
        }
    }

    fn pcm_add_int16(instance: ProjectMHandle, samples: &[i16], channels: ProjectMChannels) {
        assert!(
            samples.len() <= Self::pcm_get_max_samples() as usize,
            "Number of samples is greater than max samples"
        );
        let samples_per_channel = samples.len() / channels as usize;
        unsafe {
            ffi::projectm_pcm_add_int16(
                instance,
                samples.as_ptr(),
                samples_per_channel as u32,
                channels.try_into().unwrap(),
            )
        }
    }

    fn pcm_add_uint8(instance: ProjectMHandle, samples: &[u8], channels: ProjectMChannels) {
        assert!(
            samples.len() <= Self::pcm_get_max_samples() as usize,
            "Number of samples is greater than max samples"
        );
        let samples_per_channel = samples.len() / channels as usize;
        unsafe {
            ffi::projectm_pcm_add_uint8(
                instance,
                samples.as_ptr(),
                samples_per_channel as u32,
                channels.try_into().unwrap(),
            )
        }
    }

    // -----------------
    // Debug
    // -----------------

    fn write_debug_image_on_next_frame(instance: ProjectMHandle, output_file: Option<&String>) {
        // Transform the Rust String into a C String - this is needed due to the
        // fact that Rust Strings are not null terminated.
        let path = output_file.map(|p| {
            CString::new(p.as_str())
                .expect("Provided output file path could not be converted to a C string")
        });

        // `path` will be alive until the end of the scope, so we can safely get
        // a pointer to it.
        let ptr = path
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(std::ptr::null());

        unsafe { ffi::projectm_write_debug_image_on_next_frame(instance, ptr) };
    }
}

pub struct ProjectM {
    instance: Rc<RefCell<ProjectMHandle>>,
}

impl ProjectM {
    pub fn create() -> Self {
        let instance = Rc::new(RefCell::new(Projectm::create()));

        ProjectM { instance }
    }

    pub fn destroy(&self) {
        if let Ok(instance) = self.instance.try_borrow() {
            let _ = &Projectm::destroy(*instance);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn load_preset_file(&self, filename: &str, smooth_transition: bool) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::load_preset_file(*instance, filename, smooth_transition);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn load_preset_data(&self, data: &str, smooth_transition: bool) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::load_preset_data(*instance, data, smooth_transition);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn reset_textures(&self) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::reset_textures(*instance);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_version_components() -> (i32, i32, i32) {
        Projectm::get_version_components()
    }

    pub fn get_version_string() -> String {
        Projectm::get_version_string()
    }

    pub fn get_vcs_version_string() -> String {
        Projectm::get_vcs_version_string()
    }

    pub fn set_preset_switch_requested_event_callback<F: FnMut(bool) + 'static>(
        &self,
        callback: F,
    ) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_preset_switch_requested_event_callback(*instance, callback);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_preset_switch_failed_event_callback<F: FnMut(String, String) + 'static>(
        &self,
        callback: F,
    ) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_preset_switch_failed_event_callback(*instance, callback);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_texture_search_paths(&self, texture_search_paths: &[String], count: usize) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_texture_search_paths(*instance, texture_search_paths, count);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_beat_sensitivity(&self) -> f32 {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_beat_sensitivity(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_beat_sensitivity(&self, sensitivity: f32) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_beat_sensitivity(*instance, sensitivity);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_hard_cut_duration(&self) -> f64 {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_hard_cut_duration(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_hard_cut_duration(&self, seconds: f64) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_hard_cut_duration(*instance, seconds);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_hard_cut_enabled(&self) -> bool {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_hard_cut_enabled(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_hard_cut_enabled(&self, enabled: bool) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_hard_cut_enabled(*instance, enabled);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_hard_cut_sensitivity(&self) -> f32 {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_hard_cut_sensitivity(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_hard_cut_sensitivity(&self, sensitivity: f32) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_hard_cut_sensitivity(*instance, sensitivity);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_soft_cut_duration(&self) -> f64 {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_soft_cut_duration(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_soft_cut_duration(&self, seconds: f64) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_soft_cut_duration(*instance, seconds);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_preset_duration(&self) -> f64 {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_preset_duration(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_preset_duration(&self, seconds: f64) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_preset_duration(*instance, seconds);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_mesh_size(&self) -> (usize, usize) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_mesh_size(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_mesh_size(&self, mesh_x: usize, mesh_y: usize) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_mesh_size(*instance, mesh_x, mesh_y);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_fps(&self) -> u32 {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_fps(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_fps(&self, fps: u32) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_fps(*instance, fps);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_aspect_correction(&self) -> bool {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_aspect_correction(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_aspect_correction(&self, enabled: bool) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_aspect_correction(*instance, enabled);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_easter_egg(&self) -> f32 {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_easter_egg(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_easter_egg(&self, sensitivity: f32) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_easter_egg(*instance, sensitivity);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_preset_locked(&self) -> bool {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_preset_locked(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_preset_locked(&self, lock: bool) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_preset_locked(*instance, lock);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_window_size(&self) -> (usize, usize) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::get_window_size(*instance)
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn set_window_size(&self, width: usize, height: usize) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::set_window_size(*instance, width, height);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn render_frame(&self) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::render_frame(*instance);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn touch(&self, x: f32, y: f32, pressure: i32, touch_type: ProjectMTouchType) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::touch(*instance, x, y, pressure, touch_type);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn touch_drag(&self, x: f32, y: f32, pressure: i32) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::touch_drag(*instance, x, y, pressure);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn touch_destroy(&self, x: f32, y: f32) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::touch_destroy(*instance, x, y);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn touch_destroy_all(&self) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::touch_destroy_all(*instance);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn pcm_get_max_samples() -> u32 {
        Projectm::pcm_get_max_samples()
    }

    pub fn pcm_add_float(&self, samples: &[f32], channels: ProjectMChannels) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::pcm_add_float(*instance, samples, channels);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn pcm_add_int16(&self, samples: &[i16], channels: ProjectMChannels) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::pcm_add_int16(*instance, samples, channels);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn pcm_add_uint8(&self, samples: &[u8], channels: ProjectMChannels) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::pcm_add_uint8(*instance, samples, channels);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn write_debug_image_on_next_frame(&self, output_file: Option<&String>) {
        if let Ok(instance) = self.instance.try_borrow() {
            Projectm::write_debug_image_on_next_frame(*instance, output_file);
        } else {
            panic!("Failed to borrow instance");
        }
    }

    pub fn get_instance(&self) -> Rc<RefCell<ProjectMHandle>> {
        self.instance.clone()
    }
}

unsafe impl Send for ProjectM {}
unsafe impl Sync for ProjectM {}

// allow cloning the handle
impl Clone for ProjectM {
    fn clone(&self) -> Self {
        ProjectM {
            instance: self.instance.clone(),
        }
    }
}
