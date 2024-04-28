# C-Rust (inter)Operability Kit

An in-development workspace for Rust-C Interoperability Tooling

## How does it work?

```rust
/// Test Structure
#[derive(Debug, Default, PartialEq, Clone, CBuilder)]
pub struct TestScruct {
    field_a: f64,
    field_b: String,
    field_c: Option<u32>,
    field_d: Vec<u32>,
    field_e: InnerStruct,
    field_f: InnerEnum,
}

#[derive(Debug, Default, PartialEq, Clone, CBuilder)]
#[c_builder(constructor = (field_1,))]
pub struct InnerStruct {
    field_1: String,
}


#[derive(Debug, PartialEq, Clone, CBuilder)]
pub enum InnerEnum {
    None,
    Option1(u32),
    Option2(f64)
}

impl Default for InnerEnum {
    fn default() -> Self {
        Self::None
    }
}

```

By default the `CBuilder` will generate three basic functions for constructing, debugging, and cloning a structure:
```rust
/// Test Scructure
#[no_mangle]
pub extern "C" fn test_scruct_default() -> *mut TestScruct {
    Box::into_raw(Box::default())
}
/// Test Scructure
#[no_mangle]
pub extern "C" fn test_scruct_clone(s: &TestScruct) -> *mut TestScruct {
    Box::into_raw(Box::new(s.clone()))
}
/// Test Scructure
#[no_mangle]
pub extern "C" fn test_scruct_debug(s: &TestScruct) {
    {
        ::std::io::_print(format_args!("{0:?}\n", s));
    };
}
```

Then, it will construct the extra constructors that we specified. Here is one example from the `InnerStruct`:
```rust
fn inner_inner_struct_from_field_1(
    field_1: &::libc::c_char,
) -> Result<InnerStruct, String> {
    let mut res = InnerStruct::default();
    res.field_1 = utils::as_string(field_1)?;
    Ok(res)
}
#[no_mangle]
pub extern "C" fn inner_struct_from_field_1(
    field_1: &::libc::c_char,
) -> *mut InnerStruct {
    let res = inner_inner_struct_from_field_1(field_1)
        .expect(
            &{
                let res = ::alloc::fmt::format(
                    format_args!("Error creating: {0:?}", "InnerStruct"),
                );
                res
            },
        );
    Box::into_raw(Box::new(res))
}
```

The top function here is a non-c API like function that masks the C-like "Return an `int` error code" functionality with the rust `?` return a result simplicity.

The lower function is in fact the C API.

Next, it will generate mutators, for example.
```rust
/// Replaces the current value with the provided value
fn _inner_test_scruct_with_field_a(
    source: *mut TestScruct,
    value: f64,
) -> utils::CResult {
    utils::check_null(source)
        .map_err(|e| {
            let res = ::alloc::fmt::format(
                format_args!("{1} ({0})", "TestScruct", e),
            );
            res
        })?
        .field_a = value;
    Ok(())
}
/// Replaces the current value with the provided value
#[no_mangle]
pub extern "C" fn test_scruct_with_field_a(
    source: *mut TestScruct,
    value: f64,
) -> i32 {
    match _inner_test_scruct_with_field_a(source, value) {
        Ok(_) => 0,
        Err(e) => {
            {
                ::std::io::_eprint(format_args!("{0:?}\n", e));
            };
            1
        }
    }
}
```

Different wrapper types get different methods:

All of these will return an error code if you pass in a null pointer

- `Vec`
    - `push`: Push an element to the end of the vector (rust creates a Clone of your data structure)
    - `get`: Get a copy of an element at index `idx` - we return a copy so you don't accidentaly leave rust with an invalid pointer in it's vector.
        - Will return error code if `idx` is out of range
    - `remove`: Remove and get an element at index `idx`
        - Will return error code is `idx` is out of range 
- `Option`
    - `get`: Get a copy to to the value inside the option
        - Will return error code is `Option::is_none`
    - `replace`: Replace the value in the option
    - `take`: Take and return the value inside the option.
        - Will return error code if `Option::is_none`


We then use `ctypesgen` to build a c-api based on these `#[no_mangle]` functions. So the LLA api will look like this:

```c
/**
 * Test Scructure
 */
struct TestScruct *test_scruct_clone(const struct TestScruct *s);

/**
 * Test Scructure
 */
void test_scruct_debug(const struct TestScruct *s);

/**
 * Test Scructure
 */
struct TestScruct *test_scruct_default(void);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_get_field_a(const struct TestScruct *source, double *c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_get_field_b(const struct TestScruct *source, struct StringBuffer c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_get_field_c(const struct TestScruct *source, uint32_t *c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_get_field_d(const struct TestScruct *source, size_t idx, uint32_t *c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_get_field_e(const struct TestScruct *source, struct InnerStruct *c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_get_field_f(const struct TestScruct *source, struct InnerEnum *c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_push_field_d(struct TestScruct *source, uint32_t value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_remove_field_d(struct TestScruct *source, size_t idx, uint32_t *c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_replace_field_c(struct TestScruct *source, uint32_t value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_take_field_c(struct TestScruct *source, uint32_t *c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_with_field_a(struct TestScruct *source, double value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_with_field_b(struct TestScruct *source, const char *value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_with_field_e(struct TestScruct *source, const struct InnerStruct *value);

/**
 * Replaces the current value with the provided value
 */
int32_t test_scruct_with_field_f(struct TestScruct *source, const struct InnerEnum *value);


int32_t inner_enum_as_none(struct InnerEnum *res);

int32_t inner_enum_as_option_1(struct InnerEnum *res, uint32_t value);

int32_t inner_enum_as_option_2(struct InnerEnum *res, double value);

struct InnerEnum *inner_enum_clone(const struct InnerEnum *s);

void inner_enum_debug(const struct InnerEnum *s);

struct InnerEnum *inner_enum_default(void);

struct InnerEnum *inner_enum_from_none(void);

struct InnerEnum *inner_enum_from_option_1(uint32_t value);

struct InnerEnum *inner_enum_from_option_2(double value);

struct InnerStruct *inner_struct_clone(const struct InnerStruct *s);

void inner_struct_debug(const struct InnerStruct *s);

struct InnerStruct *inner_struct_default(void);

struct InnerStruct *inner_struct_from_field_1(const char *field_1);

/**
 * Replaces the current value with the provided value
 */
int32_t inner_struct_get_field_1(const struct InnerStruct *source, struct StringBuffer c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t inner_struct_with_field_1(struct InnerStruct *source, const char *value);
```

## Demo

Want to try it out?

1. First `nix develop` in the root.

2. The `simple` example will be available to play with in the python repl.
```bash
$ nix develop
$ python
>>> import simple
>>> b = simple.brush_default()
>>> simple.brush_debug(b)
```
