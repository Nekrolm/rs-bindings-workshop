#include "clib_iface.h"

extern "C" {


uint64_t count_non_zero_pixels(ContiguousImageU8View img) {
    uint64_t cnt = 0;
    for (auto pxl : SliceU8 { img.data, img.size.area() }) {
        cnt += pxl != 0;
    }
    return cnt;
}

void for_each_contiguous_non_zero_pixels_horizontal_line_do(ContiguousImageU8View img, SliceCallback cbk, CallbackContext ctx)
{
    auto [width, height] = img.size;
    auto begin_row = img.data;
    for (uint32_t i = 0; i < height; ++i) {
         auto end_row = begin_row + width;
         while (begin_row != end_row) {
            while (begin_row != end_row && *begin_row == 0) {
                ++begin_row;
            } 
            auto end_of_segment = begin_row;
            while (end_of_segment != end_row && *end_of_segment != 0) {
                ++end_of_segment;
            }

            if (end_of_segment != begin_row) {
                cbk(SliceU8 { begin_row, static_cast<uint64_t>(end_of_segment - begin_row)  }, ctx);
            }
            begin_row = end_of_segment;
         }
    }
}
    
}


