local DATA_PACKED = "helloworld";

-- @BEGIN BYTECODE_READER@
local bytecodeCounter = 1;

-- If pop is true, then only the content will be popped off the 
-- bytecode. Everything skipped leading up to the start will 
-- be kept.
local function popBytecode(amount, pop, offset)
    offset = offset or 0;
    local req = string.sub(DATA_PACKED, bytecodeCounter + offset, bytecodeCounter + offset + amount - 1);
    if pop then
        if offset == 0 then
            bytecodeCounter = bytecodeCounter + amount;
            return req;
        else 
            DATA_PACKED = string.sub(DATA_PACKED, bytecodeCounter, bytecodeCounter + offset - 1)..string.sub(DATA_PACKED, bytecodeCounter + offset + amount, #DATA_PACKED);
            return req;
        end
    else
        return req;
    end
end

local function readUntil(token, amount)
    
end
-- @END BYTECODE_READER@