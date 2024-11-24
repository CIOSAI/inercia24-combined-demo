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

    fn audio_shader(t:f32) -> (f32, f32) {
        let tau = 2.0 * std::f32::consts::PI;

        fn pitch(p:f32) -> f32 {
            return f32::powf(1.059460646483, p+3.) * 440.0;
        }

        
        let melody = vec![0.0,3.0,5.0,6.0,7.0,10.0,12.0][((t*7.0) as usize)%7];

        let n = (tau * pitch(melody) * t).sin();
        // n *= f32::powf((0.5-f32::abs((f32::sin(t)*18.).fract()-0.5))*2.,0.5);

        (n, n)
    }
    app.run(config, Option::None);
}
