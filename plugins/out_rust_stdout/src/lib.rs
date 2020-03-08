extern crate rmp_serde as rmps;
extern crate serde;

use std::collections::HashMap;
use std::ffi::c_void;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ptr;

use serde::{Deserialize, Serialize};

use rust_binding;

// static functions in C
// https://www.tutorialspoint.com/static-functions-in-c
// https://stackoverflow.com/questions/31701655/can-a-rust-constant-static-be-exposed-to-c

// Rust's convention is to use CAP_SNAKE for statics. However,
// https://github.com/fluent/fluent-bit/blob/master/src/flb_plugin.c#L128
// https://github.com/fluent/fluent-bit/blob/master/conf/plugins.conf
// https://github.com/fluent/fluent-bit/blob/master/src/flb_plugin.c#L273
// https://github.com/fluent/fluent-bit/blob/master/src/fluent-bit.c#L865
// requires the struct's exported name to follow certain naming convention.
// https://users.rust-lang.org/t/option-is-ffi-safe-or-not/29820/9
#[export_name = "out_rust_stdout_plugin"]
pub static mut OUT_STDOUT2_PLUGIN: rust_binding::flb_output_plugin =
    rust_binding::flb_output_plugin {
        // seems like plugin type is determined by path name here: https://github.com/fluent/fluent-bit/blob/1.3/src/flb_plugin.c#L223
        type_: 1,
        // https://doc.rust-lang.org/std/mem/union.MaybeUninit.html
        // https://doc.rust-lang.org/nomicon/unchecked-uninit.html
        // We will leave the memory allocation and initialization to
        // fluentbit core
        proxy: ptr::null_mut(),
        flags: 0,
        // http://jakegoulding.com/rust-ffi-omnibus/string_return/
        // https://stackoverflow.com/questions/53611161/how-do-i-expose-a-compile-time-generated-static-c-string-through-ffi
        name: "rust_stdout\0".as_ptr() as *const c_char,
        description: "experiement\0".as_ptr() as *const c_char,
        // http://jakegoulding.com/rust-ffi-omnibus/
        // https://medium.com/jim-fleming/complex-types-with-rust-s-ffi-315d14619479
        // https://s3.amazonaws.com/temp.michaelfbryan.com/arrays/index.html
        // https://github.com/fluent/fluent-bit/blob/master/src/flb_output.c#L628
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
                // following fields uninitialized
                // Initialie empty struct in Rust: https://gist.github.com/ChrisWellsWood/84421854794037e760808d5d97d21421
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
                // https://crates.io/crates/memoffset
                // TODO: need to figure out how to do this because
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
            // EOF
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
        // TODO: figure out whther I need to deal with cb_pre_run
        cb_pre_run: None,
        // From https://github.com/fluent/fluent-bit/blob/e6506b7b5364c77bec186d94e51c4b3b51e6fbac/src/flb_plugin.c#L248
        // seems like it will be allocated so no need to allocate here
        _head: rust_binding::mk_list {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        },
        cb_init: Some(plugin_init),
        cb_flush: Some(plugin_flush),
        cb_exit: Some(plugin_exit),
    };

#[repr(C)]
#[derive(Debug)]
pub struct sss {
    /*
     * configuration properties: incoming properties set by the caller. This
     * list is what the instance received by either a configuration file or
     * through the command line arguments. This list is validated by the
     * plugin.
     */
    pub properties: rust_binding::mk_list,

    /*
     * configuration map: a new API is landing on Fluent Bit v1.4 that allows
     * plugins to specify at registration time the allowed configuration
     * properties and it data types. Config map is an optional API for now
     * and some plugins will take advantage of it. When the API is used, the
     * config map will validate the configuration, set default values
     * and merge the 'properties' (above) into the map.
     */
    pub config_map: *mut rust_binding::mk_list,
    // struct mk_list _head;                /* link to config->inputs       */

    /* Keep a reference to the original context this instance belongs to */
    // struct flb_config *config;
}

#[no_mangle]
extern "C" fn rust_checkit(
    ins: *mut rust_binding::flb_output_instance,
    config: *mut rust_binding::flb_config,
    data: *mut c_void,
    ins_config_map: *mut rust_binding::mk_list,
    p_config_map: *mut rust_binding::mk_list,
    ss: *mut sss,
) {
    unsafe {
        eprintln!("rust_checkit ins.config_map: {:?}", (*ins).config_map);
        eprintln!(
            "rust_checkit (*ins).properties.prev: {:?}",
            (*ins).properties.prev
        );
        eprintln!(
            "rust_checkit (*ins).properties.next: {:?}",
            (*ins).properties.next
        );
        eprintln!("rust_checkit ins_config_map: {:?}", ins_config_map);
        eprintln!("rust_checkit p_config_map: {:?}", p_config_map);
        eprintln!("rust_checkit ss: {:?}", *ss);
    }
}

