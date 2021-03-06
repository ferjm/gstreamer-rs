extern crate gstreamer as gst;
use std::io;
use std::io::Write;
use gst::prelude::*;
use gst::MessageView;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

struct CustomData {
    playbin: gst::Element,    // Our one and only element
    playing: bool,            // Are we in the PLAYING state?
    terminate: bool,          // Should we terminate execution?
    seek_enabled: bool,       // Is seeking enabled for this media?
    seek_done: bool,          // Have we performed the seek already?
    duration: gst::ClockTime, // How long does this media last, in nanoseconds
}

fn tutorial_main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Creat the playbin element
    let playbin =
        gst::ElementFactory::make("playbin", "playbin").expect("Failed to create playbin element");

    // Set the URI to play
    let uri =
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    playbin
        .set_property("uri", &uri)
        .expect("Can't set uri property on playbin");

    // Start playing
    let ret = playbin.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    // Listen to the bus
    let bus = playbin.get_bus().unwrap();
    let mut custom_data = CustomData {
        playbin: playbin,
        playing: false,
        terminate: false,
        seek_enabled: false,
        seek_done: false,
        duration: gst::CLOCK_TIME_NONE,
    };

    while !custom_data.terminate {
        let msg = bus.timed_pop(100 * gst::MSECOND);

        match msg {
            Some(msg) => {
                handle_message(&mut custom_data, &msg);
            }
            None => {
                if custom_data.playing {
                    let position = custom_data
                        .playbin
                        .query_position::<gst::ClockTime>()
                        .expect("Could not query current position.");

                    // If we didn't know it yet, query the stream duration
                    if custom_data.duration == gst::CLOCK_TIME_NONE {
                        custom_data.duration = custom_data
                            .playbin
                            .query_duration()
                            .expect("Could not query current duration.")
                    }

                    // Print current position and total duration
                    print!("\rPosition {} / {}", position, custom_data.duration);
                    io::stdout().flush().unwrap();

                    if custom_data.seek_enabled && !custom_data.seek_done
                        && position > 10 * gst::SECOND
                    {
                        println!("\nReached 10s, performing seek...");
                        custom_data
                            .playbin
                            .seek_simple(
                                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                                30 * gst::SECOND,
                            )
                            .expect("Failed to seek.");
                        custom_data.seek_done = true;
                    }
                }
            }
        }
    }

    // Shutdown pipeline
    let ret = custom_data.playbin.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}

fn handle_message(custom_data: &mut CustomData, msg: &gst::GstRc<gst::MessageRef>) {
    match msg.view() {
        MessageView::Error(err) => {
            println!(
                "Error received from element {:?}: {} ({:?})",
                err.get_src().map(|s| s.get_path_string()),
                err.get_error(),
                err.get_debug()
            );
            custom_data.terminate = true;
        }
        MessageView::Eos(..) => {
            println!("End-Of-Stream reached.");
            custom_data.terminate = true;
        }
        MessageView::DurationChanged(_) => {
            // The duration has changed, mark the current one as invalid
            custom_data.duration = gst::CLOCK_TIME_NONE;
        }
        MessageView::StateChanged(state_changed) => if state_changed
            .get_src()
            .map(|s| s == custom_data.playbin)
            .unwrap_or(false)
        {
            let new_state = state_changed.get_current();
            let old_state = state_changed.get_old();

            println!(
                "Pipeline state changed from {:?} to {:?}",
                old_state, new_state
            );

            custom_data.playing = new_state == gst::State::Playing;
            if custom_data.playing {
                let mut seeking = gst::Query::new_seeking(gst::Format::Time);
                if custom_data.playbin.query(&mut seeking) {
                    let (seekable, start, end) = seeking.get_result();
                    custom_data.seek_enabled = seekable;
                    if seekable {
                        println!("Seeking is ENABLED from {:?} to {:?}", start, end)
                    } else {
                        println!("Seeking is DISABLED for this stream.")
                    }
                } else {
                    eprintln!("Seeking query failed.")
                }
            }
        },
        _ => (),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    tutorials_common::run(tutorial_main);
}
