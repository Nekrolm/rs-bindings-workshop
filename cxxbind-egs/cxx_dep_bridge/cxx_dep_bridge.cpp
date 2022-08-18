#include "cxx_dep_bridge.hpp"

#include "cxxbind-egs/src/bridge.rs.h"

#include <utility>

#include <cstdlib>
#include <unordered_map>



namespace cxx_dep {

struct JustSummator : public SummatorIface {
    int64_t summate(rust::Slice<const int64_t> range) override {
        if (range.empty()) {
            throw std::runtime_error("cannot summate over empty range");
        }
        int64_t val = 0; 
        for (auto x : range) {
            val += x;
        }
        return val;
    }
};

std::unique_ptr<SummatorIface> make_summator() {
    std::string greeating = "summator created with seed ";
    greeating += std::to_string(rand() % 10);
    log_message(greeating);
    return std::make_unique<JustSummator>();
}


struct Substitutor : public SubstitutorIface {
    rust::String substitute(rust::Str str) const override {
        std::string str_copy {str};
        return replaces.at(str_copy);
    }

    explicit Substitutor(std::unordered_map<std::string, std::string> replaces) : replaces{std::move(replaces)} {}

    std::unordered_map<std::string, std::string> replaces;
};

std::unique_ptr<SubstitutorIface> make_substitutor(rust::Slice<const SubstitutePair> pairs) {
    std::unordered_map<std::string, std::string> replaces;
    for (auto [ from, to ] : pairs) {
        std::string from_str { from };
        std::string to_str { to };
        replaces.insert({std::move(from_str), std::move(to_str)});
    }
    return std::make_unique<Substitutor>(std::move(replaces));
}



std::unique_ptr<ByRefSubstitutor> make_by_ref_substitutor(rust::Slice<const SubstitutePair> pairs) {
    std::unordered_map<std::string_view, std::string_view> replaces;
    for (auto [ from, to ] : pairs) {
        std::string_view from_str { from.begin(), from.length() };
        std::string_view to_str {to.begin(), to.length() };
        replaces.insert({from_str, to_str});
    }
    return std::make_unique<ByRefSubstitutor>(ByRefSubstitutor { std::move(replaces) });
}

rust::Str ByRefSubstitutor::substitute(rust::Str str) const {
    std::string_view s { str.begin(), str.length() };
    auto ret = replaces.at(s);
    return rust::Str{ ret.data(), ret.length() };
}

}
