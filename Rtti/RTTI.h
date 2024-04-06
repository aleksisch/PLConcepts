//
// Created by aleksey on 06.04.2024.
//

#pragma once

#include <atomic>
#include <vector>

namespace detail {
    template <typename T>
    struct GenId {
        int getIdx() {
            static std::atomic<int> cur = 0;
            return cur.fetch_add(1);
        }
    };

    struct RTTI {
    private:
        RTTI(const char *name, std::vector<RTTI*> parents) : name(name), parents(parents) {}
    public:

        template <class ...Args>
        static RTTI create(const char *name) {
            return RTTI{name, std::vector<RTTI*>{&Args::rtti_ ...}};
        }

        template <typename T, typename V>
        static T* DynamicCast(V *self) {
            if (Inherited(&(self->GetRTTI()), &(T::rtti_))) {
                return reinterpret_cast<T*>(self);
            } else {
                return nullptr;
            }
        }

        static bool Inherited(const RTTI *child, const RTTI *par) {
            if (child == par) {
                return true;
            } else {
                for (const auto &parr: child->parents) {
                    if (Inherited(parr, par)) {
                        return true;
                    }
                }
                return false;
            }
        }

        const char *name;
    private:
        std::vector<RTTI*> parents; // we may use flatten unordered map of parents to speed up lookup
    };
}

#define ADD_RTTI(ClassName, ...) \
friend class detail::RTTI;                                 \
static inline detail::RTTI rtti_ = detail::RTTI::create< __VA_ARGS__ >(#ClassName);                                 \
virtual const detail::RTTI &GetRTTI() {         \
    return rtti_;\
}

#define DYNAMIC_CAST(ToClass, ptr) detail::RTTI::DynamicCast<ToClass>(ptr)
