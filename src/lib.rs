pub extern crate log;
use config_loader::ConfigLoader;
pub use log::*;

pub fn setup_logger(config_loader: Option<config_loader::ConfigLoader>) -> Result<(), fern::InitError>
{
    let mut dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        });

    match config_loader
    {
        Some(config) => 
        {
            dispatch = get_level_filter(&config, dispatch);
            dispatch = dispatch.chain(std::io::stdout());
            dispatch = get_output_filename(&config, dispatch)?;
            dispatch.apply()?;
            
        }
        None =>
        {

         dispatch
            .level(log::LevelFilter::Info)
            .chain(std::io::stdout())
            .chain(fern::log_file("output.log")?)
            .apply()?;
        }
    }
    
    Ok(())
}

fn get_level_filter(config: &ConfigLoader, dispatch: fern::Dispatch) -> fern::Dispatch
{
    match config.get_string("level_filter")
    {
        Ok(value) => match value.as_str()
        {
            "Debug" => 
            {
                dispatch.level(log::LevelFilter::Debug)
            }
            "Error" => 
            {
                dispatch.level(log::LevelFilter::Error)
            }
            "Trace" => 
            {
                dispatch.level(log::LevelFilter::Trace)
            }
            "Warn" => 
            {
                dispatch.level(log::LevelFilter::Warn)
            }
            "Off" => 
            {
                dispatch.level(log::LevelFilter::Off)
            }

            _ => 
            {
                dispatch.level(log::LevelFilter::Info)
            }
        }
        Err(_) =>
        {
            dispatch.level(log::LevelFilter::Info)
        }
    }
}


fn get_output_filename(config: &ConfigLoader, dispatch: fern::Dispatch) -> Result<fern::Dispatch, fern::InitError>
{
    match config.get_string("log_output_filename")
    {
        Ok(value) => 
        {
            Ok(dispatch.chain(fern::log_file(value)?))
        }
        Err(_) =>
        {
            Ok(dispatch.chain(fern::log_file("output.log")?))
        }
    }
}
