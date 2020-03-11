extern crate rmp_serde as rmps;
extern crate serde;
extern crate rand;
extern crate futures;

use {
    std::{
        collections::HashMap,
        error,
        ffi::c_void,
        fmt,
        future::Future,
        mem,
        os::raw::{c_char, c_int},
        ptr,
        sync::Arc,
        task::Context,
        time,
    },
};

use {
    async_std::task,
    futures::task::{ArcWake, waker_ref},
    rand::Rng,
    serde::{Deserialize, Serialize},
};

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
                // TODO: need to figure out how to do this
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

#[no_mangle]
extern "C" fn plugin_init(
    ins: *mut rust_binding::flb_output_instance,
    config: *mut rust_binding::flb_config,
    data: *mut c_void,
) -> c_int {
    unsafe {
        eprintln!("rust_plugin_init ins.config_map: {:?}", (*ins).config_map);
        // TODO: 
        // https://stackoverflow.com/questions/28278213/how-to-lend-a-rust-object-to-c-code-for-an-arbitrary-lifetime
        let mut ctx = Box::new(mem::zeroed::<rust_binding::flb_rust_stdout>());
        *ctx.ins = ins;
        // https://doc.rust-lang.org/std/ffi/enum.c_void.html
        // https://stackoverflow.com/questions/24191249/working-with-c-void-in-an-ffi
        // https://users.rust-lang.org/t/semantics-of-mut--/5514
        let ctx_ptr: *mut c_void = &mut *ctx as *mut _ as *mut c_void;
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

async fn delay_rand_u8() -> u8 {
    eprintln!("delay_rand_u8 called, before async sleep");
    // https://blog.hwc.io/posts/rust-futures-threadsleep-and-blocking-calls-inside-async-fn/
    task::sleep(time::Duration::from_secs(10)).await;
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen();
    eprintln!("delay_rand_u8 called, after async sleep {}", n);
    n
}

// A future that can reschedule itself to be polled by an `Executor`
// (in fleunt-bit's case, it's the event loop in the fluent-bit core).
struct NoOp;

impl ArcWake for NoOp {
    // Wakers are responsible for scheduling a task to be polled again
    // once wake is called (). However, in fluen-bit's case, which uses libco,
    // I believe there's no mechanism to notify the core event loop that some
    // async operation is ready to continue, so we just do nothing here and
    // rely on the core event loop to reschedule us.
    fn wake_by_ref(arc_self: &Arc<Self>) {}
}

#[derive(Debug, Clone)]
struct CCallNonZeroError {
    errorCode: c_int,
}

impl fmt::Display for CCallNonZeroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "call to C returns non-zero code: {}", self.errorCode);
    }
}

// This is important for other errors to wrap this one.
impl error::Error for CCallNonZeroError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[no_mangle]
extern "C" fn event_handler(
    data: *const c_void,
) -> c_int {
    // There are a few examples on whether we need to free the incoming data
    // and whether it should be a *const of *mut:
    // mqtt_conn_event in mqtt_conn.c
    // tcp_conn_event in tcp_conn.c
    // syslog_conn_event in syslog_conn.c

    // This is pretty much the same as how FLB_ENGINE_EV_THREAD is handled
    // in flb_engine.c
    // cast incoming data as mk_event, then cast the data field as a pointer
    let event: &rust_binding::mk_event = unsafe { & *(data as *mut rust_binding::mk_event) };
    let th: *mut rust_binding::flb_thread = unsafe { &mut *(event.data as *mut rust_binding::flb_thread) };
    rust_binding::flb_thread_resume_non_inline(th);
    0
}

