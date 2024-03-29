use mlua::{chunk, Chunk, Lua};

pub fn get_code(context: &Lua) -> Chunk<'_, '_> {
    context.load(chunk!{
        local dorhudModuleCompat = {}
        dorhudModuleCompat.__index = dorhudModuleCompat

        function dorhudModuleCompat.new(name, details)

            local newModule = {
                name = name,
                hooks = {
                    { hook_id = "core/lib/system/coresystem", script_path = "base.lua" }
                }
            }
            if details ~= nil then
                for i, v in next, details do
                    if i == "hooks" then
                    elseif i == "description" then
                        newModule.description = v.english or type(v) == "string" and v or "Couldn't convert description"
                    else
                        newModule[i] = v
                    end
                end
            end
            setmetatable(newModule, dorhudModuleCompat)

            return newModule

        end

        local function ensureLuaExtension(file)
            if string.match(file, "%.lua$") then return file
            else return file .. ".lua" end
        end

        dorhudModuleCompat.register_include = function(self, includePath)
            dofile(includePath)
        end

        dorhudModuleCompat.hook_post_require = function(self, hooking, hook)
            table.insert(
                self.hooks,
                { hook_id = hooking, script_path = ensureLuaExtension(hook) }
            )
        end

        dorhudModuleCompat.register_post_override = dorhudModuleCompat.hook_post_require

        _G.DMod = _G.NoOpTable.new()

        _G.DMod.allModules = {}

        function DMod:new(name, details)
            local moduleObject = dorhudModuleCompat.new(name, details)
            _G.DMod.allModules[name] = moduleObject
            return moduleObject
        end

        _G.D = _G.NoOpTable.new()

        function D:module(name)
            return _G.DMod.allModules[name]
        end

        function D:root_path()
            return "mods/"
        end

        _G.DorHUDMod = _G.DMod
    })
}