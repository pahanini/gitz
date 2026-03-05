use git_url_parse::{GitUrl, GitUrlParseError};
use std::env;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

const HELP: &str = "\
Gitz extends git with extra commands:
  load    Clone repository into an auto-created domain/path directory

";

fn vcs_url_to_path(vcs_url: &str) -> Result<String, GitUrlParseError> {
    let parsed = GitUrl::parse(vcs_url)?;
    let host = parsed.host().unwrap_or("");
    let path = parsed
        .path()
        .trim_start_matches('/')
        .trim_end_matches(".git");
    Ok(format!("{host}/{path}"))
}

fn project_base_dir() -> PathBuf {
    env::var("GITZ_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().join("Projects"))
}

fn create_project_dir(path: &str) -> std::io::Result<String> {
    let full_path = project_base_dir().join(path);
    std::fs::create_dir_all(&full_path)?;
    Ok(full_path.to_string_lossy().into_owned())
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "help" {
        print!("{}", HELP);
    } else if args[0] == "load" {
        let url = args.last().unwrap().clone();
        let path = vcs_url_to_path(&url).unwrap_or_else(|e| {
            eprintln!("invalid vcs url: {e}");
            std::process::exit(1);
        });
        let full_path = create_project_dir(&path).unwrap_or_else(|e| {
            eprintln!("failed to create directory: {e}");
            std::process::exit(1);
        });
        args[0] = "clone".to_string();
        args.push(full_path);
    }

    let error = Command::new("git").args(&args).exec();
    eprintln!("can not exec git: {error}");
    std::process::exit(127);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vcs_url_to_path() {
        let cases = vec![
            ("https://github.com:3000/pahanini/test",                    "github.com/pahanini/test"),
            ("https://github.com/user/repo.git",                         "github.com/user/repo"),
            ("git@gitlab.com:user/repo.git",                             "gitlab.com/user/repo"),
            ("git@gitlab.example.com:user/department/project/repo.git",  "gitlab.example.com/user/department/project/repo"),
            ("ssh://git@bitbucket.org/user/repo.git",                    "bitbucket.org/user/repo"),
            ("ssh://git@gitlab.domain.com/group/subgroup/user/repo.git", "gitlab.domain.com/group/subgroup/user/repo"),
        ];

        for (input, expected) in cases {
            assert_eq!(
                vcs_url_to_path(input).unwrap(),
                expected,
                "failed for input: {input}",
            );
        }
    }
}