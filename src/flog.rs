use std::{path::Path, fs::File, error::Error};
use chrono::Utc;



pub fn get_logger() -> Result<(),Box<dyn Error>>{


    let d = Utc::now();
    // let dn = d.date_naive();
    let sd = d.date_naive().to_string();

    let folder = "/home/steve/federing/logs";
    let file = format!("{sd}.log");
    
    let path_name = format!("{}/{}",folder,file);
    let path = Path::new(path_name.as_str());
    let file_exists = path.exists();

    
    let f = if file_exists {
        File::options().append(true).open(path_name)?
    } else {
        File::create(path_name)?
    };
    
   tracing_subscriber::fmt().with_writer(f).init();
   Ok(())
}