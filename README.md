# gitz

`gitz` extends Git with extra commands that make it easier to work with multiple hosting platforms simultaneously — GitHub, GitLab, Bitbucket, or your own corporate Git server.

## Why gitz?

If you store your code organized by domain and path, like this:

```
~/Projects/
  github.com/
    myorg/
      myrepo/
  gitlab.com/
    anotherorg/
      anotherrepo/
  git.corp.example.com/
    team/
      project/
```

...then `gitz` automates the tedious part: instead of manually creating directories and cloning into them, a single command does it all.

## Commands

### `create`

Clones a repository into an automatically created directory based on the repository's domain and path.

```sh
gitz create https://github.com/pahanini/gitz
# Clones into ~/Projects/github.com/pahanini/gitz

gitz create git@gitlab.com:myorg/myrepo.git
# Clones into ~/Projects/gitlab.com/myorg/myrepo

gitz create https://git.corp.example.com/team/project.git
# Clones into ~/Projects/git.corp.example.com/team/project
```

## Configuration

| Environment variable | Default      | Description                            |
|----------------------|--------------|----------------------------------------|
| `GITZ_HOME`          | `~/Projects` | Base directory for cloned repositories |

## Installation

### Homebrew

```sh
brew tap pahanini/gitz
brew install gitz
```

### From source

Requires [Rust](https://rustup.rs).

```sh
git clone https://github.com/pahanini/gitz
cd gitz
cargo install --path .
```

## Usage

`gitz` passes all unrecognized commands directly to `git`, so you can use it as a drop-in replacement:

```sh
gitz status
gitz log --oneline
gitz push origin main
```

## License

MIT