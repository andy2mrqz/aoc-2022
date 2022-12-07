use itertools::Itertools;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Dir {
    name: String,
    parent: Option<Box<Dir>>,
    children: Vec<Content>
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct File {
    name: String,
    size: u32,
    parent: Option<Box<Dir>>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Content {
    F(File),
    D(Box<Dir>),
}

fn solve(input: &str) -> usize {
    let mut pwd: Dir = Dir {
        name: "/".to_string(),
        parent: None,
        children: Vec::new()
    };

    for line in input.lines().skip(1) {
        println!("{}", line);
        if line == "$ cd .." {
            if let Some(parent) = pwd.parent {
                pwd = *parent;
            }
        } else if line.starts_with("$ cd ") {
            let dir_name = line.split("$ cd ").collect::<Vec<&str>>()[1].to_string();
            let dir = Dir {
                name: dir_name.clone(),
                parent: Some(Box::new(pwd.clone())),
                children: Vec::new()
            };
            let content = Content::D(Box::new(dir.clone()));
            pwd.children.push(content);
            pwd = dir;
        } else if line.starts_with("dir") {
            let dir_name = line.split("dir ").collect::<Vec<&str>>()[1].to_string();
            let dir = Dir {
                name: dir_name,
                parent: Some(Box::new(pwd.clone())),
                children: Vec::new()
            };
            pwd.children.push(Content::D(Box::new(dir.clone())));
        } else if line.starts_with(|c: char| c.is_numeric()) {
            let (size, name) = line.split(" ").collect_tuple().unwrap();
            let size: u32 = size.parse().unwrap();

            let file = File {
                name: name.to_string(),
                size,
                parent: Some(Box::new(pwd.clone()))
            };
            pwd.children.push(Content::F(file.clone()));
        }
    }

    println!("{:?}", pwd);

    10
}

pub fn main() {
    let input = include_str!("../inputs/07ex.txt");

    println!("part one: {}", solve(input)); //
    println!();
    // println!("part two: {}", solve(input)); //
}
