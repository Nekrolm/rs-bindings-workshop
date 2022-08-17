#include "cxx_dep_bridge.hpp"

#include "cxxbind-egs/src/bridge.rs.h"

#include <utility>

#include <cstdlib>



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

}
