#pragma once

#include "rust/cxx.h"
#include <cstdint>
#include <memory>

namespace cxx_dep {

struct SubstitutePair;

class SubstitutorIface {
public:
    virtual ~SubstitutorIface() = default;
    virtual rust::String substitute(rust::Str) const = 0;
};

class SummatorIface {
public:
    virtual ~SummatorIface() = default;
    virtual int64_t summate(rust::Slice<const int64_t>) = 0;
};

std::unique_ptr<SummatorIface> make_summator();

std::unique_ptr<SubstitutorIface> make_substitutor(rust::Slice<const SubstitutePair>);

}