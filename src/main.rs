use cen::app::app::AppConfig;
use kiyo::app::App;
use kiyo::app::draw_orch::{ClearConfig, DispatchConfig, DrawConfig, ImageConfig, Pass};

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
                clear: ClearConfig::Color(0.0,0.0,0.0)
            },
            ImageConfig {
                clear: ClearConfig::Color(0.0,0.0,0.0)
            },
        ]),
        passes: Vec::from([
            Pass {
                shader: "src/shaders/main.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([]),
                output_resources: Vec::from([ 0 ]),
            },
            Pass {
                shader: "src/shaders/post1.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([ 0 ]),
                output_resources: Vec::from([ 1 ]),
            },
        ])
    };

    app.run(config, None);
}
