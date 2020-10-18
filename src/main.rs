mod api;
mod baidu;
mod config;

use anyhow::{Context, Result};
use structopt::StructOpt;

use api::Api;

#[derive(StructOpt, Debug)]
enum Driver {
    Baidu,
}

impl Driver {
    fn get_driver(&self, conf: config::Config) -> Result<Box<dyn Api>> {
        match self {
            Driver::Baidu => match conf.baidu {
                Some(b) => Ok(Box::new(baidu::Translator::new(b))),
                None => Err(anyhow::format_err!("not found baidu conf")),
            },
        }
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    driver: Driver,
    #[structopt(
        short,
        default_value = "~/.config/translate.toml",
        help = "config file"
    )]
    config: String,
    #[structopt(short)]
    text: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let conf = config::Config::from_file(&opt.config).with_context(|| opt.config.clone())?;
    let translator = opt.driver.get_driver(conf)?;
    let trans = smol::block_on(translator.translate("en", "zh", opt.text.as_str()))?;
    println!("{}", trans);
    Ok(())
}
