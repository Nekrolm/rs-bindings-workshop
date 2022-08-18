#pragma once

#include "rust/cxx.h"
#include <cstdint>
#include <memory>
#include <unordered_map>
#include <string_view>

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

class ByRefSubstitutor {
public:
    rust::Str substitute(rust::Str) const;

    std::unordered_map<std::string_view, std::string_view> replaces;
};

std::unique_ptr<SummatorIface> make_summator();

std::unique_ptr<SubstitutorIface> make_substitutor(rust::Slice<const SubstitutePair>);

std::unique_ptr<ByRefSubstitutor> make_by_ref_substitutor(rust::Slice<const SubstitutePair>);

}