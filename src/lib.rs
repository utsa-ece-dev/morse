use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

mod morse;

/// Encodes the input C string into its Morse code representation and writes the result into the provided output buffer.
///
/// # Parameters
/// - `input`: A pointer to a null-terminated C string containing the text to encode.
/// - `morse_output`: A pointer to a pre-allocated buffer where the Morse code will be written.
/// - `morse_output_size`: The size (in bytes) of the output buffer, including space for the null terminator.
///
/// # Safety
/// This function is unsafe because it dereferences raw pointers. The caller must ensure that:
/// - `input` points to a valid, null-terminated C string.
/// - `morse_output` points to a valid memory region of at least `morse_output_size` bytes.
/// - `morse_output_size` is greater than 0.
///
/// If any of these conditions are not met, the behavior is undefined. If the Morse code output exceeds the provided buffer size,
/// it will be truncated to fit, and the output will always be null-terminated.

#[unsafe(no_mangle)]
pub extern "C" fn encode_morse(
    input: *const c_char,
    morse_output: *mut c_char,
    morse_output_size: c_int,
) {
    // Validate pointers and buffer size.
    if input.is_null() || morse_output.is_null() || morse_output_size <= 0 {
        return;
    }

    // Convert the input C string to a Rust string slice.
    let c_str = unsafe { CStr::from_ptr(input) };
    let text = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return,
    };

    // Convert the text to Morse code.
    let morse_code = morse::encode_morse(text);
    if morse_code.is_empty() {
        return;
    }

    // Convert the Morse code to bytes.
    let bytes = morse_code.as_bytes();

    // Determine how many bytes can be safely copied, leaving room for a null terminator.
    let max_len = morse_output_size as usize;
    let copy_len = if bytes.len() < max_len - 1 {
        bytes.len()
    } else {
        max_len - 1
    };

    // Copy the bytes into the provided output buffer and append a null terminator.
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), morse_output as *mut u8, copy_len);
        *morse_output.add(copy_len) = 0;
    }
}

/// Decodes a Morse code C string into plain text and writes the result into the provided output buffer.
///
/// # Parameters
/// - `input`: A pointer to a null-terminated C string containing the Morse code to decode.
/// - `decoded_output`: A pointer to a pre-allocated buffer where the decoded plain text will be written.
/// - `decoded_output_size`: The size (in bytes) of the output buffer, including space for the null terminator.
///
/// # Safety
/// This function is unsafe because it dereferences raw pointers. The caller must ensure that:
/// - `input` points to a valid, null-terminated C string.
/// - `decoded_output` points to a valid memory region of at least `decoded_output_size` bytes.
/// - `decoded_output_size` is greater than 0.
/// If these conditions are not met, the behavior is undefined. If the decoded text exceeds the provided buffer size,
/// it will be truncated, and the output will always be null-terminated.
#[unsafe(no_mangle)]
pub extern "C" fn decode_morse(
    input: *const c_char,
    decoded_output: *mut c_char,
    decoded_output_size: c_int,
) {
    // Validate pointers and ensure the output buffer is large enough.
    if input.is_null() || decoded_output.is_null() || decoded_output_size <= 0 {
        return;
    }

    // Convert the input C string into a Rust string slice.
    let c_str = unsafe { CStr::from_ptr(input) };
    let text = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return,
    };

    // Decode the Morse code into plain text.
    let decoded_text = morse::decode_morse(text);
    if decoded_text.is_empty() {
        return;
    }

    // Convert the decoded text into bytes.
    let bytes = decoded_text.as_bytes();

    // Determine how many bytes can be safely copied, leaving room for a null terminator.
    let max_len = decoded_output_size as usize;
    let copy_len = if bytes.len() < max_len - 1 {
        bytes.len()
    } else {
        max_len - 1
    };

    // Copy the bytes into the provided output buffer and null-terminate.
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), decoded_output as *mut u8, copy_len);
        *decoded_output.add(copy_len) = 0;
    }
}
