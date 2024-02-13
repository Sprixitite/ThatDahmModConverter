use mlua::{chunk, Chunk, Lua};

pub fn get_code(context: &Lua) -> Chunk<'_, '_> {
    context.load(chunk!{
        if _G.NoOpTable then return end

        _G.NoOpTable = {}

        local arithmeticOps = {
            __add = function(a,b) return a+b end,
            __sub = function(a,b) return a-b end,
            __mul = function(a,b) return a*b end,
            __div = function(a,b) return a/b end,
            __mod = function(a,b) return a%b end,
            __pow = function(a,b) return a^b end
        }

        local function noOpString() return "" end
        local function noOpNumber() return 0 end
        local function noOpBool() return false end

        local function isNoOpTable(candidate)
            if candidate == nil then return false end
            if type(candidate) ~= "table" then return false end
            return getmetatable(candidate) == NoOpTable
        end

        local function replaceArithmeticOps(t)
            for k, v in next, arithmeticOps do
                local oldOp = t[k] or v
                t[k] = function(a,b)
                    local aIsNoOp = isNoOpTable(a) local bIsNoOp = isNoOpTable(b)
                    if aIsNoOp and bIsNoOp then return 0
                    elseif not aIsNoOp and not bIsNoOp then return oldOp(a, b) end
                    return aIsNoOp and b or a
                end
            end
        end

        local function replaceConcat(t)
            local oldConcat = t.__concat or function(a,b) return a..b end
            t.__concat = function(a,b)
                local aIsNoOp = isNoOpTable(a) local bIsNoOp = isNoOpTable(b)
                if aIsNoOp and bIsNoOp then return ""
                elseif not aIsNoOp and not bIsNoOp then return oldConcat(a, b) end
                return aIsNoOp and b or a
            end
        end

        local numberMeta = debug.getmetatable( 0 ) or {}
        local stringMeta = debug.getmetatable( "" ) or {}

        replaceArithmeticOps(numberMeta)
        replaceConcat(stringMeta)

        debug.setmetatable( "", stringMeta )
        debug.setmetatable( 0, numberMeta )

        function NoOpTable.new()
            local newOpTable = {
                timesCalled = 0,
                addedInfo = {}
            }
            setmetatable(newOpTable, NoOpTable)
            return newOpTable
        end

        function NoOpTable:__index(key)
            local userAdded = self.addedInfo[key]
            return userAdded or self
        end

        function NoOpTable:__newindex(key, value)
            self.addedInfo[key] = value
        end

        function NoOpTable:__len()
            return #self.addedInfo 
        end

        function NoOpTable:__pairs()
        return pairs(self.addedInfo)
        end

        function NoOpTable:__call(args)
            self.timesCalled = self.timesCalled + 1
            return self
        end

        function NoOpTable:__add(other)
            return other
        end

        NoOpTable.__tostring = noOpString
        NoOpTable.__eq = noOpBool
        NoOpTable.__le = noOpBool
        NoOpTable.__lt = noOpBool
        NoOpTable.__unm = noOpNumber
        replaceArithmeticOps(NoOpTable)
        replaceConcat(NoOpTable)
    })
}