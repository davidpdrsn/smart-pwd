#![allow(clippy::redundant_closure)]

use itertools::{Itertools, Position};
use std::env;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

fn main() -> Result<()> {
    let pwd = env::current_dir()?;

    let mut ancestors = pwd.ancestors().collect::<Vec<_>>();
    ancestors.reverse();

    for pos in ancestors.iter().with_position() {
        match pos {
            Position::First(_) => print!("/"),
            Position::Only(_) => print!("/"),
            _ => {}
        }

        {
            let inner = pos.into_inner();

            if inner.parent().is_some() {
                match pos {
                    Position::Last(_) | Position::Only(_) => {
                        print!("{}", path_file_name_to_string(inner).unwrap());
                    }
                    _ => {
                        let name = shortest_unique_path_prefix(inner);
                        print!("{}", name);
                    }
                }
            }
        }

        match pos {
            Position::Middle(_) => print!("/"),
            Position::Last(_) => {}
            Position::First(_) => {}
            Position::Only(_) => {}
        }
    }

    println!();

    Ok(())
}

fn shortest_unique_path_prefix(path: &Path) -> String {
    let name = path_file_name_to_string(path).unwrap();
    let contents = dirs_in(path, &name);
    shortest_unique_prefix(&name, &contents).to_string()
}

fn dirs_in(path: &Path, name: &str) -> Vec<String> {
    let mut contents = path
        .parent()
        .expect("no parent")
        .read_dir()
        .expect("read_dir failed")
        .map(|entry| entry.unwrap().path())
        .filter(|entry| entry.is_dir())
        .map(|entry| path_file_name_to_string(&entry).unwrap())
        .filter(|entry_name| entry_name != name)
        .collect::<Vec<_>>();
    contents.sort_unstable();
    contents
}

fn path_file_name_to_string(path: &Path) -> Option<String> {
    Some(path.file_name()?.to_str()?.to_string())
}

fn shortest_unique_prefix<'a, S: AsRef<str>>(name: &'a str, others: &[S]) -> &'a str {
    for n in 0..name.len() {
        let sub = &name[0..n];
        if others
            .iter()
            .find(|other| other.as_ref().starts_with(sub))
            .is_none()
        {
            return sub;
        }
    }

    name
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_something() {
        let name = "minor";
        let dirs = vec!["archive", "bin", "major", "reference"];

        assert_eq!("mi", shortest_unique_prefix(&name, &dirs));
    }
}
