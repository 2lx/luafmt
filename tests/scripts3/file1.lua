#!/usr/bin/lua-all

--[[first comment]]
function fn1(arg1, arg2)
    for i = arg1, arg2, 1 do --[[ comm ]]
        print(i)

        if arg1 + (arg2 // 2) > i then
            print(arg1 % arg2)
        end
    end
-- end of for
end

function Value:__eq(other)
    return self._value_t == other._value_t
       and ((not self._compare_values and not other._compare_values and not other._compare_values and not other._compare_values and not other._compare_values) or self._value == other._value)
       and ((not self._compare_values and not other._compare_values) or self._value == other._value or self._value == other._value or self._value == other._value or self._value == other._value)
       and ((not self._compare_values and not other._compare_values and not other._compare_values) or self._value == other._value)
       and ((not self._compare_values and not other._compare_values) or self._value == other._value or self._value == other._value or self._value == other._value)
end


local a = { {
    {
        a = 1,
        b = 2,
    },
    {
        a = 2,
        b = 3,
    },
}, {
    {
        a = 1,
        b = 4,
    }
}, }

local a = { { { { { {
    a = 1, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf"
}, {
    a = 2, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf"
}, }, { {
    a = 1, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf"
}, {
    a = 2, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf"
}, {
    a = 3, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf"
}, }, { {
    a = 1, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf"
}, {
    a = 2, b = 2, c = "asdfj;lkfkjsalfkjfsdsffdsfdsfdsfsddsfsdfsdfdsf"
}, {
    a = 3, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsddsfsdfsdfdsff"
}, {
    a = 4, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf"
}, } } }, { { {
    a = 1, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf"
} }, { {
    a = 2, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf"
}, {
    a = 3, b = 2, c = "asdfj;lkfkjsalfkjfsdsfdsfdsfdsfsdfdsfsdfsdfdsf"
}, }, } } }

