# CrabRave

CrabRave is a programming language based on Brainfuck that uses crab and sea-life emoji instead of boring characters.

It first transpiles to [`Crab`](https://github.com/crablang/crab), which is then compiled to a binary executable.

## Usage

### Requirements

You'll need the `crabc` compiler installed to use CrabRave. On Unix it can be installed with the CrabLang toolchain:

```sh
sh <(curl https://install.crablang.org -L)
```

### Compiling CrabRave

After cloning the repo, you can compile a CrabRave file using the following command:

```sh
crabgo run -- /path/to/file.rave
```

Then run the file named "compiled".

## Notes

Please don't actually use CrabRave.
