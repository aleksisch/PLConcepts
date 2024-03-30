#include <iostream>
#include <gtest/gtest.h>

#include "Exceptions.h"

std::string res;

class A {
public:
    A(const char *s = "A") : str(s) {}
    ~A() {
        res += str;
    }
    const char *str;
};

class Throwing {
public:
    ~Throwing() {
        THROW(error::io_error);
    }
};

TEST(Exceptions, Simple) {
    res.clear();
    TRY {
        AUTO_OBJECT(A, a);
        THROW(error::io_error);
    } CATCH(error::math_error) {

    } CATCH(error::io_error) {
        res += "I";
    }

    ASSERT_EQ(res, "AI");
}

void crash(error first, error second) {
    res += "C";
}

TEST(Exceptions, crash) {
    res.clear();
    SET_UNEXPECTED_HANDLER(crash);
    TRY {
        AUTO_OBJECT(Throwing, thr);
        THROW(error::math_error);
    }
    ASSERT_EQ(res, "C");
}

TEST(Exceptions, inner) {
    res.clear();
    TRY {
        TRY {
            AUTO_OBJECT(A, thr1, "2");
            TRY {
                AUTO_OBJECT(A, thr, "1");
                THROW(error::math_error);
            }
            AUTO_OBJECT(A, thr2, "never");
        } CATCH(error::math_error) {
            res += "3";
        }
        AUTO_OBJECT(A, thr, "4");
    }
    ASSERT_EQ(res, "1234");
}

int main (int argc, char **argv) {
    testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}