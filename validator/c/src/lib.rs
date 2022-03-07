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

/// Parser/validator configuration handle.
pub struct ConfigHandle {
    pub config: substrait_validator_core::context::Config,
}

/// Creates a parser/validator configuration structure.
#[no_mangle]
pub extern "C" fn substrait_validator_config_new() -> *mut ConfigHandle {
    // Create a box to store the return value handle on the stack.
    let handle = Box::new(ConfigHandle {
        config: substrait_validator_core::context::Config::new(),
    });

    // Convert the box to its raw pointer and relinquish ownership.
    Box::into_raw(handle)
}

/// Instructs the validator to ignore protobuf fields that it doesn't know
/// about yet (i.e., that have been added to the Substrait protobuf
/// descriptions, but haven't yet been implemented in the validator) if the
/// fields are set to their default value. If this option isn't set, or if an
/// unknown field is not set to its default value, a warning is emitted.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_ignore_unknown_fields_set_to_default(
    config: *mut ConfigHandle,
) -> bool {
    // Unpack configuration handle, catching null pointers.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }
    let config = unsafe { &mut (*config).config };

    // Update configuration and return success.
    config.ignore_unknown_fields_set_to_default();
    true
}

/// Whitelists a protobuf message type for use in advanced extensions. If an
/// advanced extension is encountered that isn't whitelisted, a warning is
/// emitted.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_whitelist_any_url(
    config: *mut ConfigHandle,
    url: *const libc::c_char,
) -> bool {
    // Unpack configuration handle, catching null pointers.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }
    let config = unsafe { &mut (*config).config };

    // Unpack URL, catching null pointers.
    if url.is_null() {
        set_last_error("received null url");
        return false;
    }
    let url = unsafe { std::ffi::CStr::from_ptr(url) };
    let url = match url.to_str() {
        Ok(u) => u,
        Err(e) => {
            set_last_error(format!("received invalid URL: {}", e));
            return false;
        }
    };

    // Update configuration and return success.
    config.whitelist_any_url(url);
    true
}

/// Converts a positive/zero/negative integer into Info/Warning/Error
/// respectively.
fn int_to_level(x: i32) -> substrait_validator_core::diagnostic::Level {
    use substrait_validator_core::diagnostic::Level;
    match x.cmp(&0) {
        std::cmp::Ordering::Greater => Level::Info,
        std::cmp::Ordering::Equal => Level::Warning,
        std::cmp::Ordering::Less => Level::Error,
    }
}

/// Sets a minimum and/or maximum error level for the given class of diagnostic
/// messages. Any previous settings for this class are overridden. The levels
/// are encoded as integers, where any positive value means info, zero means
/// warning, and negative means error.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_override_diagnostic_level(
    config: *mut ConfigHandle,
    class: u32,
    minimum: i32,
    maximum: i32,
) -> bool {
    // Unpack configuration handle, catching null pointers.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }
    let config = unsafe { &mut (*config).config };

    // Parse the diagnostic class/code.
    let class = match substrait_validator_core::diagnostic::Classification::from_code(class) {
        Some(c) => c,
        None => {
            set_last_error(format!("unknown diagnostic class {}", class));
            return false;
        }
    };

    // Parse the minimum and maximum levels.
    let minimum = int_to_level(minimum);
    let maximum = int_to_level(maximum);

    // Update configuration and return success.
    config.override_diagnostic_level(class, minimum, maximum);
    true
}

/// Overrides the resolution behavior for YAML URIs matching the given
/// pattern. The pattern may include * and ? wildcards for glob-like matching
/// (see https://docs.rs/glob/latest/glob/struct.Pattern.html for the complete
/// syntax). If resolve_as is null, the YAML file will not be resolved;
/// otherwise, it will be resolved as if the URI in the plan had been that
/// string.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_override_yaml_uri(
    config: *mut ConfigHandle,
    pattern: *const libc::c_char,
    resolve_as: *const libc::c_char,
) -> bool {
    // Unpack configuration handle, catching null pointers.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }
    let config = unsafe { &mut (*config).config };

    // Unpack and parse pattern, catching null pointers.
    if pattern.is_null() {
        set_last_error("received null url");
        return false;
    }
    let pattern = unsafe { std::ffi::CStr::from_ptr(pattern) };
    let pattern = match pattern.to_str() {
        Ok(p) => p,
        Err(e) => {
            set_last_error(format!("received invalid pattern: {}", e));
            return false;
        }
    };
    let pattern = match substrait_validator_core::context::glob::Pattern::new(pattern) {
        Ok(p) => p,
        Err(e) => {
            set_last_error(format!("received invalid pattern: {}", e));
            return false;
        }
    };

    // Unpack and parse resolve_as.
    let resolve_as = if resolve_as.is_null() {
        None
    } else {
        let resolve_as = unsafe { std::ffi::CStr::from_ptr(resolve_as) };
        Some(match resolve_as.to_str() {
            Ok(p) => p,
            Err(e) => {
                set_last_error(format!("received invalid replacement URI: {}", e));
                return false;
            }
        })
    };

    // Update configuration and return success.
    config.override_yaml_uri(pattern, resolve_as);
    true
}

