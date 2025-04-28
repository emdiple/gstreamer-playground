use anyhow::Error;
use gst::prelude::*;
use derive_more::derive::{Display, Error};

mod gst_on_mac;

const VID_WIDTH: u32 = 640;
const VID_HEIGHT: u32 = 480;

#[derive(Debug, Display, Error)]
#[display("Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init().unwrap();
    
    let stream_pipeline = gst::Pipeline::default();
    
    let vid_src = gst::ElementFactory::make("avfvideosrc")
        .property("device_index", &0)
        .build()
        .expect("Could not create video source");
    let vid_caps = gst::Caps::builder("video/x-raw")
        .field("framerate", &gst::Fraction::new(60, 1))
        .build();

    let caps_filter = gst::ElementFactory::make("capsfilter")
        .property("caps", &vid_caps)
        .build()
        .expect("Could not create capsfilter");

    let queue_01 = gst::ElementFactory::make("queue")
        .build()
        .expect("Could not create queue");
    
    let videoconvert_01 = gst::ElementFactory::make("autovideoconvert")
        .build()
        .expect("Could not create videoconvert");
    
    let encoder = gst::ElementFactory::make("x264enc")
        .property_from_str("tune", "zerolatency")
        .property_from_str("speed-preset", "ultrafast")
        .property("key-int-max", &48u32)
        .property("threads", &4u32)
        .property("bitrate", &14448u32)
        .build()
        .expect("Could not create encoder");   

    
    let queue_02 = gst::ElementFactory::make("queue")
        .build()
        .expect("Could not create queue");

    let muxer = gst::ElementFactory::make("mpegtsmux")
        .property("alignment", &7)
        .build()
        .expect("Could not create muxer");

    let debugger = gst::ElementFactory::make("identity")
        .property("silent", &false)
        .build()
        .expect("Could not create debugger");
    
    let queue_03 = gst::ElementFactory::make("queue")
        .property_from_str("leaky", "downstream")
        .build()
        .expect("Could not create queue");

    let sink = gst::ElementFactory::make("srtsink")
        .property("uri", "srt://:8888")
        .property("sync", &false)
        .property("async", &false)
        .build()
        .expect("Could not make the srt-sink");

    stream_pipeline.add_many(&[
        vid_src.upcast_ref(),
        &caps_filter,
        &queue_01,
        &videoconvert_01,  
        &encoder,
        // &enc_cap_filter,
        &queue_02,
        &muxer,
        &debugger,
        &queue_03,
        &sink,
    ]).unwrap();
    
    gst::Element::link_many(&[
        vid_src.upcast_ref(),
        &caps_filter,
        &queue_01,
        &videoconvert_01,  
        &encoder,
        // &enc_cap_filter,
        &queue_02,
        &muxer,
        &debugger,
        &queue_03,
        &sink,
    ]).unwrap();
    
    Ok(stream_pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing).expect("Couldnt start the pipeline!");

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| glib::GString::from("UNKNOWN")),
                    error: err.error(),
                    debug: err.debug(),
                }
                .into());
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;


    Ok(())
}


fn example_main() {
    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}

fn main(){
    gst_on_mac::run(example_main);
}