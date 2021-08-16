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

## Todo
1. Rust / Cargo support
2. C Support
3. C# Support (Probably won't happen)