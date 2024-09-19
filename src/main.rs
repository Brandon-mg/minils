use std::fs::{Metadata, Permissions};
use std::{env, fmt, fs, io};
use std::path::PathBuf;
use std::process;
//std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone)]
enum ShowType{
    All,
    File,
    Dir,
}

impl Default for ShowType{
    fn default() -> Self {ShowType::All}
}

#[derive(Debug, Clone)]
enum SortType {
    Name,
    LastModified,
    Created,
    Size,
}

impl Default for SortType{
    fn default() -> Self {SortType::Name}
}

#[derive(Debug, Default, Clone)]
struct Params {
    perms: bool,
    show: ShowType,
    long: bool,
    sort: SortType,
    path: PathBuf,
}

impl Params {
    fn _new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
struct FileData {
    permissions: String,
    file_name: String,
    metadata: Metadata,
    params: Params
}

impl fmt::Display for FileData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out_str = String::new();
        out_str.push_str(&self.file_name);
        if let Some(i) = self.params.perms.then(|| self.permissions.clone()) {
            out_str.push_str(&format!("\t{}", i));
        };
        if let Some(i) = self.params.long.then(|| self.metadata.len().to_string()) {
            out_str.push_str(&format!("\t{}", i));
        };
        match self.params.sort {
            SortType::LastModified => out_str.push_str(&format!("\t{:?}", self.metadata.modified().unwrap())),
            SortType::Created => out_str.push_str(&format!("\t{:?}", self.metadata.created().unwrap())),
            _ => (),
        };
        write!(f, "{}", out_str)
    }
}
fn parse_permissions(perms: Permissions) -> Option<String> {
    perms.readonly().then_some("Readonly".to_string())
}

fn ls_dir(dir: &PathBuf, params: Params) -> Result<(), Box<dyn std::error::Error>>{
    let mut items: Vec<FileData> = Vec::new(); 
    for item in fs::read_dir(dir)? {
        
        let item = item?;
        let file_name = item
                                .file_name()
                                .into_string()
                                .or_else(|f| Err(format!("Errored on: {:?}", f)))?;
        let metadata = item.metadata()?;
        let permissions = parse_permissions(metadata.permissions()).unwrap_or("Writeable".to_string());
        let new_item = FileData {
            permissions,
            file_name,
            metadata,
            params: params.clone(),
        };
        items.push(new_item);
    }
    match params.sort {
        SortType::LastModified => items.sort_by_key(|d| d.metadata.modified().unwrap()),
        SortType::Created => items.sort_by_key(|d| d.metadata.created().unwrap()),
        SortType::Size => items.sort_by_key(|d| d.metadata.len()),
        SortType::Name => (),
    };
    for item in items.iter() {
        println!("{}", item);
    }
    Ok(())
}

fn ls_file(item: &PathBuf, params: Params) -> Result<(), Box<dyn std::error::Error>>{
    
    let file_name = item
                            .file_name()
                            .ok_or(std::io::Error::new(io::ErrorKind::NotFound, "File not Found"))?
                            .to_str()
                            .ok_or(std::io::Error::new(io::ErrorKind::Unsupported, "Borken UTF-8"))?
                            .to_string();
    let metadata = item.metadata()?;
    let permissions = parse_permissions(metadata.permissions()).unwrap_or("Writeable".to_string());
    let new_item = FileData {
        permissions,
        file_name,
        metadata,
        params: params.clone(),
    };
    println!("{}", new_item);
    Ok(())
}

fn ls(path: &PathBuf, params: Params) -> Result<(), Box<dyn std::error::Error>>{
    if path.is_dir(){
        return ls_dir(path, params);
    }
    ls_file(path, params)
}
fn main() {
    let mut params = Params::default();
    println!("{:?}", params);
    let args = env::args().skip(1).collect::<Vec<String>>();
    if let Some(path) = args.last() {
        match path.chars().next().unwrap() {
            '-' => params.path = PathBuf::from("."),
            _ => params.path = PathBuf::from(path),
        }
    } else {
        params.path = PathBuf::from(".");
    };
    for argument in args {
        match argument.as_str() {
            "-p" => params.perms = true,
            "-l" => params.long = true,
            "-c" => params.sort = SortType::Created,
            "-m" => params.sort = SortType::LastModified,
            "-s" => params.sort = SortType::Size,
            "-d" => params.show = ShowType::Dir,
            "-f" => params.show = ShowType::File,
            _ => continue,

        }
    };


    if ls(&params.path.clone(), params).is_err() {
        process::exit(1);
    }
}
