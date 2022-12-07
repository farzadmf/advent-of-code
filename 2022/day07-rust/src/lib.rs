use std::fs;

#[derive(Debug, Copy)]
pub struct File {
    pub size: usize,
}

#[derive(Debug, Copy)]
pub struct Dir {
    pub files: Vec<File>,
    pub dirs: Vec<Dir>,
    pub parent: Option<Box<Dir>>,
    pub is_root: bool,
    pub name: String,
}

impl Dir {
    pub fn new(name: &str, parent: Dir) -> Dir {
        return Dir {
            files: Vec::new(),
            dirs: Vec::new(),
            parent: Some(Box::from(parent)),
            is_root: false,
            name: name.to_string(),
        };
    }

    pub fn root() -> Dir {
        return Dir {
            files: Vec::new(),
            dirs: Vec::new(),
            parent: None,
            is_root: true,
            name: "/".to_string(),
        };
    }
}

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn change_dir(cur_dir: Dir, target: &str, root_dir: Dir) -> Dir {
    match target {
        ".." => match cur_dir.parent {
            Some(parent) => *parent,
            _ => root_dir,
        },
        dir_name => {
            if cur_dir.dirs.iter().any(|d| d.name == dir_name) {
                let position = cur_dir
                    .dirs
                    .iter()
                    .position(|d| d.name == dir_name)
                    .unwrap();
                cur_dir.dirs[position]
            } else {
                Dir::new(dir_name, cur_dir)
            }
        }
        "/" => root_dir,
    }
}
