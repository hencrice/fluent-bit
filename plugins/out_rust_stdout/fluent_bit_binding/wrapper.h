#include <fluent-bit/flb_input.h>
#include <fluent-bit/flb_filter.h>
#include <fluent-bit/flb_output.h>
#include <fluent-bit/flb_pack.h>
// this is not strictly needed for plugins
// just using this to generate a struct we need
// for the stdout Rust plugin
#include <fluent-bit/flb_config_map.h>
#include <flb_stdout.h>