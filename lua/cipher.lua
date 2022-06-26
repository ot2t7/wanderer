--@BEGIN CRYPTO_HEADER@
-- Some of this is taken from:
-- https://github.com/tst2005/lua-bit-numberlua/blob/master/lmod/bit/numberlua.lua
local MOD = 2^32
local MODM = MOD-1

local function memoize(f)
    local mt = {}
    local t = setmetatable({}, mt)
    function mt:__index(k)
        local v = f(k); t[k] = v
        return v
    end
    return t
end
local function make_bitop_uncached(t, m)
    local function bitop(a, b)
        local res,p = 0,1
        while a ~= 0 and b ~= 0 do
            local am, bm = a%m, b%m
            res = res + t[am][bm]*p
            a = (a - am) / m
            b = (b - bm) / m
            p = p*m
        end
        res = res + (a+b)*p
        return res
    end
    return bitop
end
local function make_bitop(t)
    local op1 = make_bitop_uncached(t,2^1)
    local op2 = memoize(function(a)
        return memoize(function(b)
            return op1(a, b)
        end)
    end)
    return make_bitop_uncached(op2, 2^(t.n or 1))
end
local bxor = make_bitop({[0]={[0]=0,[1]=1},[1]={[0]=1,[1]=0}, n=4});
local function band(a, b, c, ...)
    local z;
    if b then
        a = a % MOD;
        b = b % MOD;
        z = ((a+b) - bxor(a,b)) / 2;
        if c then
           z = band(z, c, ...);
        end;
        return z;
    elseif a then
        return a % MOD;
    else
        return MODM;
    end;
end;
local function bor(a,b)  return MODM - band(MODM - a, MODM - b) end
local lshift, rshift
local function rshift(a,disp)
    if disp < 0 then return lshift(a,-disp) end
    return floor(a % 2^32 / 2^disp)
end
local function lshift(a,disp)
    if disp < 0 then return rshift(a,-disp) end
    return (a * 2^disp) % 2^32
end
local function bytes2word(b0, b1, b2, b3)
    local i = b0; i = lshift(i, 8);
    i = bor(i, b1); i = lshift(i, 8);
    i = bor(i, b2); i = lshift(i, 8);
    i = bor(i, b3);
    return i;
end

local function word2bytes(word)
    local b0, b1, b2, b3;
    b3 = band(word, 0xFF); word = rshift(word, 8);
    b2 = band(word, 0xFF); word = rshift(word, 8);
    b1 = band(word, 0xFF); word = rshift(word, 8);
    b0 = band(word, 0xFF);
    return b0, b1, b2, b3;
end
--@END CRYPTO_HEADER@
--@BEGIN XTEA@
local function decrypt(key, data)
    local y = bytes2word(data[1], data[2], data[3], data[4]);
    local z = bytes2word(data[5], data[6], data[7], data[8]);

    local delta = 0x9e3779b9;
    local sum = 0xc6ef3720; --band(delta*32,0xFFFFFFFF);

    local k0 = bytes2word(key[ 1], key[ 2], key[ 3], key[ 4]);
    local k1 = bytes2word(key[ 5], key[ 6], key[ 7], key[ 8]);
    local k2 = bytes2word(key[ 9], key[10], key[11], key[12]);
    local k3 = bytes2word(key[13], key[14], key[15], key[16]);
    local k = {[0] = k0, k1, k2, k3};

    for _ = 1, 32 do
        local temp;

        temp = bxor(lshift(y, 4), rshift(y, 5)) + y
        temp = bxor(temp, sum + k[ band(rshift(sum, 11), 0x3) ])
        z = band(z + 0x100000000 - temp, 0xFFFFFFFF);

        sum = band(sum + 0x100000000 - delta, 0xFFFFFFFF);

        temp = bxor(lshift(z, 4), rshift(z, 5)) + z
        temp = bxor(temp, sum + k[ band(sum, 0x3) ])
        y = band(y + 0x100000000 - temp, 0xFFFFFFFF);

    end

    local out = {};

    out[1], out[2], out[3], out[4] = word2bytes(y);
    out[5], out[6], out[7], out[8] = word2bytes(z);

    return out;
end
--@END XTEA@