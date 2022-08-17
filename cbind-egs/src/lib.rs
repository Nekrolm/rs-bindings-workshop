mod clib_dep;

#[derive(Debug, Clone, Copy)]
pub struct ContiguousImageU8View<'a> {
    pub width : usize,
    pub data_slice : &'a[u8], // requires data_slice.len() % width == 0
}


unsafe fn take_cdep_image_ref(img: ContiguousImageU8View<'_>) -> clib_dep::ContiguousImageU8View {
    let ContiguousImageU8View { width, data_slice } = img;
    let (data, len) = (data_slice.as_ptr(), data_slice.len());
    let height = len / width;
    clib_dep::ContiguousImageU8View {
        data,
        size : clib_dep::ImageSize { width: width as _, height: height as _ }
    }
}

unsafe fn cdep_slice_to_slice<'a>(s : clib_dep::SliceU8) -> &'a [u8] {
    std::slice::from_raw_parts(s.data, s.size as _)
}

pub fn count_non_zero_pixels(img: ContiguousImageU8View<'_>) -> usize {
    (unsafe {
        clib_dep::count_non_zero_pixels(take_cdep_image_ref(img))
    }) as usize
}


pub fn for_each_contiguous_non_zero_pixels_horizontal_line_do<'a, F>(img: ContiguousImageU8View<'a>, f : F) 
where F : FnMut(&'a [u8]) {
    unsafe extern "C" fn process_slice<'a, F : FnMut(&'a [u8])>(c_slice: clib_dep::SliceU8, ctx: clib_dep::CallbackContext) {
        let f = ctx as *mut F;
        (*f)(cdep_slice_to_slice(c_slice))
    } 

    let mut f = f;    
    let ctx ={
        // introduce mut reference
        let ctx = &mut f;
        ctx as *mut F as clib_dep::CallbackContext 
    };
    // mut reference goes out of scope and we can legaly recreate it in the callback
    unsafe {
        clib_dep::for_each_contiguous_non_zero_pixels_horizontal_line_do(take_cdep_image_ref(img), Some(process_slice::<'a, F>), ctx);
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_count_pixels() {
        let image_buf = vec![
            1, 1, 1, 1, 0,
            0, 1, 1, 0, 1,
            1, 1, 1, 0, 0
        ];
        let image = ContiguousImageU8View {
            data_slice : &image_buf,
            width : 5
        };
        assert_eq!(count_non_zero_pixels(image), 10)
    }

    #[test]
    fn test_callback_bindings() {
        let image_buf = vec![
            1, 1, 1, 1, 0,
            0, 1, 1, 0, 1,
            1, 1, 1, 0, 0
        ];
        let image = ContiguousImageU8View {
            data_slice : &image_buf,
            width : 5
        };

        let mut segment_sizes = Vec::new();
        for_each_contiguous_non_zero_pixels_horizontal_line_do(image, |segment| segment_sizes.push(segment.len()) );
        assert_eq!(&segment_sizes, &[4, 2, 1, 3]);

        let mut segments = Vec::new();
        for_each_contiguous_non_zero_pixels_horizontal_line_do(image, |segment| segments.push(segment) );
        assert_eq!(segments.len(), 4);
        
        // -- following should introduce CE: --
        
        // drop(image_buf);
        // for s in segments {
        //     println!("{s:?}");
        // }
    }


   

}


