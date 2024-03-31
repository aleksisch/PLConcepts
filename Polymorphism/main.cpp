//
// Created by aleksey on 31.03.2024.
//

#include <gtest/gtest.h>

#include "Polymorphism.h"

class A {
public:
    VIRTUAL_CTOR(A)

    VIRTUAL(A, int, get_x, int(x)) {
        return x + 123;
    }

    VIRTUAL(A, int, get_y, int(x)) {
        return x + 1;
    }

};

class B : public A {
public:
    OVERRIDE_CTOR(B)

    OVERRIDE(B, int, get_x, int(x)) {
        return x;
    }
};


class C : public A {
public:
    OVERRIDE_CTOR(C)

    OVERRIDE(C, int, get_fake, int(x)) {
        return x;
    }
};

TEST(Virtual, TestBase) {
    A a;
    ASSERT_EQ(a.get_x(13), 123 + 13);
    ASSERT_EQ(a.get_y(13), 1 + 13);
    B b;
    ASSERT_EQ(b.get_x(13), 13);
    ASSERT_EQ(b.get_y(13), 1 + 13);
}

TEST(Virtual, Inheritance) {
    B b;
    ASSERT_EQ(reinterpret_cast<A*>(&b)->get_x(13), 13);
    ASSERT_EQ(reinterpret_cast<A*>(&b)->get_y(13), 1 + 13);
}

TEST(Virtual, NonExistent) {
    EXPECT_DEATH(C(), ".*");
}

int main (int argc, char **argv) {
    testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}