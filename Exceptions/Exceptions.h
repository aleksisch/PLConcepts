//
// Created by aleksey on 30.03.2024.
//

#pragma once

#include <stack>
#include <memory>
#include <csetjmp>
#include <optional>
#include <cassert>

#include "Errors.h"

namespace detail {
class AutoObject {
public:
    virtual ~AutoObject() {};
};

using ObjectHolder = std::stack<std::unique_ptr<AutoObject>>;

class Exceptions {
    using ErrorT = error;


    template <typename T>
    class AutoWeak {
    public:
        AutoWeak(T *ptr) : ptr_(ptr) {}
        AutoWeak(const AutoWeak &) = delete;
        AutoWeak(AutoWeak &&weak) : ptr_(weak.ptr_), cur_(weak.cur_) {
            weak.cur_ = nullptr;
            weak.ptr_ = nullptr;
        }
        ~AutoWeak() {
            if (cur_ != nullptr && Exceptions::Top() == cur_) {
                Exceptions::Top()->Pop();
            }
        }

        T &GetValue() {
            return *ptr_;
        }

        const T &GetValue() const {
            return *ptr_;
        }

    private:
        T *ptr_;
        Exceptions *cur_;
    };


    template <typename T>
    class AutoImpl: public AutoObject {
        template <typename ...Args>
        AutoImpl(Args... args) : data_(std::forward<Args>(args)...) {}

    public:
        template <typename ...Args>
        static std::pair<AutoWeak<T>, std::unique_ptr<AutoImpl<T>>> Create(Args... args) {
            auto strong = std::unique_ptr<AutoImpl>(new AutoImpl(std::forward<Args>(args)...));
            AutoWeak<T> weak = &(strong->data_);
            return {std::move(weak), std::move(strong)};
        }

    private:
        T data_;
    };

public:
    Exceptions() : prev_(Exceptions::Top()) {
        stackObjects = this;
    }

    static Exceptions *Top() {
        return stackObjects;
    }

    void SetError(ErrorT err) {
        if (err_) {
            fn_(err, err_.value());
            err_.reset();
        } else {
            err_.emplace(err);
        }
    }


    static bool MatchError(ErrorT err) {
        assert(err_);
        if (err_ == err) {
            err_.reset();
            return true;
        }
        return false;
    }

    void Pop() {
        objects_.pop();
    }

    static std::jmp_buf &Throw() {
        return Top()->head_;
    }

    void ClearObjs() {
        while (!objects_.empty()) {
            objects_.pop();
        }
    }

    ~Exceptions() {
        ClearObjs();
        stackObjects = prev_;
        if (err_) {
            std::longjmp(detail::Exceptions::Throw(), 1);
        }
    }

    template <typename T, typename ...Args>
    static AutoWeak<T> Create(Args... args) {
        auto [weak, strong] = AutoImpl<T>::Create(std::forward<Args>(args)...);
        Top()->objects_.push(std::move(strong));
        return std::move(weak);
    }

    static void SetHandler(void(*fn)(error first, error second)) {
        fn_ = fn;
    }

private:
    static thread_local Exceptions *stackObjects;
    static thread_local std::optional<ErrorT> err_;
    Exceptions *prev_;
    std::jmp_buf head_;
    ObjectHolder objects_;
    static void (*fn_)(ErrorT err1, ErrorT err2);
};

};

#define TRY if (detail::Exceptions exception; setjmp(exception.Throw()) == 0)
#define CATCH(err) else if (exception.ClearObjs(), detail::Exceptions::MatchError(err))
#define THROW(err) detail::Exceptions::Top()->SetError(err); std::longjmp(detail::Exceptions::Throw(), 1)
#define AUTO_OBJECT(Type, name, ...) auto name = detail::Exceptions::Create<Type>(__VA_ARGS__)
#define SET_UNEXPECTED_HANDLER(crash) detail::Exceptions::SetHandler(crash)