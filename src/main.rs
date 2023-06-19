// use std::env::ArgsOs;
use std::fs;
use std::io;

fn main() {
    std::process::exit(decompress());
}

fn decompress() -> i32 {
    let arg: Vec<_> = std::env::args().collect();

    if arg.len() < 2 {
        print!("Usage: {} <file> ", arg[0]);
        return 1;
    }

    let file_name = std::path::Path::new(&*arg[1]);
    let file = fs::File::open(&file_name).expect("Could not open file");
    let mut archieve = zip::ZipArchive::new(file).unwrap();

    for i in 0..archieve.len() {
        let mut file = archieve.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}
