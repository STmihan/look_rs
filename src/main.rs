use std::env;
use std::path::Path;
use std::process::exit;

use glob::glob;

struct SearchRes {
    size: String,
    full_path: String,
}

#[cfg(not(target_os = "windows"))]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().display().to_string()
}

#[cfg(target_os = "windows")]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let p = p.as_ref().display().to_string();
    if p.starts_with(VERBATIM_PREFIX) {
        p[VERBATIM_PREFIX.len()..].to_string()
    } else {
        p
    }
}

fn size_to_human_format(raw_size: u64) -> String {
    let sizes = ["B", "KB", "MB", "GB", "TB"];
    let mut size = raw_size as f64;
    let mut i = 0;

    while size >= 1024.0 && i < sizes.len() - 1 {
        size /= 1024.0;
        i += 1;
    }

    if i == 0 {
        format!("{} {}", size as u64, sizes[i])
    } else {
        format!("{:.2} {}", size, sizes[i])
    }
}

fn print_all(res: &Vec<SearchRes>) {
    let max_size_len = res.iter().map(|r| r.size.len()).max().unwrap_or(0);

    for r in res {
        println!("{:<width$} - {}", r.size, r.full_path, width = max_size_len);
    }
}

fn usage() {
    println!("USAGE: look <pattern>");
    println!("pattern '/media/**/*.jpg' will search for all jpg in media folder");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("ERROR: Incorrect arguments");
        usage();
        exit(69);
    }

    let pattern = &args[1];
    let mut res: Vec<SearchRes> = vec![];

    match glob(pattern) {
        Ok(paths) => for entry in paths {
            match entry {
                Ok(path) => {
                    let path = path.as_path();
                    let full_path = adjust_canonicalization(path.canonicalize().unwrap());
                    let size = size_to_human_format(path.metadata().unwrap().len());
                    res.push(SearchRes {
                        size,
                        full_path,
                    });
                }
                Err(e) => {
                    print_all(&res);
                    println!("ERROR: {:?}", e);
                }
            }
        },
        Err(err) => {
            println!("ERROR: Failed to read glob pattern");
            println!("{:?}", err);
            usage();
            exit(69);
        }
    }

    print_all(&res);
}
