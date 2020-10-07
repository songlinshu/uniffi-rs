// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!
{% import "macros.rs" as rs %}

{% include "RustBuffer.rs" %}

// We generate error mappings into ffi_support::ExternErrors
// so that the errors can propagate through the FFI
{% for e in ci.iter_error_definitions() %}
{% include "ErrorTemplate.rs" %}
{% endfor %}

// For each enum declared in the UDL, we assume the caller as provided a corresponding
// rust `enum`. We provide the traits for sending it across the FFI, which will fail to
// compile if the provided struct has a different shape to the one declared in the UDL.
//
// The enum will be sent over the FFI as a u32, with values assigned according to the
// order of items *as declared in the UDL file*. This might be different to the order
// of items as declared in the rust code, but no harm will come from it.
{% for e in ci.iter_enum_definitions() %}
{% include "EnumTemplate.rs" %}
{% endfor %}

// For each record declared in the UDL, we assume the caller has provided a corresponding
// rust `struct` with the declared fields. We provide the traits for sending it across the FFI.
// If the caller's struct does not match the shape and types declared in the UDL then the rust
// compiler will complain with a type error.
{% for rec in ci.iter_record_definitions() %}
{% include "RecordTemplate.rs" %}
{% endfor %}

// For each top-level function declared in the UDL, we assume the caller has provided a corresponding
// rust function of the same name. We provide a `pub extern "C"` wrapper that does type conversions to
// send data across the FFI, which will fail to compile if the provided function does not match what's
// specified in the UDL.
{%- for func in ci.iter_function_definitions() %}
{% include "TopLevelFunctionTemplate.rs" %}
{% endfor -%}

// For each Object definition, we assume the caller has provided an appropriately-shaped `struct`
// with an `impl` for each method on the object. We create a `ConcurrentHandleMap` for safely handing
// out references to these structs to foreign language code, and we provide a `pub extern "C"` function
// corresponding to each method.
//
// If the caller's implementation of the struct does not match with the methods or types specified
// in the UDL, then the rust compiler will complain with a (hopefully at least somewhat helpful!)
// error message when processing this generated code.
{% for obj in ci.iter_object_definitions() %}
{% include "ObjectTemplate.rs" %}
{% endfor %}

{%- import "macros.rs" as rs -%}
