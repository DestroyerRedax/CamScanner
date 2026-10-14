pub mod detector;
pub mod filters;
pub mod ocr;
pub mod editor;
pub mod pdf_exporter;

use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn apply_scanner_filter(
    input_bytes: *const u8,
    input_len: usize,
    filter_type: i32,
    output_len: *mut usize,
) -> *mut u8 {
    if input_bytes.is_null() || output_len.is_null() {
        return std::ptr::null_mut();
    }
    let slice = unsafe { std::slice::from_raw_parts(input_bytes, input_len) };
    if let Ok(img) = image::load_from_memory(slice) {
        let result = match filter_type {
            1 => filters::apply_magic_color(&img),
            2 => filters::apply_bw_threshold(&img, 150),
            3 => filters::apply_grayscale(&img),
            4 => filters::apply_shadow_removal(&img),
            _ => img,
        };
        let mut output = Vec::new();
        let _ = result.write_to(&mut Cursor::new(&mut output), ImageOutputFormat::Jpeg(85));
        unsafe { *output_len = output.len(); }
        let ptr = output.as_mut_ptr();
        std::mem::forget(output);
        return ptr;
    }
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn perform_ocr_on_image(
    input_bytes: *const u8,
    input_len: usize,
) -> *mut c_char {
    if input_bytes.is_null() {
        return std::ptr::null_mut();
    }
    let slice = unsafe { std::slice::from_raw_parts(input_bytes, input_len) };
    let text = ocr::extract_text_from_image(slice);
    std::ffi::CString::new(text).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn apply_watermark_to_image(
    input_bytes: *const u8,
    input_len: usize,
    watermark_text: *const c_char,
    output_len: *mut usize,
) -> *mut u8 {
    if input_bytes.is_null() || watermark_text.is_null() || output_len.is_null() {
        return std::ptr::null_mut();
    }
    let slice = unsafe { std::slice::from_raw_parts(input_bytes, input_len) };
    let text_str = unsafe { CStr::from_ptr(watermark_text).to_string_lossy() };
    if let Ok(img) = image::load_from_memory(slice) {
        let watermarked = editor::apply_watermark(&img, &text_str);
        let mut output = Vec::new();
        let _ = watermarked.write_to(&mut Cursor::new(&mut output), ImageOutputFormat::Jpeg(85));
        unsafe { *output_len = output.len(); }
        let ptr = output.as_mut_ptr();
        std::mem::forget(output);
        return ptr;
    }
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn export_scans_to_pdf(
    output_path: *const c_char,
) -> bool {
    if output_path.is_null() {
        return false;
    }
    let path_str = unsafe { CStr::from_ptr(output_path).to_str().unwrap_or("") };
    let sample_buffers: Vec<Vec<u8>> = vec![];
    pdf_exporter::create_pdf_from_images(&sample_buffers, path_str).is_ok()
}

#[no_mangle]
pub extern "C" fn free_scanner_buffer(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, len, len);
        }
    }
}