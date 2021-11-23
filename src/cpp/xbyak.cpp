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

extern "C" struct Add : Xbyak::CodeGenerator
{
    Add(int m, int n)
    {
        mov(eax, m);
        add(eax, n);
        ret();
    }
};

extern "C" int _add(int m, int n)
{
    Add a(m, n);
    auto (*f)(int, int) = a.getCode<int (*)(int, int)>();
    return f(m, n);
}
