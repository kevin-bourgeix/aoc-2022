use crate::parser::parse_aoc_file;

enum Cmd {
    Cd(String),
    Ls,
}

#[derive(Debug)]
struct Directory {
    path: Vec<String>,
    file_size: u32,
    sub_dirs: Vec<Vec<String>>,
}

impl Directory {
    fn new(path: Vec<String>) -> Directory {
        Directory {
            path,
            file_size: 0,
            sub_dirs: Vec::new(),
        }
    }
}

// parses a command line starting with $
// if it is a result of a command, this command will return None
fn parse_command_line(line: &str) -> Option<Cmd> {
    let mut splits = line.split(' ').collect::<Vec<&str>>();
    splits.reverse();
    let Some(first) = splits.pop() else {
        return None;
    };
    if first != "$" {
        return None;
    }
    let Some(second) = splits.pop() else {
        return None;
    };
    if second == "cd" {
        let Some(third) = splits.pop() else {
            return None;
        };
        return Some(Cmd::Cd(third.to_string()));
    }
    if second == "ls" {
        return Some(Cmd::Ls);
    }
    None
}

// parses a result line, which is always after a ls command
// if it is a dir, put it in the sub_dirs of the current directory
// otherwise is is a file, so add its size to the current directory
fn parse_ls_line(line: &str, directory: &mut Directory, pwd: &[String]) {
    let mut splits = line.split(' ').collect::<Vec<&str>>();
    splits.reverse();
    let Some(first) = splits.pop() else {
        return;
    };
    if first == "dir" {
        let Some(second) = splits.pop() else {
            return;
        };
        let mut new_dir = pwd.to_owned();
        new_dir.push(second.to_string());
        directory.sub_dirs.push(new_dir);
        return;
    }
    directory.file_size += first
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("Could not parse {first} as u32"));
}

// dirty recursive function that i could have derecursified but it was ok performance wise
fn compute_dir_size(directory: &Directory, all: &Vec<Directory>) -> u32 {
    directory.file_size
        + directory
            .sub_dirs
            .iter()
            .map(|s| {
                let Some(dir) = all.iter().find(|d| d.path == *s) else {
            panic!("Could not find directory {s:?}");
        };
                compute_dir_size(dir, all)
            })
            .sum::<u32>()
}

// compute all directories, their size without subdirectories and their subdirectories
fn compute_directories(filename: &str) -> Vec<Directory> {
    let mut lines = parse_aoc_file(filename, None);
    lines.reverse();

    let mut directories: Vec<Directory> = Vec::new();
    let mut pwd: Vec<String> = Vec::new();
    let mut current_dir: Directory = Directory::new(Vec::new());

    // Fill directories first, without calculating additional file sizes
    loop {
        let Some(line) = lines.pop() else {
            break;
        };
        let Some(command) = parse_command_line(&line) else {
            parse_ls_line(&line, &mut current_dir, &pwd);
            continue;
        };

        match command {
            Cmd::Cd(path) => {
                if path == ".." {
                    pwd.pop();
                } else if path == "/" {
                    pwd.clear()
                } else {
                    pwd.push(path);
                }
            }
            Cmd::Ls => {
                directories.push(current_dir);
                current_dir = Directory::new(pwd.clone());
            }
        }
    }

    // Push last current_dir
    directories.push(current_dir);

    directories
}

pub fn day_7_1(filename: &str) -> u32 {
    let directories = compute_directories(filename);

    directories.iter().fold(0, |acc, d| {
        let size = compute_dir_size(d, &directories);
        if size <= 100000 {
            acc + size
        } else {
            acc
        }
    })
}

pub fn day_7_2(filename: &str) -> u32 {
    let directories = compute_directories(filename);

    // get 2nd directory, which is the computed root, above algorithm inserts an empty dir at first position.
    let Some(first_dir) = directories.get(1) else {
        panic!("Could not find root directory");
    };
    let root_size = compute_dir_size(first_dir, &directories);
    let unused = 70000000 - root_size;
    let to_free = 30000000 - unused;
    let max = root_size;

    directories.iter().fold(max, |acc, d| {
        let size = compute_dir_size(d, &directories);
        if size > to_free && size <= acc {
            size
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::exercises::day7::day_7_2;

    use super::day_7_1;

    #[test]
    fn test_day_7_1() {
        assert_eq!(day_7_1("src/files/day7_1.test"), 95437);
    }

    #[test]
    fn test_day_7_2() {
        assert_eq!(day_7_2("src/files/day7_1.test"), 24933642);
    }
}
