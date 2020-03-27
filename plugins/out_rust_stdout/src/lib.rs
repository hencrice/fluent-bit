use {
    std::{
        collections::HashMap,
        error,
        ffi::c_void,
        mem,
        os::raw::{c_char, c_int},
        ptr,
    },
};

use serde::{Deserialize, Serialize};
use rmp_serde as rmps;

use rust_binding;

// Rust's convention is to use CAP_SNAKE for statics. However,
// the fluent-bit codebase expects plugin name to conform to certain
// convention:
// https://github.com/fluent/fluent-bit/blob/baff15640e97ac46d457aab011e9103f2dca53ce/plugins/CMakeLists.txt#L61
// So we export the symbol as a different name below.
#[export_name = "out_rust_stdout_plugin"]
pub static mut OUT_STDOUT2_PLUGIN: rust_binding::flb_output_plugin =
    rust_binding::flb_output_plugin {
        // TODO: Define enum for plugin types
        // https://github.com/fluent/fluent-bit/blob/e6506b7b5364c77bec186d94e51c4b3b51e6fbac/include/fluent-bit/flb_plugin.h#L28
        type_: 1,
        proxy: ptr::null_mut(),
        flags: 0,
        // the following is a constant, properly null-terminated C string
        name: "rust_stdout\0".as_ptr() as *const c_char,
        description: "experiement\0".as_ptr() as *const c_char,
        config_map: [
            rust_binding::flb_config_map {
                type_: 0,
                name: "format\0".as_ptr() as *const c_char,
                def_value: ptr::null(),
                flags: 0,
                set_property: 0,
                offset: 0,
                desc: ptr::null(),
                // https://github.com/fluent/fluent-bit/blob/46c322c0cc8c09908c25f8356ea7bf8b848ff6b2/src/flb_config_map.c#L287
                // looks like we always allocated new memory, so it might be ok to leave the
                // some fields uninitialized or null
                value: rust_binding::flb_config_map_val {
                    val: rust_binding::flb_config_map_val__bindgen_ty_1 {
                        i_num: rust_binding::__BindgenUnionField::new(),
                        boolean: rust_binding::__BindgenUnionField::new(),
                        d_num: rust_binding::__BindgenUnionField::new(),
                        s_num: rust_binding::__BindgenUnionField::new(),
                        str: rust_binding::__BindgenUnionField::new(),
                        list: rust_binding::__BindgenUnionField::new(),
                        bindgen_union_field: 0,
                    },
                    mult: ptr::null_mut(),
                    _head: rust_binding::mk_list {
                        prev: ptr::null_mut(),
                        next: ptr::null_mut(),
                    },
                },
                _head: rust_binding::mk_list {
                    prev: ptr::null_mut(),
                    next: ptr::null_mut(),
                },
            },
            rust_binding::flb_config_map {
                type_: 0,
                name: "json_date_format\0".as_ptr() as *const c_char,
                def_value: ptr::null(),
                flags: 0,
                set_property: 0,
                offset: 0,
                desc: ptr::null(),
                value: rust_binding::flb_config_map_val {
                    val: rust_binding::flb_config_map_val__bindgen_ty_1 {
                        i_num: rust_binding::__BindgenUnionField::new(),
                        boolean: rust_binding::__BindgenUnionField::new(),
                        d_num: rust_binding::__BindgenUnionField::new(),
                        s_num: rust_binding::__BindgenUnionField::new(),
                        str: rust_binding::__BindgenUnionField::new(),
                        list: rust_binding::__BindgenUnionField::new(),
                        bindgen_union_field: 0,
                    },
                    mult: ptr::null_mut(),
                    _head: rust_binding::mk_list {
                        prev: ptr::null_mut(),
                        next: ptr::null_mut(),
                    },
                },
                _head: rust_binding::mk_list {
                    prev: ptr::null_mut(),
                    next: ptr::null_mut(),
                },
            },
            rust_binding::flb_config_map {
                type_: 0,
                name: "json_date_format\0".as_ptr() as *const c_char,
                def_value: "date\0".as_ptr() as *const c_char,
                flags: 0,
                set_property: 1,
                // calculate offset same as https://github.com/fluent/fluent-bit/blob/e6506b7b5364c77bec186d94e51c4b3b51e6fbac/plugins/out_stdout/stdout.c#L171
                // I couldn't find an offsetof implementation in Rust that works with static variable.
                // TODO: need to figure out how to properly calculate this
                offset: 20,
                desc: ptr::null(),
                value: rust_binding::flb_config_map_val {
                    val: rust_binding::flb_config_map_val__bindgen_ty_1 {
                        i_num: rust_binding::__BindgenUnionField::new(),
                        boolean: rust_binding::__BindgenUnionField::new(),
                        d_num: rust_binding::__BindgenUnionField::new(),
                        s_num: rust_binding::__BindgenUnionField::new(),
                        str: rust_binding::__BindgenUnionField::new(),
                        list: rust_binding::__BindgenUnionField::new(),
                        bindgen_union_field: 0,
                    },
                    mult: ptr::null_mut(),
                    _head: rust_binding::mk_list {
                        prev: ptr::null_mut(),
                        next: ptr::null_mut(),
                    },
                },
                _head: rust_binding::mk_list {
                    prev: ptr::null_mut(),
                    next: ptr::null_mut(),
                },
            },
            // This item represents the "EOF" of this config_map list. It consists of 0 for type_ and null for name:
            // https://github.com/fluent/fluent-bit/blob/5b08b2073cb86b34fa2419be55078a45fdf37236/src/flb_config_map.c#L274
            rust_binding::flb_config_map {
                type_: 0,
                name: ptr::null(),
                def_value: ptr::null(),
                flags: 0,
                set_property: 0,
                offset: 0,
                desc: ptr::null(),
                value: rust_binding::flb_config_map_val {
                    val: rust_binding::flb_config_map_val__bindgen_ty_1 {
                        i_num: rust_binding::__BindgenUnionField::new(),
                        boolean: rust_binding::__BindgenUnionField::new(),
                        d_num: rust_binding::__BindgenUnionField::new(),
                        s_num: rust_binding::__BindgenUnionField::new(),
                        str: rust_binding::__BindgenUnionField::new(),
                        list: rust_binding::__BindgenUnionField::new(),
                        bindgen_union_field: 0,
                    },
                    mult: ptr::null_mut(),
                    _head: rust_binding::mk_list {
                        prev: ptr::null_mut(),
                        next: ptr::null_mut(),
                    },
                },
                _head: rust_binding::mk_list {
                    prev: ptr::null_mut(),
                    next: ptr::null_mut(),
                },
            },
        ]
        .as_ptr(),
        host: rust_binding::flb_net_host {
            ipv6: 0,
            address: ptr::null(),
            port: 0,
            name: ptr::null(),
            listen: ptr::null(),
            uri: ptr::null(),
        },
        cb_pre_run: None,
        _head: rust_binding::mk_list {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        },
        cb_init: Some(plugin_init),
        cb_flush: Some(plugin_flush),
        cb_exit: Some(plugin_exit),
    };

