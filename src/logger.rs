use log::LevelFilter;
use syslog::Facility;

pub fn init() {
    syslog::init(Facility::LOG_DAEMON, LevelFilter::Trace, Some("mcr"))
        .expect("Unable to initialize syslog");
}
