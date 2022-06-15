---`START REFERENCES`---
local floor = math.floor
---`END REFERENCES`---

---`START BITLIB`---
local size_word = 2^32
local lshift, rshift

function rshift(a, disp) 
    if disp < 0 then return lshift(a,-disp) end
    return floor(a % size_word / 2^disp)
end

function lshift(a, disp) 
    if disp < 0 then return rshift(a,-disp) end
    return (a * 2^disp) % size_word
end

local function band() 
    
end
---`END BITLIB`---