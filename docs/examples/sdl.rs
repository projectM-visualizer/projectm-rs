extern crate sdl2;

use std::fs::read_to_string;

use projectm_rs::core::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// #[cfg(example)]
fn main() -> Result<(), String> {
    // setup sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // create window
    let window = video_subsystem
        .window("projectm-rs-test-sdl", 1024, 768)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    // create canvas/renderer
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    // projectm::init
    let ProjectMHandle = projectm::create();

    // projectm::settings
    initiate_settings(ProjectMHandle);

    println!("ProjectM -> Initialized");

    // events
    let mut event_pump = sdl_context.event_pump()?;

    // renderLoop
    'running: loop {
        // check for event
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    test_destroy(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    test_get_versions(); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    test_set_texture_search_paths(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    test_get_and_set_beat_sensitivity(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => {
                    test_get_and_set_hard_cut_duration(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Y),
                    ..
                } => {
                    test_get_and_set_hard_cut_enabled(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::U),
                    ..
                } => {
                    test_get_and_set_hard_cut_sensitivity(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    test_get_and_set_soft_cut_duration(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    test_get_and_set_preset_duration(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    test_get_and_set_mesh_size(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    test_get_and_set_fps(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    test_get_and_set_aspect_correction(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    test_get_and_set_easter_egg(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    test_get_and_set_window_size(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => {
                    test_write_debug_image_on_next_frame(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    test_load_preset_file(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => {
                    test_load_preset_data(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => {
                    test_get_preset_locked(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    test_set_preset_locked(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    test_touch(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    test_touch_drag(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    test_touch_destroy(ProjectMHandle); //working
                }
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => {
                    test_touch_destroy_all(ProjectMHandle); //working
                }
                _ => {}
            }
        }

        // generate random audio
        generate_random_audio_data(ProjectMHandle);

        // projectm::render
        projectm::render_frame(ProjectMHandle);

        // present/render
        canvas.present();
    }

    // finish okay
    Ok(())
}

fn initiate_settings(projectm_handle: ProjectMHandle) {
    projectm::set_window_size(projectm_handle, 1024, 768);
    projectm::set_mesh_size(projectm_handle, 32, 24);
    projectm::set_soft_cut_duration(projectm_handle, 3.0);
    projectm::set_preset_duration(projectm_handle, 10.0);
    projectm::set_easter_egg(projectm_handle, 0.0);
    projectm::set_hard_cut_enabled(projectm_handle, false);
    projectm::set_hard_cut_duration(projectm_handle, 1.0);
    projectm::set_beat_sensitivity(projectm_handle, 1.0);
    projectm::set_aspect_correction(projectm_handle, true);
    projectm::set_fps(projectm_handle, 60);

    fn on_preset_switch_requested(is_hard_cut: bool) {
        println!("on_preset_switch_requested:is_hard_cut:{:?}", is_hard_cut);
    }

    fn on_preset_switch_failed(preset_filename: String, message: String) {
        println!(
            "on_preset_switch_failed:preset_filename:{:?}",
            preset_filename
        );
        println!("on_preset_switch_failed:message:{:?}", message);
    }

    projectm::set_preset_switch_requested_event_callback(
        projectm_handle,
        on_preset_switch_requested,
    );
    projectm::set_preset_switch_failed_event_callback(projectm_handle, on_preset_switch_failed);
}

fn generate_random_audio_data(projectm_handle: ProjectMHandle) {
    let mut pcm_data: [[i16; 512]; 2] = [[0; 512]; 2];
    let mut i: i32 = 0;
    while i < 512 {
        if i % 2 == 1 {
            pcm_data[0 as usize][i as usize] = -(pcm_data[0 as usize][i as usize] as i32) as i16;
            pcm_data[1 as usize][i as usize] = -(pcm_data[1 as usize][i as usize] as i32) as i16
        }
        i += 1
    }

    projectm::pcm_add_int16(projectm_handle, &pcm_data[0][0], 512, 2)
}

// Tests: Core
fn test_destroy(projectm_handle: ProjectMHandle) {
    println!("Test -> destroy");
    projectm::destroy(projectm_handle);
}

fn test_get_versions() {
    println!("Test -> get_version_components");
    println!(
        "--version-components: {:?}",
        projectm::get_version_components()
    );

    println!("Test -> get_version_string");
    println!("--version-string: {:?}", projectm::get_version_string());

    println!("Test -> get_vcs_version_string");
    println!(
        "--vcs_version-string: {:?}",
        projectm::get_vcs_version_string()
    );
}

// Tests: Parameters
fn test_set_texture_search_paths(ProjectMHandle: ProjectMHandle) {
    println!("Test -> set_texture_search_paths");
    let mut search_paths = Vec::new();
    search_paths.push("./examples".to_string());
    search_paths.push("./presets".to_string());
    let count = search_paths.len();

    projectm::set_texture_search_paths(ProjectMHandle, search_paths, count);
}

fn test_get_and_set_beat_sensitivity(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_beat_sensitivity");
    println!(
        "--beat-sensitivity: {}",
        projectm::get_beat_sensitivity(ProjectMHandle)
    );

    println!("Test -> set_beat_sensitivity");
    projectm::set_beat_sensitivity(ProjectMHandle, 0.9);
    println!(
        "--beat-sensitivity: {}",
        projectm::get_beat_sensitivity(ProjectMHandle)
    );
}

fn test_get_and_set_hard_cut_duration(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_hard_cut_duration");
    println!(
        "--hard_cut_duration: {}",
        projectm::get_hard_cut_duration(ProjectMHandle)
    );

    println!("Test -> set_hard_cut_duration");
    projectm::set_hard_cut_duration(ProjectMHandle, 30.0);
    println!(
        "--hard_cut_duration: {}",
        projectm::get_hard_cut_duration(ProjectMHandle)
    );
}

fn test_get_and_set_hard_cut_enabled(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_hard_cut_enabled");
    println!(
        "--hard_cut_enabled: {}",
        projectm::get_hard_cut_enabled(ProjectMHandle)
    );

    println!("Test -> set_hard_cut_enabled");
    projectm::set_hard_cut_enabled(ProjectMHandle, true);
    println!(
        "--hard_cut_enabled: {}",
        projectm::get_hard_cut_enabled(ProjectMHandle)
    );
}

fn test_get_and_set_hard_cut_sensitivity(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_hard_cut_sensitivity");
    println!(
        "--hard_cut_sensitivity: {}",
        projectm::get_hard_cut_sensitivity(ProjectMHandle)
    );

    println!("Test -> set_hard_cut_sensitivity");
    projectm::set_hard_cut_sensitivity(ProjectMHandle, 0.2);
    println!(
        "--hard_cut_sensitivity: {}",
        projectm::get_hard_cut_sensitivity(ProjectMHandle)
    );
}

fn test_get_and_set_soft_cut_duration(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_soft_cut_duration");
    println!(
        "--soft_cut_duration: {}",
        projectm::get_soft_cut_duration(ProjectMHandle)
    );

    println!("Test -> set_soft_cut_duration");
    projectm::set_soft_cut_duration(ProjectMHandle, 25.0);
    println!(
        "--soft_cut_duration: {}",
        projectm::get_soft_cut_duration(ProjectMHandle)
    );
}

fn test_get_and_set_preset_duration(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_preset_duration");
    println!(
        "--preset_duration: {}",
        projectm::get_preset_duration(ProjectMHandle)
    );

    println!("Test -> set_preset_duration");
    projectm::set_preset_duration(ProjectMHandle, 2.0);
    println!(
        "--preset_duration: {}",
        projectm::get_preset_duration(ProjectMHandle)
    );
}

fn test_get_and_set_mesh_size(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_mesh_size");
    println!("--mesh_size: {:?}", projectm::get_mesh_size(ProjectMHandle));

    println!("Test -> set_mesh_size");
    projectm::set_mesh_size(ProjectMHandle, 128, 80);
    println!("--mesh_size: {:?}", projectm::get_mesh_size(ProjectMHandle));
}

fn test_get_and_set_fps(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_fps");
    println!("--fps: {}", projectm::get_fps(ProjectMHandle));

    println!("Test -> set_fps");
    projectm::set_fps(ProjectMHandle, 33);
    println!("--fps: {}", projectm::get_fps(ProjectMHandle));
}

fn test_get_and_set_aspect_correction(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_aspect_correction");
    println!(
        "--aspect_correction: {:?}",
        projectm::get_aspect_correction(ProjectMHandle)
    );

    println!("Test -> set_aspect_correction");
    projectm::set_aspect_correction(ProjectMHandle, false);
    println!(
        "--aspect_correction: {:?}",
        projectm::get_aspect_correction(ProjectMHandle)
    );
}

fn test_get_and_set_easter_egg(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_easter_egg");
    println!(
        "--easter_egg: {:?}",
        projectm::get_easter_egg(ProjectMHandle)
    );

    println!("Test -> set_easter_egg");
    projectm::set_easter_egg(ProjectMHandle, 0.25);
    println!(
        "--easter_egg: {:?}",
        projectm::get_easter_egg(ProjectMHandle)
    );
}

fn test_get_and_set_window_size(ProjectMHandle: ProjectMHandle) {
    println!("Test -> get_window_size");
    println!(
        "--window_size: {:?}",
        projectm::get_window_size(ProjectMHandle)
    );

    println!("Test -> set_window_size");
    projectm::set_window_size(ProjectMHandle, 640, 360);
    println!(
        "--window_size: {:?}",
        projectm::get_window_size(ProjectMHandle)
    );
}

fn test_write_debug_image_on_next_frame(ProjectMHandle: ProjectMHandle) {
    println!("Test -> write_debug_image_on_next_frame_with_filename");
    let save_filename = String::from("test_debug_image.bmp");
    projectm::write_debug_image_on_next_frame(ProjectMHandle, Some(&save_filename));

    // println!("Test -> write_debug_image_on_next_frame_without_filename");
    // projectm::write_debug_image_on_next_frame(ProjectMHandle, None);
}

fn test_load_preset_file(ProjectMHandle: ProjectMHandle) {
    println!("Test -> load_preset_file");
    let filename = String::from("presets/103-multiple-eqn.milk");
    projectm::load_preset_file(ProjectMHandle, &filename, false);
}

fn test_load_preset_data(ProjectMHandle: ProjectMHandle) {
    println!("Test -> load_preset_data");
    let data = read_to_string("presets/110-per_pixel.milk").unwrap();
    projectm::load_preset_data(ProjectMHandle, &data, false);
}

fn test_get_preset_locked(ProjectMHandle: ProjectMHandle) {
    println!("Test -> is_preset_locked");
    println!(
        "--locked: {:?}",
        projectm::get_preset_locked(ProjectMHandle)
    );
}

fn test_set_preset_locked(ProjectMHandle: ProjectMHandle) {
    println!("Test -> lock_preset");
    projectm::set_preset_locked(ProjectMHandle, true);

    println!("Test -> is_preset_locked");
    println!(
        "--locked: {:?}",
        projectm::get_preset_locked(ProjectMHandle)
    );
}

fn test_touch(ProjectMHandle: ProjectMHandle) {
    println!("Test -> touch");
    projectm::touch(ProjectMHandle, 32.0, 32.0, 1, TOUCH_TYPE_CIRCLE);
}

fn test_touch_drag(ProjectMHandle: ProjectMHandle) {
    println!("Test -> touch_drag");
    projectm::touch_drag(ProjectMHandle, 32.0, 32.0, 1);
}

fn test_touch_destroy(ProjectMHandle: ProjectMHandle) {
    println!("Test -> touch_destroy");
    projectm::touch_destroy(ProjectMHandle, 32.0, 32.0);
}

fn test_touch_destroy_all(ProjectMHandle: ProjectMHandle) {
    println!("Test -> touch_destroy_all");
    projectm::touch_destroy_all(ProjectMHandle);
}
