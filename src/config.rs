use clap;
use ini;

pub struct Configuration {
    igmp: Igmp
}

pub struct Igmp {
    query_interval: u8,
    querier_timeout: u8,
}

pub fn init() -> Configuration {
    let mut configfile: Option<String> = None;

    let matches = clap::App::new("MultiCast Router")
        .author("Curtis W. Ruck <curtis@ruck.io>")
        .about("Sets up multicast routing for pure IGMP (Layer 2) management.")
        

    let config_ini = ini::Ini::load_from_file(configfile.expect("No configuration file specified")).unwrap()    ;

    let igmp_ini = config_ini.section(Some("igmp".to_owned())).unwrap();
    let config = Configuration {
        igmp: Igmp {
            query_interval: igmp_ini.get("query_interval").unwrap().parse::<u8>().unwrap(),
            querier_timeout: igmp_ini.get("querier_timeout").unwrap().parse::<u8>().unwrap()
        }
    };

    config
}