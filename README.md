## ``gm_binpkg``
Package manager for garrysmod binary modules.  
Clone and build github repos to DLLs to use in garrysmod.  

## Commands
* ``clone`` - Clones a github repo into a cache to be built with ``build``.
* ``build`` - Builds a repo into a DLL to be installed with ``install``.
* ``install`` - Moves a DLL from an existing and compiled package into the ``garrysmod/lua/bin`` folder.
* ``verify`` - Verifies the integrity of a package's DLL. (Does basic checks for now.)

## Supported Languages / Compilers
* C++ (GCC + MSBuild)

# Installation
1. Download the latest exe from the releases page or from github actions artifacts.
2. Add the directory you put the exe in to your PATH.
## Todo
1. Rust / Cargo support
2. C Support
3. C# Support (Probably won't happen)