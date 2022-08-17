#pragma once

#include <stdint.h>

struct ImageSize {
    uint32_t width;
    uint32_t height;

    uint64_t area() const {
        return uint64_t(width) * height;
    }
};

struct ContiguousImageU8View {
    const uint8_t* data;
    ImageSize size;
};

struct SliceU8 {
    const uint8_t* data;
    uint64_t size;

    auto begin() {
        return data;
    }
    auto end() {
        return data + size;
    }
};

using CallbackContext = void*;
using SliceCallback = void(*)(SliceU8 slice, CallbackContext cbk);

extern "C" {

uint64_t count_non_zero_pixels(ContiguousImageU8View img);
void for_each_contiguous_non_zero_pixels_horizontal_line_do(ContiguousImageU8View img, SliceCallback cbk, CallbackContext ctx);

}