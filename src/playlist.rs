// idk ?
#![allow(non_camel_case_types)]

extern crate libc;
extern crate projectm_sys as ffi;

use std::ffi::CString;
use ffi::projectm_handle;

// pub enum projectm_playlist {}

pub struct Playlist {
    // projectm:  *const projectm_handle,
    playlist: *mut ffi::projectm_playlist,
}

impl Playlist {
    /// Create a new playlist for [projectm](projectm_handle)
    pub fn create(projectm: projectm_handle) -> Playlist {
        unsafe {
            let playlist = ffi::projectm_playlist_create(projectm);
            Playlist { playlist  }
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            let len = ffi::projectm_playlist_size(self.playlist);
            len as usize
        }
    }

    /// Scan and add a directory of presets to the playlist.
    pub fn add_path(&mut self, path: &str, recursive: bool ) {
        unsafe {
            let c_path = CString::new(path).unwrap();
            ffi::projectm_playlist_add_path(self.playlist, c_path.as_ptr(), recursive , false);
        }
    }


}