// https://rust-lang.github.io/async-book/02_execution/04_executor.html
// https://boats.gitlab.io/blog/post/wakers-i/
pub fn ExecuteFuture<T>(todo: &mut Future<T>, config: *mut rust_binding::flb_config) -> Result<T, CCallNonZeroError> {
    // https://www.reddit.com/r/rust/comments/cfvmj6/is_a_contextwaker_really_required_for_polling_a/
    let task = NoOp;
    let waker = waker_ref(&task);
    let ctx = &mut Context::from_waker(&*waker);

    let event = rust_binding::mk_event {
        // Basically follow MK_EVENT_INIT in mk_event.h
        fd: -1,
        type_: 4, // MK_EVENT_CUSTOM
        mask: 0, // MK_EVENT_EMPTY
        status: 1, // MK_EVENT_NONE
        data: rust_binding::flb_get_pthread() as *mut c_void,
        handler: Some(event_handler),
        _head: rust_binding::mk_list {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        },
    };

    loop {
        match todo.poll(ctx) {
            Poll::Ready(todoOutcome) => break Ok(todoOutcome),
            Poll::Pending => {
                // register a callback by mk_event_add()
                
                // extern "C" {
                //     pub fn mk_event_add(
                //         loop_: *mut mk_event_loop,
                //         fd: ::std::os::raw::c_int,
                //         type_: ::std::os::raw::c_int,
                //         mask: u32,
                //         data: *mut ::std::os::raw::c_void,
                //     ) -> ::std::os::raw::c_int;
                // }
                rust_binding::mk_event_add(
                    config.evl, // event loop
                    -1, // we don't care about fd since we are not using socket here
                    4, // FLB_ENGINE_EV_CUSTOM
                    4, // MK_EVENT_WRITE. TODO: figure out the significance of this value
                    // TODO: [MemoryManagement] do we need to free the following struct (and its fields) or fluent-bit C code does it?
                    // https://stackoverflow.com/questions/38289355/drop-a-rust-void-pointer-stored-in-an-ffi
                    // https://stackoverflow.com/questions/50107792/what-is-the-better-way-to-wrap-a-ffi-struct-that-owns-or-borrows-data
                    // [2nd solution?] https://stackoverflow.com/questions/28278213/how-to-lend-a-rust-object-to-c-code-for-an-arbitrary-lifetime
                    // Might also need to call ::std::mem::forget(obj) in case C will free this for us?
                    &mut event as *mut _ as *mut c_void,
                )
                
                unsafe {
                    rust_binding::flb_thread_yield_non_inline(rust_binding::flb_get_pthread(), 0); // FLB_FALSE == 0
                    
                    let mask = event.mask; // Save events mask since mk_event_del() will reset it
                    ret = rust_binding::mk_event_del(config.evl, &mut event);
                    if ret == -1 {
                        break Err(CCallNonZeroError{ret});
                    }

                    if (event.mask & 4) // MK_EVENT_WRITE {
                        // same as MK_EVENT_NEW
                        event.mask = 0; // MK_EVENT_EMPTY
                        event.status = 1; // MK_EVENT_NONE
                    } else {
                        break Err(CCallNonZeroError{ret}) 
                    }
            },
        }
    }
}

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

    let fut = delay_rand_u8();
    let result = ExecuteFuture(&mut fut, config);
    match result {
        Ok(v) => eprintln!("delay random number: {:?}", v),
        Err(e) => eprintln!("ExecuteFuture error: {:?}", e),
    }
    unsafe {
        rust_binding::flb_output_return_non_inline(1);
    }
}

#[no_mangle]
extern "C" fn plugin_exit(data: *mut c_void, config: *mut rust_binding::flb_config) -> c_int {
    // TODO: [MemoryManagement] Do we need to free the data argument just like the
    // C stdout output plugin?
    // https://stackoverflow.com/questions/38289355/drop-a-rust-void-pointer-stored-in-an-ffi
    // https://stackoverflow.com/questions/50107792/what-is-the-better-way-to-wrap-a-ffi-struct-that-owns-or-borrows-data
    // [2nd solution?] https://stackoverflow.com/questions/28278213/how-to-lend-a-rust-object-to-c-code-for-an-arbitrary-lifetime
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}