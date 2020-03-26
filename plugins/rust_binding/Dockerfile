# the version here does not matter since we will install the stable toolchain below
FROM rust:latest

RUN apt-get clean && apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    llvm-dev libclang-dev clang valgrind tree

WORKDIR /usr/src/myapp

# https://blog.rust-lang.org/2019/10/15/Rustup-1.20.0.html
# rustup set profile only affect newly installed toolchain, so
# install the stable one
RUN rustup set profile default && rustup update stable && rustup default stable

RUN git clone --branch rustPlugins https://github.com/hencrice/fluent-bit.git

# build and install the fluent-bit executable
RUN mkdir -p /fluent-bit/bin /fluent-bit/etc /fluent-bit/log /tmp/fluent-bit-master/ && \
    cd fluent-bit/build && \
    cmake -DFLB_DEBUG=On \
    -DFLB_TRACE=Off \
    -DFLB_JEMALLOC=On \
    -DFLB_TLS=On \
    -DFLB_OUT_RUST_STDOUT=On \
    -DFLB_OUT_KAFKA=Off \
    -DFLB_HTTP_SERVER=Off \
    -DFLB_IN_SYSTEMD=Off \
    -DFLB_IN_CPU=Off \
    -DFLB_SHARED_LIB=Off \
    -DFLB_STREAM_PROCESSOR:BOOL=Off \
    -DFLB_RECORD_ACCESSOR=Off \
    -DFLB_LUAJIT=Off \
    -DFLB_SIGNV4=Off \
    -DFLB_SQLDB=Off \
    -DFLB_EXAMPLES=Off \
    -DFLB_IN_THERMAL=Off \
    -DFLB_IN_DISK=Off \
    -DFLB_IN_DOCKER=Off \
    -DFLB_IN_EXEC=Off \
    -DFLB_IN_FORWARD=On \
    -DFLB_IN_HEALTH=Off \
    -DFLB_IN_HTTP=Off \
    -DFLB_IN_MEM=Off \
    -DFLB_IN_KMSG=Off \
    -DFLB_IN_LIB=Off \
    -DFLB_IN_RANDOM=Off \
    -DFLB_IN_SERIAL=Off \
    -DFLB_IN_STDIN=Off \
    -DFLB_IN_SYSLOG=Off \
    -DFLB_IN_TAIL=Off \
    -DFLB_IN_TCP=Off \
    -DFLB_IN_MQTT=Off \
    -DFLB_IN_HEAD=Off \
    -DFLB_IN_PROC=Off \
    -DFLB_IN_SYSTEMD=Off \
    -DFLB_IN_DUMMY=Off \
    -DFLB_IN_NETIF=Off \
    -DFLB_IN_WINLOG=Off \
    -DFLB_IN_COLLECTD=Off \
    -DFLB_IN_STATSD=Off \
    -DFLB_IN_STORAGE_BACKLOG=Off \
    -DFLB_IN_EMITTER=Off \
    -DFLB_OUT_AZURE=Off \
    -DFLB_OUT_BIGQUERY=Off \
    -DFLB_OUT_COUNTER=Off \
    -DFLB_OUT_DATADOG=Off \
    -DFLB_OUT_ES=Off \
    -DFLB_OUT_EXIT=Off \
    -DFLB_OUT_FORWARD=Off \
    -DFLB_OUT_GELF=Off \
    -DFLB_FILTER_REWRITE_TAG=Off \
    -DFLB_FILTER_GREP=Off \
    -DFLB_OUT_HTTP=Off \
    -DFLB_OUT_INFLUXDB=Off \
    -DFLB_OUT_NATS=Off \
    -DFLB_OUT_TCP=Off \
    -DFLB_OUT_PLOT=Off \
    -DFLB_OUT_FILE=Off \
    -DFLB_OUT_TD=Off \
    -DFLB_OUT_RETRY=Off \
    -DFLB_OUT_SLACK=Off \
    -DFLB_OUT_SPLUNK=Off \
    -DFLB_OUT_STACKDRIVER=Off \
    -DFLB_OUT_STDOUT=On \
    -DFLB_OUT_LIB=Off \
    -DFLB_OUT_NULL=Off \
    -DFLB_OUT_FLOWCOUNTER=Off \
    -DFLB_FILTER_AWS=Off \
    -DFLB_FILTER_GREP=Off \
    -DFLB_FILTER_MODIFY=Off \
    -DFLB_FILTER_STDOUT=Off \
    -DFLB_FILTER_PARSER=Off \
    -DFLB_FILTER_KUBERNETES=Off \
    -DFLB_FILTER_REWRITE_TAG=Off \
    -DFLB_FILTER_THROTTLE=Off \
    -DFLB_FILTER_NEST=Off \
    -DFLB_FILTER_LUA=Off \
    -DFLB_FILTER_RECORD_MODIFIER=Off \
    -DFLB_PROXY_GO=Off .. && \
    make -j $(getconf _NPROCESSORS_ONLN) && echo $PWD && tree && install bin/fluent-bit /fluent-bit/bin/ && make clean
# RUN cat /usr/src/myapp/fluent-bit/build/Rust/src/flb-plugin-out_rust_stdout_target-stamp/flb-plugin-out_rust_stdout_target-build-*.log