#[no_mangle]
extern "C" fn plugin_init(
    ins: *mut rust_binding::flb_output_instance,
    config: *mut rust_binding::flb_config,
    data: *mut c_void,
) -> c_int {
    unsafe {
        // https://stackoverflow.com/questions/28278213/how-to-lend-a-rust-object-to-c-code-for-an-arbitrary-lifetime
        // Allocate memory on the heap by using Box. Otherwise the memory will be allocated on the stack and
        // the ctx_ptr below will point to an invalid chunk of memory right after this function returns.
        let mut ctx = Box::new(mem::zeroed::<rust_binding::flb_rust_stdout>());
        ctx.ins = ins;

        // TODO: define constant in Rust
        ctx.out_format = 0;
        let fmt_ptr =
            rust_binding::flb_output_get_property("format".as_ptr() as *const c_char, ins);
        if fmt_ptr.as_ref().is_some() {
            let ret = rust_binding::flb_pack_to_json_format_type(fmt_ptr);
            if ret == -1 {
                // TODO: If we want to use fluent-bit's logger, need to property expose the macro defined at:
                // at https://github.com/fluent/fluent-bit/blob/master/include/fluent-bit/flb_output_plugin.h#L28
                println!("flb_pack_to_json_format_type error")
            } else {
                ctx.out_format = ret;
            }
        }

        ctx.json_date_format = 0;
        let date_fmt_ptr = rust_binding::flb_output_get_property(
            "json_date_format".as_ptr() as *const c_char,
            ins,
        );
        if date_fmt_ptr.as_ref().is_some() {
            let ret = rust_binding::flb_pack_to_json_date_type(date_fmt_ptr);
            if ret == -1 {
                // TODO: use fluent-bit's logger?
                println!("flb_pack_to_json_date_type error");
            } else {
                ctx.json_date_format = ret;
            }
        }

        // Retrieve a raw pointer out of the heap-allocated variable, ctx, then cast the
        // variable as raw *mut c_void pointer.
        let ctx_ptr: *mut c_void = Box::into_raw(ctx) as *mut c_void;
        let ret = rust_binding::flb_config_map_set(
            &mut (*ins).properties,
            (*ins).config_map,
            ctx_ptr,
        );
        if ret == -1 {
            return ret;
        }

        rust_binding::flb_output_set_context(ins, ctx_ptr);
    }

    0
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Record {
    // https://docs.fluentbit.io/manual/development/ingest_records_manually
    timestamp: u32,
    record: HashMap<String, String>,
}

#[no_mangle]
extern "C" fn plugin_flush<'a>(
    data: *const c_void,
    bytes: usize,
    tag: *const c_char,
    tag_len: c_int,
    i_ins: *mut rust_binding::flb_input_instance,
    out_context: *mut c_void,
    config: *mut rust_binding::flb_config,
) {
    // unpack `data`, which is of length `bytes` and in msgpack format

    // https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
    let msg_pack_raw_data: &'a [u8] = unsafe {
        // Rust code here is not responsible for deallocating the memory section pointed
        // by `data`. `msg_pack_raw_data` is merely a read-only reference.
        // Additional info:
        // https://stackoverflow.com/questions/33305573/why-is-the-lifetime-important-for-slicefrom-raw-parts
        std::slice::from_raw_parts(data as *const u8, bytes)
    };

    let value: Result<Record, rmps::decode::Error> = rmps::from_slice(msg_pack_raw_data);
    match value {
        Ok(v) => {
            println!("ok from msg pack: {:#?}", v);
        }
        Err(e) => println!("err returned from msg pack: {}", e),
    }

    unsafe {
        rust_binding::flb_output_return_non_inline(1);
    }
}

#[no_mangle]
extern "C" fn plugin_exit(data: *mut c_void, config: *mut rust_binding::flb_config) -> c_int {
    // Deallocate the `ctx` allocated in `plugin_init()`.
    
    // Additional info:
    // 2nd solution on https://stackoverflow.com/questions/28278213/how-to-lend-a-rust-object-to-c-code-for-an-arbitrary-lifetime
    if !data.is_null() {
        unsafe {
            Box::from_raw(data as *mut rust_binding::flb_rust_stdout); // Rust auto-drops it at the end of this function
        }
    }
    0
}

// TODO: add real tests
// TODO: refactor some of the unsafe operations into separate utilities for more comprehensive testing
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
