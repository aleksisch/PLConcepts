//
// Created by aleksey on 30.03.2024.
//

#include "Exceptions.h"

thread_local std::optional<detail::Exceptions::ErrorT> detail::Exceptions::err_;
thread_local detail::Exceptions *detail::Exceptions::stackObjects;
void (*detail::Exceptions::fn_)(detail::Exceptions::ErrorT err1, detail::Exceptions::ErrorT err2);
