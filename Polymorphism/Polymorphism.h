//
// Created by aleksey on 31.03.2024.
//

#pragma once

#include <functional>
#include <unordered_map>
#include <string>
#include <memory>

namespace detail {

template <typename T>
struct function_traits : public function_traits<decltype(&T::operator())>
{};

template <typename ClassType, typename ReturnType, typename... Args>
struct function_traits<ReturnType(ClassType::*)(Args...) const>
// we specialize for pointers to member function
{
    using result_type = ReturnType;
    using arg_tuple = std::tuple<Args...>;
    template <typename T>
    using functionT = std::function<T(Args...)>;
    static constexpr auto arity = sizeof...(Args);
};

template <class ReturnT, class F, std::size_t ... Is, class T>
auto lambda_to_func_impl(F f, std::index_sequence<Is...>, T) {
    return std::function<ReturnT(std::tuple_element_t<Is, typename T::arg_tuple>...)>(f);
}

template <class ReturnT, class F>
auto lambda_to_func(F f) {
    using traits = function_traits<F>;
    return lambda_to_func_impl<ReturnT>(f, std::make_index_sequence<traits::arity>{}, traits{});
}

class VirtualTable {
public:
    class BaseMethod {

    };

    template <class CbT>
    class Method : public BaseMethod {
    public:
        Method(CbT cb) {
            cb_ = cb;
        }
        CbT cb_;
    };


    template <class CbT, class ...Args>
    auto Call(std::string name, Args &&...args) {
        return reinterpret_cast<Method<CbT>*>(methods.at(name).get())->cb_(std::forward<Args>(args)...);
    }

    template <class CbT>
    int Add(std::string name, CbT cb) {
        methods[name] = std::make_unique<Method<CbT>>(cb);
        return 0;
    }

    void Extend(const VirtualTable &other, bool is_override = true) {
        for (const auto [k, v]: other.methods) {
            if (is_override && !methods.count(k)) {
                std::abort();
            } else {
                methods[k] = v;
            }
        }
    }

    std::unordered_map<std::string, std::shared_ptr<BaseMethod>> methods;
};

}

#define VIRTUAL(class_, returnT, name, ...) \
    using functionT##name = detail::function_traits<decltype([](__VA_ARGS__){})>::functionT<returnT>;\
    int unused_create_##name = class_vtable.Add<functionT##name>(#name,             \
        [this](__VA_ARGS__) -> returnT {                          \
            return actual_##name(__VA_ARGS__);                                 \
        });\
    returnT name(__VA_ARGS__) {         \
        return vtable.Call<functionT##name>(#name, __VA_ARGS__);\
    }                                    \
    returnT actual_##name(__VA_ARGS__)

#define VIRTUAL_CTOR(class_) \
    detail::VirtualTable vtable;      \
                              \
    class_() {                                    \
        this->vtable.Extend(class_vtable, false); \
    }                                             \
    static inline detail::VirtualTable class_vtable;

#define OVERRIDE_CTOR(class_) \
    class_() {                \
        this->vtable.Extend(class_vtable); \
    }                                             \
    static inline detail::VirtualTable class_vtable;

#define OVERRIDE(class_, returnT, name, ...) \
    using functionT##name = detail::function_traits<decltype([](__VA_ARGS__){})>::functionT<returnT>;\
    int unused_create_##name = class_vtable.Add<functionT##name>(#name,             \
        [this](__VA_ARGS__) -> returnT {                          \
            return actual_##name(__VA_ARGS__);                                 \
        });\
    returnT name(__VA_ARGS__) {         \
        return vtable.Call<functionT##name>(#name, __VA_ARGS__);\
    }                                    \
    returnT actual_##name(__VA_ARGS__)
