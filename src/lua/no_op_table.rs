use mlua::{chunk, Chunk, Lua};

pub fn get_code(context: &Lua) -> Chunk<'_, '_> {
    context.load(chunk!{
        if _G.NoOpTable then return end

        _G.NoOpTable = {}

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
    })
}