<div align="center" style="margin: 30px;">
     <h1>Packit</h1>
     <div align="center">
        <a href="##Installation">Installation</a> |
        <a href="/">Documentation</a> | 
        <a href="https://github.com/proxin187">Author</a>
    </div>
    <br>
    <strong>A  package manager written in rust.</strong>
</div>

## Description
---
Packit is a package manager with no repository, this means it can install packages from anywhere and is not limited to a repository.

## Roadmap
---
- [x] Installing from package hosted on github/gitlab
- [x] Installing from local package
- [ ] Scripting language for better BUILD files

## Bootstraping
---

```
$ git clone https://github.com/proxin187/packit
```


bootstrap:

```
$ sudo cargo run install
```

## Usage
---

```
Usage: packit [OPTIONS] <COMMAND>

Commands:
  install  
  update   
  help     Print this message or the help of the given subcommand(s)

Options:
  -s, --silent   
  -h, --help     Print help
  -V, --version  Print version
```


## License
---
packit is licensed under the MIT license.

## Contribute
---
packit is currently not open for contributions.



