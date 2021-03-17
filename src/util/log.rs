use log::{LevelFilter, SetLoggerError};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;

pub fn configure_logger(path: &str) -> Result<(), SetLoggerError> {
  let level = log::LevelFilter::Info;
  let file_path = path;

  // Build a stderr logger.
  let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

  // Logging to log file.
  let logfile = FileAppender::builder().encoder(Box::new(PatternEncoder::new("{l} - {m}\n"))).build(file_path).unwrap();

  // Log Trace level output to file where trace is the default level
  // and the programmatically specified level to stderr.
  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .appender(Appender::builder().filter(Box::new(ThresholdFilter::new(level))).build("stderr", Box::new(stderr)))
    .build(Root::builder().appender("logfile").appender("stderr").build(LevelFilter::Trace))
    .unwrap();

  // Use this to change log levels at runtime.
  // This means you can change the default log level to trace
  // if you are trying to debug an issue and need more logs on then turn it off
  // once you are done.
  let _handle = log4rs::init_config(config)?;
  Ok(())
}