/// Callback function for deleting a buffer allocated by the user application.
pub type Deleter =
    Option<unsafe extern "C" fn(user: *mut libc::c_void, buf: *const u8, size: usize)>;

/// (YAML) URI resolution callback function.
///
/// The first argument (uri) is set to a null-terminated UTF-8 string
/// representing the URI that is to be resolved. If resolution succeeds, the
/// function must return the binary result buffer via buf and size and return
/// true. If it fails, it should instead write a UTF-8 error message to this
/// buffer (but it may also set buf to nullptr or leave it unchanged) and
/// return false.
///
/// The buffer must remain valid only until the validator library returns
/// control to the application. Thus, the application may keep track of the
/// current buffer via thread-local storage or a global. It may also assign a
/// deleter function to the deleter parameter, which will be called by the
/// validator library when it is done with the buffer. deleter_user may be
/// used to pass additional contextual information to the deleter; it is not
/// used by the validator library for any purpose other than calling the
/// deleter function.
///
/// All output parameters will be set to zero by the validator library before
/// the callback is called.
pub type Resolver = Option<
    unsafe extern "C" fn(
        uri: *const libc::c_char,
        buf: *mut *const u8,
        size: *mut usize,
        deleter: *mut Deleter,
        deleter_user: *mut *mut libc::c_void,
    ) -> bool,
>;

/// Wraps a buffer returned by Resolver.
struct ApplicationBuffer {
    pub buf: *const u8,
    pub size: usize,
    pub deleter: Deleter,
    pub deleter_user: *mut libc::c_void,
}

impl Default for ApplicationBuffer {
    fn default() -> Self {
        Self {
            buf: std::ptr::null(),
            size: 0,
            deleter: None,
            deleter_user: std::ptr::null_mut(),
        }
    }
}

impl Drop for ApplicationBuffer {
    fn drop(&mut self) {
        if let Some(deleter) = self.deleter {
            unsafe { deleter(self.deleter_user, self.buf, self.size) }
        }
    }
}

impl AsRef<[u8]> for ApplicationBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.buf, self.size) }
    }
}

/// Rust representation of an error returned by the Resolver callback function.
#[derive(Debug, thiserror::Error)]
struct ApplicationError {
    msg: String,
}

impl ApplicationError {
    fn new<S: Into<String>>(msg: S) -> Self {
        ApplicationError { msg: msg.into() }
    }
}

impl From<ApplicationBuffer> for ApplicationError {
    fn from(buf: ApplicationBuffer) -> Self {
        ApplicationError {
            msg: match std::str::from_utf8(buf.as_ref()) {
                Ok(e) => e.to_string(),
                Err(e) => format!("unknown error (failed to decode error message: {})", e),
            },
        }
    }
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

/// Registers a YAML URI resolution function with this configuration. If
/// the given function fails, any previously registered function will be
/// used as a fallback.
///
/// See the documentation for the substrait_validator_resolver typedef for
/// more information about the semantics of the callback function.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_yaml_uri_resolver(
    config: *mut ConfigHandle,
    resolver: Resolver,
) -> bool {
    // Unpack configuration handle, catching null pointers.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }
    let config = unsafe { &mut (*config).config };

    // Unpack resolution function.
    let resolver = match resolver {
        Some(r) => r,
        None => {
            set_last_error("received null resolution function pointer");
            return false;
        }
    };

    // Update configuration and return success.
    config.add_yaml_uri_resolver(move |uri| {
        let uri = match std::ffi::CString::new(uri) {
            Ok(u) => u,
            Err(_) => {
                return Err(ApplicationError::new(
                    "cannot resolve URI with embedded nul characters",
                ))
            }
        };
        let mut buffer = ApplicationBuffer::default();
        let result = unsafe {
            resolver(
                uri.as_ptr(),
                &mut buffer.buf,
                &mut buffer.size,
                &mut buffer.deleter,
                &mut buffer.deleter_user,
            )
        };
        if result {
            if buffer.buf.is_null() {
                Err(ApplicationError::new(
                    "URI resolver callback returned success but also a null buffer",
                ))
            } else {
                Ok(buffer)
            }
        } else if buffer.buf.is_null() {
            Err(ApplicationError::new("URI resolver callback failed"))
        } else {
            Err(ApplicationError::from(buffer))
        }
    });
    true
}

