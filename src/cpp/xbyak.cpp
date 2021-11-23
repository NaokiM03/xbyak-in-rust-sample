#include <xbyak/xbyak_util.h>

extern "C" struct ReturnSameValue : Xbyak::CodeGenerator
{
    ReturnSameValue(int x)
    {
        mov(eax, x);
        ret();
    }
};

extern "C" int _return_same_value(int x)
{
    ReturnSameValue r(x);
    auto (*f)() = r.getCode<int (*)()>();
    return f();
}
