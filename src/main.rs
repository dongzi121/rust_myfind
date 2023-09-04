use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use colored::Colorize;
use std::collections::HashSet;
use tracing;
pub mod tracing_init;

//模式1： cargo run -- -i <目录1> <目录2>.... <正则>    可以同时搜索多个path,并输出一共多少个匹配项，并去重排序
//模式2： cargo run -- -v <目录>  <正则>  ，并去重排序，并输出所有的遍历文件
//模式3： cargo run -- -z <目录> <正则1>...<正则n>   可以同时匹配多个正则,并输出一共多少个匹配项，并去重排序
//同时，命令行可以彩色输出，一些语句进行了彩色处理
//尝试通过tracing输出日志，但有奇怪的环境错误
fn main() {
    tracing_init::tracing_init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("使用方法：{} <目标目录> <要搜索的正则表达式>", args[0]);
        process::exit(1);
    }
    let pattern = &args[args.len() - 1];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效的正则表达式 '{}': {}", pattern, err);
            process::exit(1);
        }
    };

    if args[1] == "-i" {
        let mut total_matches = 0;
        let mut all_files = Vec::new();

        for r in 2..(args.len() - 1) {
            match find(&args[r], &regex) {
                Ok((matches, count, files)) => {
                    total_matches += count;
                    all_files.extend(files);
                    if matches.is_empty() {
                        println!("{}", "未找到匹配项".blue());
                    } else {
                        println!("{}", "找到以下匹配项".yellow());
                        let mut matches: Vec<_> = matches.into_iter().collect::<HashSet<_>>().into_iter().collect();//去重
                        matches.sort(); //排序
                        for file in matches {
                            println!("{}", file.green());
                        }
                    }
                }
                
                Err(error) => {
                    eprintln!("发生错误: {}", error);
                    process::exit(1);
                }
            }
            println!("----------");
        }
        println!("总共找到 {} 个匹配项", total_matches);

    }else if args[1] == "-v"{
        match find(&args[2], &regex) {
            Ok((matches, total_matches, all_files)) => {
                if matches.is_empty() {
                    println!("{}", "未找到匹配项".blue());
                } else {
                    println!("{}", "找到以下匹配项".yellow());
                    let mut matches: Vec<_> = matches.into_iter().collect::<HashSet<_>>().into_iter().collect();//去重
                    matches.sort(); //排序
                    for file in matches {
                        println!("{}", file.green());
                    }
                }
                println!("总共找到 {} 个匹配项", total_matches);

                {
                    println!("{}", "所有遍历到的文件：".cyan());
                    for file in all_files {
                        println!("{}", file);
                    }
                }
            }
            Err(error) => {
                eprintln!("发生错误: {}", error);
                process::exit(1);
            }
        }
    
    }else if args[1] == "-z"{
        let mut total_matches = 0;
        let mut all_files = Vec::new();
        for r in 3..(args.len() ) {
            let pattern2 = &args[r];
            let regex2 = match Regex::new(pattern2) {
                Ok(re) => re,
                Err(err) => {
                    eprintln!("无效的正则表达式 '{}': {}", pattern2, err);
                    process::exit(1);
                }
            };
            match find(&args[2], &regex2) {
                Ok((matches, count, files)) => {
                    total_matches += count;
                    all_files.extend(files);
                    if matches.is_empty() {
                        println!("{}", "未找到匹配项".blue());
                    } else {
                        println!("{}", "找到以下匹配项".yellow());
                        let mut matches: Vec<_> = matches.into_iter().collect::<HashSet<_>>().into_iter().collect();//去重
                        matches.sort(); //排序
                        for file in matches {
                            println!("{}", file.green());
                        }
                    }
                }
                
                Err(error) => {
                    eprintln!("发生错误: {}", error);
                    process::exit(1);
                }
            }
            println!("----------");
        }
        println!("总共找到 {} 个匹配项", total_matches);

    }else if args.len() > 3 {
        eprintln!("{}", "you can't do it".cyan());
        process::exit(1);
    } else {
        match find(&args[1], &regex) {
            Ok((matches, total_matches, all_files)) => {
                if matches.is_empty() {
                    println!("{}", "未找到匹配项".blue());
                } else {
                    println!("{}", "找到以下匹配项".yellow());
                    let mut matches: Vec<_> = matches.into_iter().collect::<HashSet<_>>().into_iter().collect();//去重
                    matches.sort(); //排序
                    for file in matches {
                        println!("{}", file.green());
                    }
                }
                println!("总共找到 {} 个匹配项", total_matches);  
            }
            Err(error) => {
                eprintln!("发生错误: {}", error);
                process::exit(1);
            }
        }
    }
    tracing::event!(tracing::Level::INFO, "");//输出日志
}

fn find<P: AsRef<Path>>(
    root: P,
    regex: &Regex,
) -> Result<(Vec<String>, usize, Vec<String>), Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    let mut total_matches = 0;
    let mut all_files = Vec::new();

    walk_tree(root.as_ref(), regex, &mut matches, &mut total_matches, &mut all_files)?;

    Ok((matches, total_matches, all_files))
}

fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<String>,
    total_matches: &mut usize,
    all_files: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches, total_matches, all_files)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                    *total_matches += 1;
                }
            }
            
            all_files.push(path.to_string_lossy().to_string());
            
        }
    }
    Ok(())
}