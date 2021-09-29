local PROJ_PATH, BUILD_PATH = os.getenv("PROJ_PATH"), os.getenv("BUILD_PATH")

-- Assert that the premake5.lua file does really exist (just in case)
assert( os.isfile(PROJ_PATH) )

location( BUILD_PATH )

local old = _G.location
_G.location = function() end -- Disable location function

dofile( PROJ_PATH ) -- Run project's premake5.lua file.

_G.location = old

-- Extra forced configs
defines { "GMMODULE" }
targetdir( BUILD_PATH .. "\\bin" )
kind "SharedLib"