/// Frees memory associated with a configuration handle. No-op if given a
/// nullptr.
#[no_mangle]
pub extern "C" fn substrait_validator_config_free(handle: *mut ConfigHandle) {
    // Ignore null pointers.
    if handle.is_null() {
        return;
    }

    // Recover the box that we created the handle with and drop it.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

/// Parse/validation result handle.
pub struct ResultHandle {
    pub root: substrait_validator_core::tree::Node,
}

/// Parses the given byte buffer as a substrait.Plan message, using the given
/// configuration. If a null pointer is passed for the configuration, the
/// default configuration is used.
///
/// Returns a handle to the parse result. This handle must be freed using
/// substrait_validator_free() when it is no longer needed. Fails and returns
/// nullptr only if the incoming buffer is nullptr; any other failure to parse
/// or validate the buffer is embedded in the handle.
#[no_mangle]
pub extern "C" fn substrait_validator_parse(
    data: *const u8,
    size: u64,
    config: *const ConfigHandle,
) -> *mut ResultHandle {
    // Catch null pointers.
    if data.is_null() {
        set_last_error("received null input buffer");
        return std::ptr::null_mut();
    }

    // Convert the incoming buffer information into a slice.
    let data = unsafe { std::slice::from_raw_parts(data, size.try_into().unwrap()) };

    // Perform the actual parsing.
    let root = if config.is_null() {
        substrait_validator_core::parse(data, &substrait_validator_core::context::Config::default())
    } else {
        substrait_validator_core::parse(data, unsafe { &(*config).config })
    };

    // Create a box to store the return value handle on the stack.
    let handle = Box::new(ResultHandle { root });

    // Convert the box to its raw pointer and relinquish ownership.
    Box::into_raw(handle)
}

/// Frees memory associated with a parse result handle. No-op if given a
/// nullptr.
#[no_mangle]
pub extern "C" fn substrait_validator_free(handle: *mut ResultHandle) {
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
pub extern "C" fn substrait_validator_check(handle: *const ResultHandle) -> i32 {
    // Dereference the handle.
    let handle = unsafe { handle.as_ref() };
    if handle.is_none() {
        return -1;
    }
    let root = &handle.as_ref().unwrap().root;

    // Perform the check.
    match substrait_validator_core::check(root) {
        substrait_validator_core::Validity::Valid => 1,
        substrait_validator_core::Validity::MaybeValid => 0,
        substrait_validator_core::Validity::Invalid => -1,
    }
}

/// The guts for the export functions.
fn export(
    format: substrait_validator_core::export::Format,
    handle: *const ResultHandle,
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
    if let Err(e) = substrait_validator_core::export(&mut data, format, root) {
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
    handle: *const ResultHandle,
    size: *mut u64,
) -> *mut u8 {
    export(
        substrait_validator_core::export::Format::Diagnostics,
        handle,
        size,
    )
}

/// Same as substrait_validator_export_diagnostics(), but instead returns a
/// buffer filled with a HTML-based human-readable description of the parsed
/// plan.
#[no_mangle]
pub extern "C" fn substrait_validator_export_html(
    handle: *const ResultHandle,
    size: *mut u64,
) -> *mut u8 {
    export(substrait_validator_core::export::Format::Html, handle, size)
}

/// Same as substrait_validator_export_diagnostics(), but instead returns a
/// substrait.validator.Node message in its binary serialization format. The
/// buffer is null-terminated, but note that protobuf serialization is a binary
/// format, so you'll need to use the size argument to get an accurate size.
#[no_mangle]
pub extern "C" fn substrait_validator_export_proto(
    handle: *const ResultHandle,
    size: *mut u64,
) -> *mut u8 {
    export(
        substrait_validator_core::export::Format::Proto,
        handle,
        size,
    )
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
