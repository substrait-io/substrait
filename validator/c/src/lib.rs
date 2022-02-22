// Functions dereferencing raw pointers are kind of par for the course in a C
// interface.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::cell::RefCell;

thread_local! {
    /// Most recent error message, stored in thread-local storage for
    /// thread-safety.
    pub static LAST_ERROR: RefCell<std::ffi::CString> = RefCell::new(std::ffi::CString::default());
}

/// Pushes an error message.
fn set_last_error<S: AsRef<str>>(s: S) {
    LAST_ERROR.with(|f| {
        *f.borrow_mut() = std::ffi::CString::new(s.as_ref()).unwrap_or_default();
    });
}

/// Returns the most recent error message. Note that the returned pointer is
/// only valid until the next call that the current thread makes to this
/// library.
#[no_mangle]
pub extern "C" fn substrait_validator_get_last_error() -> *const libc::c_char {
    LAST_ERROR.with(|f| {
        let reference = f.borrow();
        reference.as_bytes_with_nul().as_ptr() as *const libc::c_char
    })
}

/// Parse result handle.
pub struct Handle {
    pub root: substrait_validator::tree::Node,
}

/// Parses the given byte buffer as a substrait.Plan message. Returns a handle
/// to the parse result. This handle must be freed using
/// substrait_validator_free() when it is no longer needed. Fails and returns
/// nullptr only if the incoming buffer is nullptr; any other failure to parse
/// or validate the buffer is embedded in the handle.
#[no_mangle]
pub extern "C" fn substrait_validator_parse(data: *const u8, size: u64) -> *mut Handle {
    // Catch null pointers.
    if data.is_null() {
        set_last_error("received null input buffer");
        return std::ptr::null_mut();
    }

    // Convert the incoming buffer information into a slice.
    let data = unsafe { std::slice::from_raw_parts(data, size.try_into().unwrap()) };

    // Perform the actual parsing.
    let root = substrait_validator::parse(data);

    // Create a box to store the return value handle on the stack.
    let handle = Box::new(Handle { root });

    // Convert the box to its raw pointer and relinquish ownership.
    Box::into_raw(handle)
}

/// Frees memory associated with a parse result handle. No-op if given a
/// nullptr.
#[no_mangle]
pub extern "C" fn substrait_validator_free(handle: *mut Handle) {
    // Ignore null pointers.
    if handle.is_null() {
        return;
    }

    // Recover the box that we created the handle with and drop it.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

/// Returns whether the given parse result handle refers to a valid (positive
/// return value), invalid (negative return value), or possibly valid plan
/// (0 return value).
#[no_mangle]
pub extern "C" fn substrait_validator_check(handle: *const Handle) -> i32 {
    // Dereference the handle.
    let handle = unsafe { handle.as_ref() };
    if handle.is_none() {
        return -1;
    }
    let root = &handle.as_ref().unwrap().root;

    // Perform the check.
    match substrait_validator::check(root) {
        substrait_validator::Validity::Valid => 1,
        substrait_validator::Validity::MaybeValid => 0,
        substrait_validator::Validity::Invalid => -1,
    }
}

/// The guts for the export functions.
fn export(
    format: substrait_validator::export::Format,
    handle: *const Handle,
    size: *mut u64,
) -> *mut u8 {
    // Dereference the handle.
    let handle = unsafe { handle.as_ref() };
    if handle.is_none() {
        set_last_error("received null handle");
        return std::ptr::null_mut();
    }
    let root = &handle.as_ref().unwrap().root;

    // Create a byte vector as output. The first 8 bytes are reserved: we'll
    // store the length of the vector in there, and advance the pointer beyond
    // this length before passing the data to the user.
    let mut data: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];

    // Perform the actual export function.
    if let Err(e) = substrait_validator::export(&mut data, format, root) {
        set_last_error(e.to_string());
        return std::ptr::null_mut();
    }

    // Append a null character, to prevent pain and misery if the user treats
    // the buffer as a null-terminated string.
    data.push(0);

    // Make sure vec.len() == vec.capacity() so we don't have to store both.
    data.shrink_to_fit();

    // Save the length (and capacity) of the vector to the start of said
    // vector. We'll recover it from there in
    // substrait_validator_free_exported(), so we don't have to rely on the
    // user to save this data.
    let len: u64 = data.len().try_into().unwrap();
    data[..8].clone_from_slice(&len.to_ne_bytes());

    // Also pass the length to the user, if they wanted to know about it.
    if let Some(size) = unsafe { size.as_mut() } {
        // Note that the true size is smaller than the vector because of the
        // 8-byte length prefix and the 1-byte null-termination character.
        *size = len - 9;
    }

    // Get the pointer to the vector, and relinquish ownership.
    let ptr = data.as_mut_ptr();
    std::mem::forget(data);

    // Advance the pointer beyond the bytes that we're using to store the size
    // of the vector.
    unsafe { ptr.add(8) }
}

/// Converts the given parse result to a multiline, null-terminated string,
/// where each line represents a diagnostic message. If size is non-null, the
/// length of the string (excluding null-termination byte) will be written to
/// it. The function will return nullptr upon failure, in which case
/// substrait_validator_get_last_error() can be used to retrieve an error
/// message. If the function succeeds, the returned pointer must eventually be
/// freed using substrait_validator_free_exported() in order to not leak
/// memory.
#[no_mangle]
pub extern "C" fn substrait_validator_export_diagnostics(
    handle: *const Handle,
    size: *mut u64,
) -> *mut u8 {
    export(
        substrait_validator::export::Format::Diagnostics,
        handle,
        size,
    )
}

/// Same as substrait_validator_export_diagnostics(), but instead returns a
/// buffer filled with a HTML-based human-readable description of the parsed
/// plan.
#[no_mangle]
pub extern "C" fn substrait_validator_export_html(
    handle: *const Handle,
    size: *mut u64,
) -> *mut u8 {
    export(substrait_validator::export::Format::Html, handle, size)
}

/// Same as substrait_validator_export_diagnostics(), but instead returns a
/// substrait.validator.Node message in its binary serialization format. The
/// buffer is null-terminated, but note that protobuf serialization is a binary
/// format, so you'll need to use the size argument to get an accurate size.
#[no_mangle]
pub extern "C" fn substrait_validator_export_proto(
    handle: *const Handle,
    size: *mut u64,
) -> *mut u8 {
    export(substrait_validator::export::Format::Proto, handle, size)
}

/// Frees memory associated with an exported buffer. No-op if given a nullptr.
#[no_mangle]
pub extern "C" fn substrait_validator_free_exported(data: *mut u8) {
    // Don't do anything if the user passed nullptr.
    if data.is_null() {
        return;
    }

    // Point the pointer to the start of the allocated region.
    let data = unsafe { data.sub(8) };

    // Recover the length of the vector.
    let len = u64::from_ne_bytes(
        unsafe { std::slice::from_raw_parts(data, 8) }
            .try_into()
            .unwrap(),
    );
    let len = usize::try_from(len).unwrap();

    // Recover the vector and drop it.
    unsafe {
        drop(Vec::from_raw_parts(data, len, len));
    }
}
