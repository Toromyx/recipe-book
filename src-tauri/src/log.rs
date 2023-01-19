use log::LevelFilter;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::path::app_data_dir;

pub fn init() {
    let pattern_encoder = PatternEncoder::default();
    let mut config_builder = Config::builder();
    let mut root_builder = Root::builder();

    // console
    if cfg!(debug_assertions) {
        config_builder = config_builder.appender(
            Appender::builder().build(
                "console",
                Box::new(
                    ConsoleAppender::builder()
                        .encoder(Box::new(pattern_encoder.clone()))
                        .build(),
                ),
            ),
        );
        root_builder = root_builder.appender("console");
    }

    // file
    let log_path = app_data_dir().join("log");
    config_builder = config_builder.appender(
        Appender::builder().build(
            "file",
            Box::new(
                RollingFileAppender::builder()
                    .encoder(Box::new(pattern_encoder))
                    .build(
                        log_path.join("recipe.log"),
                        Box::new(CompoundPolicy::new(
                            Box::new(SizeTrigger::new(50 * 2_u64.pow(10))),
                            Box::new(
                                FixedWindowRoller::builder()
                                    .build(
                                        &log_path
                                            .join("recipe.{}.log")
                                            .into_os_string()
                                            .into_string()
                                            .unwrap(),
                                        5,
                                    )
                                    .unwrap(),
                            ),
                        )),
                    )
                    .unwrap(),
            ),
        ),
    );
    root_builder = root_builder.appender("file");

    let root = root_builder.build(LevelFilter::Warn);

    log4rs::init_config(config_builder.build(root).unwrap()).unwrap();
}
