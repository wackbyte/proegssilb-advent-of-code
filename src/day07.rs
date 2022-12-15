use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

pub type Day7Output = u64;

#[derive(Debug)]
pub enum DataLine {
    CommandCdToRoot,
    CommandCdUpdir,
    CommandCdToSubdir(String),
    CommandLs,
    ListingFile(u64, String),
    ListingDirectory(String),
}

type Path = Vec<String>;

#[derive(Debug)]
struct FileInfo {
    size: u64,
}

#[derive(Debug)]
struct Terminal {
    current_dir: Path,
    fs: Folder,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal { current_dir: vec![], fs: Folder::new() }
    }
}

#[derive(Debug)]
struct Folder {
    subdirs: HashMap<String, Folder>,
    files: Vec<FileInfo>,
    size: u64,
}

impl Folder {
    fn new() -> Folder {
        Folder { subdirs: HashMap::new(), files: vec![], size: 0 }
    }

    fn get_path(&self, p: &Path) -> &Folder {
        let mut current = self;
        for d in p {
            current = current.subdirs.get(d).unwrap()
        }
        current
    }

    fn get_path_mut(&mut self, p: &Path) -> &mut Folder {
        let mut current = self;
        for d in p {
            current = current.subdirs.get_mut(d).unwrap()
        }
        current
    }

    fn compute_size(&mut self) {

        for f in self.subdirs.values_mut() {
            f.compute_size();
        }

        self.size = 0;
        self.size = self.subdirs.values().fold(self.size, |x, y| x + y.size);
        self.size = self.files.iter().fold(self.size, |x, y| x + y.size);
    }

    fn get_all_folders(&self) -> Vec<&Folder> {
        let mut results = vec![self];
        results.extend(self.subdirs.values().flat_map(|d| d.get_all_folders()));
        results
    }
}


#[aoc_generator(day07, default)]
pub fn day07_generator(input: &str) -> Vec<DataLine> {
    let mut results: Vec<DataLine> = Vec::new();
    for line in input.lines() {
        let data = match line.as_bytes()[0] {
            b'$' => {
                if line.trim() == "$ ls" {
                    DataLine::CommandLs
                } else {
                    let (_, cmd) = (*line).split_once(" ").unwrap();
                    let (_, dir) = cmd.split_once(" ").unwrap();
                    match dir {
                        "/" => DataLine::CommandCdToRoot,
                        ".." => DataLine::CommandCdUpdir,
                        _ => DataLine::CommandCdToSubdir(dir.to_string())
                    }
                }
            },
            b'd' => DataLine::ListingDirectory(line.split_whitespace().skip(1).next().unwrap().to_string()),
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                let (size, name) = line.split_once(" ").unwrap();
                DataLine::ListingFile(size.parse().unwrap(), name.to_string())
            }
            _ => panic!("Unknown line: '{}'", line)
        };
        results.push(data)
    }
    
    results
}

fn build_fs_step(term: &mut Terminal, cmd: &DataLine) {
    match cmd {
        DataLine::CommandCdToRoot => { term.current_dir = Vec::new() },
        DataLine::CommandCdUpdir => { term.current_dir.pop(); },
        DataLine::CommandCdToSubdir(subdir) => {
            assert!(term.fs.get_path(&term.current_dir).subdirs.contains_key(subdir), "Tried to cd into directory not known to exist!");
            term.current_dir.push(subdir.clone());
        },
        DataLine::CommandLs => {},
        DataLine::ListingFile(size, _name) => {
            let dir = term.fs.get_path_mut(&term.current_dir);
            dir.files.push(FileInfo {size: *size});
        },
        DataLine::ListingDirectory(name) => {
            let dir = term.fs.get_path_mut(&term.current_dir);
            dir.subdirs.entry(name.clone()).or_insert(Folder { subdirs: HashMap::new(), files: vec![], size: 0 });
        },
    };
}

#[aoc(day07, part1, default)]
pub fn solve_part1(input: &[DataLine]) -> Day7Output {
    let mut t = Terminal::new();
    for dl in input {
        build_fs_step(&mut t, dl);
    }

    let Terminal { current_dir: mut _dir, mut fs } = t;

    fs.compute_size();

    let mut sum = 0u64;

    for f in fs.get_all_folders() {
        if f.size < 100_000 {
            sum += f.size;
        }
    }

    sum
}

#[allow(unused)]
const TEST_INPUT1_STR: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

#[test]
pub fn test_part1() {
    assert_eq!(solve_part1(&day07_generator(TEST_INPUT1_STR)), 95437);
}

#[aoc(day07, part2, default)]
pub fn solve_part2(input: &[DataLine]) -> Day7Output {
    let mut t = Terminal::new();
    for dl in input {
        build_fs_step(&mut t, dl);
    }

    let Terminal { current_dir: mut _dir, mut fs } = t;

    fs.compute_size();

    let free_space = 70000000 - fs.size;
    let space_needed = 30000000 - free_space;

    fs.get_all_folders()
        .into_iter()
        .map(|f| f.size)
        .filter(|s| *s > space_needed)
        .min()
        .unwrap()
}

#[test]
pub fn test_part2() {
    assert_eq!(solve_part2(&day07_generator(TEST_INPUT1_STR)), 24933642);
}