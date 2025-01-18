#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_process_eth_trigger_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let len0 = arg1;
    let result1 = T::process_eth_trigger(_rt::Vec::from_raw_parts(arg0.cast(), len0, len0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(e) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
            let vec3 = (e).into_boxed_slice();
            let ptr3 = vec3.as_ptr().cast::<u8>();
            let len3 = vec3.len();
            ::core::mem::forget(vec3);
            *ptr2.add(8).cast::<usize>() = len3;
            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
        }
        Err(e) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
            let vec4 = (e.into_bytes()).into_boxed_slice();
            let ptr4 = vec4.as_ptr().cast::<u8>();
            let len4 = vec4.len();
            ::core::mem::forget(vec4);
            *ptr2.add(8).cast::<usize>() = len4;
            *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_process_eth_trigger<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = *arg0.add(4).cast::<*mut u8>();
            let l2 = *arg0.add(8).cast::<usize>();
            let base3 = l1;
            let len3 = l2;
            _rt::cabi_dealloc(base3, len3 * 1, 1);
        }
        _ => {
            let l4 = *arg0.add(4).cast::<*mut u8>();
            let l5 = *arg0.add(8).cast::<usize>();
            _rt::cabi_dealloc(l4, l5, 1);
        }
    }
}
pub trait Guest {
    fn process_eth_trigger(input: _rt::Vec<u8>) -> Result<_rt::Vec<u8>, _rt::String>;
}
#[doc(hidden)]
macro_rules! __export_world_eth_trigger_world_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "process-eth-trigger"] unsafe extern "C" fn
        export_process_eth_trigger(arg0 : * mut u8, arg1 : usize,) -> * mut u8 {
        $($path_to_types)*:: _export_process_eth_trigger_cabi::<$ty > (arg0, arg1) }
        #[export_name = "cabi_post_process-eth-trigger"] unsafe extern "C" fn
        _post_return_process_eth_trigger(arg0 : * mut u8,) { $($path_to_types)*::
        __post_return_process_eth_trigger::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_eth_trigger_world_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 12]);
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_eth_trigger_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_eth_trigger_world_cabi!($ty
        with_types_in $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_eth_trigger_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:lay3r:avs@0.3.0:eth-trigger-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 224] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07Y\x01A\x02\x01A\x04\x01\
p}\x01j\x01\0\x01s\x01@\x01\x05input\0\0\x01\x04\0\x13process-eth-trigger\x01\x02\
\x04\0!lay3r:avs/eth-trigger-world@0.3.0\x04\0\x0b\x17\x01\0\x11eth-trigger-worl\
d\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10\
wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
