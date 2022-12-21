use std::fs::{read_to_string};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use projectm_rs::core::*;

fn main() -> Result<(), String> {
    // setup sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // create window
    let window = video_subsystem.window("projectm-rs-test-sdl", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    
    // create canvas/renderer
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    // projectm::settings
    let settings = settings {
        mesh_x: 96,
        mesh_y: 54,
        fps: 30,
        texture_size: 512,
        window_width: 1280,
        window_height: 720,
        preset_duration: 3.0,
        soft_cut_duration: 15.0,
        hard_cut_duration: 60.0,
        hard_cut_enabled: false,
        hard_cut_sensitivity: 0.0,
        beat_sensitivity: 0.5,
        aspect_correction: true,
        easter_egg: 0.5,
        texture_path: String::from("./textures"),
        data_path: String::from("./"),
    };

    // projectm::init
    let projectm_handle = projectm::create(&settings);
   
    projectm::set_window_size(projectm_handle, 800, 600);
    println!("ProjectM -> Initialized");

    
    fn on_preset_switch_requested(is_hard_cut: bool) {
        println!("{:?}", is_hard_cut);
    }

    fn on_preset_switch_failed(preset_filename: String, message: String) {
        println!("{:?}", preset_filename);
        println!("{:?}", message);
    }

    projectm::set_preset_switch_requested_event_callback(projectm_handle, on_preset_switch_requested); 
    projectm::set_preset_switch_failed_event_callback(projectm_handle, on_preset_switch_failed);

    // events
    let mut event_pump = sdl_context.event_pump()?;

    // renderLoop
    'running: loop {
        // check for event
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    test_destroy(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    test_get_settings(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    test_get_and_set_beat_sensitivity(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    test_get_and_set_hard_cut_duration(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                    test_get_and_set_hard_cut_enabled(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::Y), .. } => {
                    test_get_and_set_hard_cut_sensitivity(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::U), .. } => {
                    test_get_and_set_soft_cut_duration(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    test_get_and_set_preset_duration(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    test_get_and_set_mesh_size(projectm_handle); //not working
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    test_get_and_set_fps(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    test_get_paths(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    test_get_and_set_aspect_correction(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    test_get_and_set_easter_egg(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    test_get_and_set_window_size(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                    test_write_debug_image_on_next_frame(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::H), .. } => {
                    test_init_render_to_texture(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    test_load_preset_file(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    test_load_preset_data(projectm_handle); //not working
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    test_is_preset_locked(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    test_lock_preset(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    test_touch(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    test_touch_drag(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    test_touch_destroy(projectm_handle); //working
                },
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    test_touch_destroy_all(projectm_handle); //working
                },
                _ => {}
            }
        }

        // generate random audio
        generate_random_audio_data(projectm_handle);

        // projectm::render
        projectm::render_frame(projectm_handle);    
        
        // present/render
        canvas.present();
    }

    // finish okay
    Ok(())
}

fn generate_random_audio_data(projectm_handle: projectm_handle)
{
    let mut pcm_data: [[i16; 512]; 2] = [[0; 512]; 2];
    let mut i: i32 = 0;
    while i < 512 {
        if i % 2 == 1 {
            pcm_data[0 as usize][i as usize] =
                -(pcm_data[0 as usize][i as usize] as
                      i32) as i16;
            pcm_data[1 as usize][i as usize] =
                -(pcm_data[1 as usize][i as usize] as
                      i32) as i16
        }
        i += 1
    };

    projectm::pcm_add_int16(projectm_handle, &pcm_data[0][0], 512, 2)    
}

// -- Tests --
fn test_destroy(projectm_handle: projectm_handle) {
    println!("Test: destroy");
    projectm::destroy(projectm_handle);
}

fn test_get_settings(projectm_handle: projectm_handle) {
    println!("Test -> get_settings");
    let settings = projectm::get_settings(projectm_handle);
    println!("{:?}", settings);
}

fn test_get_and_set_beat_sensitivity(projectm_handle: projectm_handle) {
    println!("Test -> get_beat_sensitivity");
    println!("--beat-sensitivity: {}", projectm::get_beat_sensitivity(projectm_handle));

    println!("Test -> set_beat_sensitivity");
    projectm::set_beat_sensitivity(projectm_handle, 0.9);
    println!("--beat-sensitivity: {}", projectm::get_beat_sensitivity(projectm_handle));
}

fn test_get_and_set_hard_cut_duration(projectm_handle: projectm_handle) {
    println!("Test -> get_hard_cut_duration");
    println!("--hard_cut_duration: {}", projectm::get_hard_cut_duration(projectm_handle));

    println!("Test -> set_hard_cut_duration");
    projectm::set_hard_cut_duration(projectm_handle, 30.0);
    println!("--hard_cut_duration: {}", projectm::get_hard_cut_duration(projectm_handle));
}

fn test_get_and_set_hard_cut_enabled(projectm_handle: projectm_handle) {
    println!("Test -> get_hard_cut_enabled");
    println!("--hard_cut_enabled: {}", projectm::get_hard_cut_enabled(projectm_handle));

    println!("Test -> set_hard_cut_enabled");
    projectm::set_hard_cut_enabled(projectm_handle, true);
    println!("--hard_cut_enabled: {}", projectm::get_hard_cut_enabled(projectm_handle));
}

fn test_get_and_set_hard_cut_sensitivity(projectm_handle: projectm_handle) {
    println!("Test -> get_hard_cut_sensitivity");
    println!("--hard_cut_sensitivity: {}", projectm::get_hard_cut_sensitivity(projectm_handle));

    println!("Test -> set_hard_cut_sensitivity");
    projectm::set_hard_cut_sensitivity(projectm_handle, 0.2);
    println!("--hard_cut_sensitivity: {}", projectm::get_hard_cut_sensitivity(projectm_handle));
}

fn test_get_and_set_soft_cut_duration(projectm_handle: projectm_handle) {
    println!("Test -> get_soft_cut_duration");
    println!("--soft_cut_duration: {}", projectm::get_soft_cut_duration(projectm_handle));

    println!("Test -> set_soft_cut_duration");
    projectm::set_soft_cut_duration(projectm_handle, 25.0);
    println!("--soft_cut_duration: {}", projectm::get_soft_cut_duration(projectm_handle));
}

fn test_get_and_set_preset_duration(projectm_handle: projectm_handle) {
    println!("Test -> get_preset_duration");
    println!("--preset_duration: {}", projectm::get_preset_duration(projectm_handle));

    println!("Test -> set_preset_duration");
    projectm::set_preset_duration(projectm_handle, 2.0);
    println!("--preset_duration: {}", projectm::get_preset_duration(projectm_handle));
}

fn test_get_and_set_mesh_size(projectm_handle: projectm_handle) {
    println!("Test -> get_mesh_size");
    println!("--mesh_size: {:?}", projectm::get_mesh_size(projectm_handle));

    println!("Test -> set_mesh_size");
    projectm::set_mesh_size(projectm_handle, 128, 80);
    println!("--mesh_size: {:?}", projectm::get_mesh_size(projectm_handle));
}

fn test_get_and_set_fps(projectm_handle: projectm_handle) {
    println!("Test -> get_fps");
    println!("--fps: {}", projectm::get_fps(projectm_handle));

    println!("Test -> set_fps");
    projectm::set_fps(projectm_handle, 33);
    println!("--fps: {}", projectm::get_fps(projectm_handle));
}

fn test_get_paths(projectm_handle: projectm_handle) {
    println!("Test -> get_texture_path");
    let texture_path = projectm::get_texture_path(projectm_handle);
    println!("--texture_path: {}", texture_path);

    println!("Test -> get_data_path");
    println!("--data_path: {}", projectm::get_data_path(projectm_handle));
}

fn test_get_and_set_aspect_correction(projectm_handle: projectm_handle) {
    println!("Test -> get_aspect_correction");
    println!("--aspect_correction: {:?}", projectm::get_aspect_correction(projectm_handle));

    println!("Test -> set_aspect_correction");
    projectm::set_aspect_correction(projectm_handle, false);
    println!("--aspect_correction: {:?}", projectm::get_aspect_correction(projectm_handle));
}

fn test_get_and_set_easter_egg(projectm_handle: projectm_handle) {
    println!("Test -> get_easter_egg");
    println!("--easter_egg: {:?}", projectm::get_easter_egg(projectm_handle));

    println!("Test -> set_easter_egg");
    projectm::set_easter_egg(projectm_handle, 0.25);
    println!("--easter_egg: {:?}", projectm::get_easter_egg(projectm_handle));
}

fn test_get_and_set_window_size(projectm_handle: projectm_handle) {
    println!("Test -> get_window_size");
    println!("--window_size: {:?}", projectm::get_window_size(projectm_handle));

    println!("Test -> set_window_size");
    projectm::set_window_size(projectm_handle, 640, 360);
    println!("--window_size: {:?}", projectm::get_window_size(projectm_handle));
}

fn test_write_debug_image_on_next_frame(projectm_handle: projectm_handle) {
    println!("Test -> write_debug_image_on_next_frame");
    projectm::write_debug_image_on_next_frame(projectm_handle);
}

fn test_init_render_to_texture(projectm_handle: projectm_handle) {
    println!("Test -> init_render_to_texture");
    println!("--texture_id: {:?}", projectm::init_render_to_texture(projectm_handle));
}

fn test_load_preset_file(projectm_handle: projectm_handle) {
    println!("Test -> load_preset_file");
    let filename = String::from("presets/103-multiple-eqn.milk");
    projectm::load_preset_file(projectm_handle, &filename, false);
}

fn test_load_preset_data(projectm_handle: projectm_handle) {
    println!("Test -> load_preset_data");
    let data = read_to_string("presets/207-wave.milk").unwrap();
    projectm::load_preset_data(projectm_handle, &data, false);
}

fn test_is_preset_locked(projectm_handle: projectm_handle) {
    println!("Test -> is_preset_locked");
    println!("--locked: {:?}", projectm::is_preset_locked(projectm_handle));
}

fn test_lock_preset(projectm_handle: projectm_handle) {
    println!("Test -> lock_preset");
    projectm::lock_preset(projectm_handle, true);

    println!("Test -> is_preset_locked");
    println!("--locked: {:?}", projectm::is_preset_locked(projectm_handle));
}

fn test_touch(projectm_handle: projectm_handle) {
    println!("Test -> touch");
    projectm::touch(projectm_handle, 32.0, 32.0, 1, TOUCH_TYPE_CIRCLE);
}

fn test_touch_drag(projectm_handle: projectm_handle) {
    println!("Test -> touch_drag");
    projectm::touch_drag(projectm_handle, 32.0, 32.0, 1);
}

fn test_touch_destroy(projectm_handle: projectm_handle) {
    println!("Test -> touch_destroy");
    projectm::touch_destroy(projectm_handle, 32.0, 32.0);
}

fn test_touch_destroy_all(projectm_handle: projectm_handle) {
    println!("Test -> touch_destroy_all");
    projectm::touch_destroy_all(projectm_handle);
}