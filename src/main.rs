use kiyo::app::app::AppConfig;
use kiyo::app::App;
use kiyo::app::draw_orch::{ClearConfig, DispatchConfig, DrawConfig, ImageConfig, Pass};
use kiyo::app::audio_orch::{AudioConfig};

fn main() {

    const DEV:bool = true;

    let dev_cfg = AppConfig {
        width: 1920 / 2,
        height: 1080 / 2,
        vsync: true,
        log_fps: false,
        fullscreen: false,
    };

    let prod_cfg = AppConfig {
        width: 1920,
        height: 1080,
        vsync: true,
        log_fps: false,
        fullscreen: true,
    };

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

    App::run(if DEV { dev_cfg } else { prod_cfg }, config, AudioConfig::AudioFile("src/music/inercia-jy-24.mp3".to_string()));
}
