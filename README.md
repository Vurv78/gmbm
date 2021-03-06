# ``gmbm`` [![Release Shield](https://img.shields.io/github/v/release/Vurv78/gmbm)](https://github.com/Vurv78/gmbm/releases/latest) ![CI](https://github.com/Vurv78/gmbm/actions/workflows/ci.yml/badge.svg) [![github/Vurv78](https://img.shields.io/discord/824727565948157963?label=Discord&logo=discord&logoColor=ffffff&labelColor=7289DA&color=2c2f33)](https://discord.gg/epJFC6cNsw)
> Binary module manager for garrysmod.  

## Commands
* ``init`` - Initializes gmbm at the program's dir. You will need to run this when first installing gmbm in the future, but right now it is optional.
* ``clone`` - Clones a github repo into a cache to be built with ``build``.
* ``build`` - Builds a repo into a DLL to be installed with ``install``.
* ``install`` - Moves a DLL from an existing and compiled package into the ``garrysmod/lua/bin`` folder.
* ``verify`` - Verifies the integrity of a package's DLL. (Does basic checks for now.)

## Supported Languages / Compilers
* Rust (Cargo)
* C/C++ (MSBuild & CMake)

## Installation
1. Download the latest exe from the releases page or from github actions artifacts.
2. Add the directory you put the exe in to your PATH.

## Package Install Example
```bash
# Download the git repo on your machine (Also downloads submodules ofc)
gmbm clone https://github.com/Derpius/VisTrace

# Build the .dll
gmbm build vistrace

# Place the DLL in your gmod directory as gmcl_vistrace_win64.dll (Changes depending on your arch and machine)
gmbm install vistrace

# From here, just require("vistrace") inside of gmod, and you're good to go!
```
