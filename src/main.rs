use cen::app::app::AppConfig;
use kiyo::app::App;
use kiyo::app::draw_orch::{ClearConfig, DispatchConfig, DrawConfig, ImageConfig, Pass};
use std::process::Command;

fn main() {

    let app = App::new(AppConfig {
        width: 1920 / 2,
        height: 1080 / 2,
        vsync: true,
        log_fps: false,
    });

    let config = DrawConfig {
        images: Vec::from([
            ImageConfig {
                clear: ClearConfig::Color(1.0,1.0,1.0)
            },
            ImageConfig {
                clear: ClearConfig::None
            },
            ImageConfig {
                clear: ClearConfig::None
            },
        ]),
        passes: Vec::from([
            Pass {
                shader: "src/shaders/sdf_attract.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([]),
                output_resources: Vec::from([ 0 ]),
            },
            Pass {
                shader: "src/shaders/particle_swim.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([]),
                output_resources: Vec::from([ 0 ]),
            },
            Pass {
                shader: "src/shaders/clearline.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([]),
                output_resources: Vec::from([ 0 ]),
            },
            Pass {
                shader: "src/shaders/post2.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([ 0 ]),
                output_resources: Vec::from([ 1 ]),
            },
            Pass {
                shader: "src/shaders/post1.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([ 1 ]),
                output_resources: Vec::from([ 2 ]),
            },
        ])
    };

    let mut p = Command::new("src/music/symphonia-play.exe")
        .arg("src/music/inercia-jy-24.mp3")
        .spawn()
        .expect("symphonia failed");

    app.run(config, Option::None);

    _ = p.kill();
}
