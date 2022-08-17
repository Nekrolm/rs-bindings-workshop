#pragma once

#include "rust/cxx.h"
#include <cstdint>
#include <memory>

namespace cxx_dep {


class SummatorIface {
public:
    virtual ~SummatorIface() = default;
    virtual int64_t summate(rust::Slice<const int64_t>) = 0;
};

std::unique_ptr<SummatorIface> make_summator();

}