#[no_mangle]
extern "C" fn plugin_init(
    ins: *mut rust_binding::flb_output_instance,
    config: *mut rust_binding::flb_config,
    data: *mut c_void,
) -> c_int {
    unsafe {
        eprintln!("rust_plugin_init ins.config_map: {:?}", (*ins).config_map);
        // https://medium.com/thinkthenrant/rust-tidbits-mut-mut-let-mut-let-mut-oh-my-ede02aa07eb6
        let mut ctx = mem::zeroed::<rust_binding::flb_rust_stdout>();
        ctx.ins = ins;
        // https://doc.rust-lang.org/std/ffi/enum.c_void.html
        // https://stackoverflow.com/questions/24191249/working-with-c-void-in-an-ffi
        // https://users.rust-lang.org/t/semantics-of-mut--/5514
        let ctx_ptr: *mut c_void = &mut ctx as *mut _ as *mut c_void;
        // https://github.com/rust-lang/rust/issues/61820
        // https://stackoverflow.com/questions/17081131/how-can-a-shared-library-so-call-a-function-that-is-implemented-in-its-loadin
        // https://stackoverflow.com/questions/36692315/what-exactly-does-rdynamic-do-and-when-exactly-is-it-needed
        // https://stackoverflow.com/questions/5555632/can-gcc-not-complain-about-undefined-references
        // this is how fluent-bit compiles its built-in plugins:
        // https://github.com/fluent/fluent-bit/blob/master/plugins/CMakeLists.txt#L110
        // https://github.com/fluent/fluent-bit/blob/master/plugins/out_stdout/CMakeLists.txt
        println!("ins: {:?}", ins);
        println!("ctr_ptr: {:?}", ctx_ptr);
        let ret = rust_binding::flb_config_map_set(
            &mut (*ins).properties,
            (*ins).config_map,
            ctx_ptr,
        );
        if ret == -1 {
            return ret;
        }

        // One potential solution to access #define constant in C through Rust FFI:
        // https://stackoverflow.com/questions/21485655/how-do-i-use-c-preprocessor-macros-with-rusts-ffi
        ctx.out_format = 0;
        let fmt_ptr =
            rust_binding::flb_output_get_property("format".as_ptr() as *const c_char, ins);
        // https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref
        if fmt_ptr.as_ref().is_some() {
            let ret = rust_binding::flb_pack_to_json_format_type(fmt_ptr);
            if ret == -1 {
                // TODO: use fluent-bit's logger? flb_plg_error is a macro defined
                // at https://github.com/fluent/fluent-bit/blob/master/include/fluent-bit/flb_output_plugin.h#L28
                // flb_plg_error(ctx->ins, "invalid json_date_format '%s'. "
                //              "Using 'double' type", tmp);
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
                // TODO: use fluent-bit's logger? flb_plg_error is a macro defined
                // at https://github.com/fluent/fluent-bit/blob/master/include/fluent-bit/flb_output_plugin.h#L28
                // flb_plg_error(ctx->ins, "invalid json_date_format '%s'. "
                // "Using 'double' type", tmp);
                println!("flb_pack_to_json_date_type error");
            } else {
                ctx.json_date_format = ret;
            }
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

// #[derive(Debug, PartialEq, Deserialize, Serialize)]
// struct CPURecord {
//     cpu_p: f32,
//     user_p: f32,
//     system_p: f32,
// }

#[no_mangle]
extern "C" fn plugin_flush(
    data: *const c_void,
    bytes: usize,
    tag: *const c_char,
    tag_len: c_int,
    i_ins: *mut rust_binding::flb_input_instance,
    out_context: *mut c_void,
    config: *mut rust_binding::flb_config,
) {
    // https://www.reddit.com/r/rust/comments/9wk0jy/free_memory_allocated_from_c_through_ffi/
    // https://users.rust-lang.org/t/c-ffi-memory-leak-take-ownership-of-allocated-memory-in-c-c/24337/3
    // https://hacks.mozilla.org/2019/04/crossing-the-rust-ffi-frontier-with-protocol-buffers/

    // https://github.com/fluent/fluent-bit-go/blob/master/output/decoder.go#L57
    // https://github.com/aws/amazon-kinesis-firehose-for-fluent-bit/blob/6ca31170fc03aa8081255de927a87156d787ce14/fluent-bit-firehose.go#L105
    // https://github.com/fluent/fluent-bit-go/blob/master/output/decoder.go#L70
    // just unpack the data, which is in msgpack format,
    // generated from https://docs.fluentbit.io/manual/input/cpu
    // and print.

    // https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
    // https://stackoverflow.com/questions/27150652/how-can-i-get-an-array-or-a-slice-from-a-raw-pointer
    let msg_pack_raw_data: &[u8] = unsafe {
        // TODO: verify correct lifetime of the returned variable:
        // https://stackoverflow.com/questions/33305573/why-is-the-lifetime-important-for-slicefrom-raw-parts
        std::slice::from_raw_parts(data as *const u8, bytes)
    };

    // https://docs.rs/rmp-serde/0.14.3/rmp_serde/
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
    // TODO: Do we need to free the data argument just like the
    // C stdout output plugin?
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
