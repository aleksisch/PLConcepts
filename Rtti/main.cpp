//
// Created by aleksey on 06.04.2024.
//

#include <gtest/gtest.h>
#include "RTTI.h"

class A {
public:
    ADD_RTTI(A);
};

class B : public A {
public:
    ADD_RTTI(B, A);
};

class C : public A {
public:
    ADD_RTTI(C, A);
};

class D {
public:
    ADD_RTTI(D);
};

class Multiple : public D, public C {
public:
    ADD_RTTI(Multiple, D, C)
};

class Multiple2 : public B, public C {
public:
    ADD_RTTI(Multiple2, B, C)
};

TEST(RTTI, Simple) {
    A a;
    B b;
    ASSERT_EQ((&a)->GetRTTI().name, "A");
    auto c = DYNAMIC_CAST(A, &b);
    static_assert(std::is_same_v<decltype(c), A*>);
    ASSERT_EQ(c->GetRTTI().name, "B");
    auto d = DYNAMIC_CAST(B, c);
    ASSERT_EQ(d->GetRTTI().name, "B");
    auto e = DYNAMIC_CAST(B, &a);
    ASSERT_EQ(e, nullptr);
}

TEST(RTTI, MultipleInheritance) {
    Multiple b;
    auto d = DYNAMIC_CAST(D, &b);
    auto c = DYNAMIC_CAST(C, &b);
    ASSERT_EQ(d->GetRTTI().name, "Multiple");
    ASSERT_EQ(c->GetRTTI().name, "Multiple");
    ASSERT_EQ(DYNAMIC_CAST(Multiple, d), &b);
    ASSERT_EQ(DYNAMIC_CAST(Multiple, c), &b);
}

TEST(RTTI, RepeatBase) {
    Multiple2 b;
    ASSERT_EQ(DYNAMIC_CAST(D, &b), nullptr); // negative test
    auto a = DYNAMIC_CAST(A, &b);
    ASSERT_EQ(a->GetRTTI().name, "Multiple2");
}

int main (int argc, char **argv) {
    testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}