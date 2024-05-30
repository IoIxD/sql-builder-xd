use core::panic;
use std::{
    ffi::{CStr, CString},
    mem::ManuallyDrop,
};

#[repr(C)]
pub struct SqlBuilder {
    inner: *mut sql_builder::SqlBuilder,
}

impl SqlBuilder {
    pub fn inner(&self) -> ManuallyDrop<&mut sql_builder::SqlBuilder> {
        if self.inner.is_null() {
            panic!("SqlBuilder.inner is null");
        }
        match unsafe { self.inner.as_mut() } {
            Some(a) => ManuallyDrop::new(a),
            None => panic!("SqlBuilder.inner is null"),
        }
    }
}

fn rewrap(this: &mut sql_builder::SqlBuilder) -> *mut SqlBuilder {
    return Box::leak(Box::new(SqlBuilder { inner: this })) as *mut SqlBuilder;
}

fn m<'a>(this: *mut SqlBuilder) -> ManuallyDrop<&'a mut sql_builder::SqlBuilder> {
    unsafe {
        match this.as_mut() {
            Some(a) => a.inner(),
            None => panic!("SqlBuilder is null"),
        }
    }
}

fn ptr_to_string(st: *mut i8) -> String {
    if st.is_null() {
        panic!("Null pointer passed to SqlBuilder.")
    }
    return unsafe { CStr::from_ptr(*st as *const i8) }
        .to_str()
        .unwrap()
        .to_string();
}
fn string_array_from_double_ptr(ptr: *const *mut i8, length: usize) -> Vec<String> {
    unsafe { std::slice::from_raw_parts(ptr, length) }
        .into_iter()
        .map(|f| {
            unsafe { CStr::from_ptr(*f as *const i8) }
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

#[no_mangle]
pub extern "C" fn new_sql_builder_select_from(table: *mut i8) -> *mut SqlBuilder {
    let one = ptr_to_string(table);
    let two = &mut sql_builder::SqlBuilder::select_from(one);
    return rewrap(two);
}
#[no_mangle]
pub extern "C" fn new_sql_builder_select_values(
    values: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(&mut sql_builder::SqlBuilder::select_values(
        &string_array_from_double_ptr(values, length),
    ));
}
#[no_mangle]
pub extern "C" fn new_sql_builder_insert_into(table: *mut i8) -> *mut SqlBuilder {
    return rewrap(&mut sql_builder::SqlBuilder::insert_into(ptr_to_string(
        table,
    )));
}
#[no_mangle]
pub extern "C" fn new_sql_builder_update_table(table: *mut i8) -> *mut SqlBuilder {
    return rewrap(&mut sql_builder::SqlBuilder::update_table(ptr_to_string(
        table,
    )));
}
#[no_mangle]
pub extern "C" fn new_sql_builder_delete_from(table: *mut i8) -> *mut SqlBuilder {
    return rewrap(&mut sql_builder::SqlBuilder::delete_from(ptr_to_string(
        table,
    )));
}

#[no_mangle]
pub extern "C" fn sql_builder_and_table(this: *mut SqlBuilder, table: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).and_table(ptr_to_string(table)));
}
#[no_mangle]
pub extern "C" fn sql_builder_natural(this: *mut SqlBuilder) -> *mut SqlBuilder {
    return rewrap(m(this).natural());
}
#[no_mangle]
pub extern "C" fn sql_builder_left(this: *mut SqlBuilder) -> *mut SqlBuilder {
    return rewrap(m(this).left());
}
#[no_mangle]
pub extern "C" fn sql_builder_left_outer(this: *mut SqlBuilder) -> *mut SqlBuilder {
    return rewrap(m(this).left_outer());
}
#[no_mangle]
pub extern "C" fn sql_builder_right(this: *mut SqlBuilder) -> *mut SqlBuilder {
    return rewrap(m(this).right());
}
#[no_mangle]
pub extern "C" fn sql_builder_right_outer(this: *mut SqlBuilder) -> *mut SqlBuilder {
    return rewrap(m(this).right_outer());
}
#[no_mangle]
pub extern "C" fn sql_builder_inner(this: *mut SqlBuilder) -> *mut SqlBuilder {
    return rewrap(m(this).inner());
}
#[no_mangle]
pub extern "C" fn sql_builder_cross(this: *mut SqlBuilder) -> *mut SqlBuilder {
    return rewrap(m(this).cross());
}
#[no_mangle]
pub extern "C" fn sql_builder_join(this: *mut SqlBuilder, table: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).join(ptr_to_string(table)));
}
#[no_mangle]
pub extern "C" fn sql_builder_on(this: *mut SqlBuilder, constraint: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).on(ptr_to_string(constraint)));
}
#[no_mangle]
pub extern "C" fn sql_builder_on_eq(
    this: *mut SqlBuilder,
    c1: *mut i8,
    c2: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).on_eq(ptr_to_string(c1), ptr_to_string(c2)));
}
#[no_mangle]
pub extern "C" fn sql_builder_distinct(this: *mut SqlBuilder) -> *mut SqlBuilder {
    return rewrap(m(this).distinct());
}
#[no_mangle]
pub extern "C" fn sql_builder_fields(
    this: *mut SqlBuilder,
    fields: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).fields(&string_array_from_double_ptr(fields, length)));
}
#[no_mangle]
pub extern "C" fn sql_builder_set_fields(
    this: *mut SqlBuilder,
    fields: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).set_fields(&string_array_from_double_ptr(fields, length)));
}
#[no_mangle]
pub extern "C" fn sql_builder_field(this: *mut SqlBuilder, field: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).field(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_set_field(this: *mut SqlBuilder, field: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).set_field(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_count(this: *mut SqlBuilder, field: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).count(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_count_as(
    this: *mut SqlBuilder,
    field: *mut i8,
    name: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).count_as(ptr_to_string(field), ptr_to_string(name)));
}
#[no_mangle]
pub extern "C" fn sql_builder_set(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).set(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_set_str(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).set_str(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_values(
    this: *mut SqlBuilder,
    values: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).values(&string_array_from_double_ptr(values, length)));
}
#[no_mangle]
pub extern "C" fn sql_builder_select(this: *mut SqlBuilder, query: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).select(ptr_to_string(query)));
}
#[no_mangle]
pub extern "C" fn sql_builder_returning(this: *mut SqlBuilder, field: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).returning(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_returning_id(this: *mut SqlBuilder) -> *mut SqlBuilder {
    return rewrap(m(this).returning_id());
}
#[no_mangle]
pub extern "C" fn sql_builder_group_by(this: *mut SqlBuilder, field: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).group_by(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_having(this: *mut SqlBuilder, cond: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).having(ptr_to_string(cond)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where(this: *mut SqlBuilder, cond: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).and_where(ptr_to_string(cond)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_eq(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_eq(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_ne(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_ne(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_gt(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_gt(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_ge(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_ge(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_lt(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_lt(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_le(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_le(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_like(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_like(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_like_right(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_like_right(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_like_left(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_like_left(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_like_any(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_like_any(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_not_like(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_not_like(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_not_like_right(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_not_like_right(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_not_like_left(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_not_like_left(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_not_like_any(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_not_like_any(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_is_null(
    this: *mut SqlBuilder,
    field: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_is_null(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_is_not_null(
    this: *mut SqlBuilder,
    field: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_is_not_null(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_in(
    this: *mut SqlBuilder,
    field: *mut i8,
    list: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_in(
        ptr_to_string(field),
        &string_array_from_double_ptr(list, length),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_in_quoted(
    this: *mut SqlBuilder,
    field: *mut i8,
    list: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_in_quoted(
        ptr_to_string(field),
        &string_array_from_double_ptr(list, length),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_not_in(
    this: *mut SqlBuilder,
    field: *mut i8,
    list: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_not_in(
        ptr_to_string(field),
        &string_array_from_double_ptr(list, length),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_not_in_quoted(
    this: *mut SqlBuilder,
    field: *mut i8,
    list: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_not_in_quoted(
        ptr_to_string(field),
        &string_array_from_double_ptr(list, length),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_in_query(
    this: *mut SqlBuilder,
    field: *mut i8,
    query: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_in_query(ptr_to_string(field), ptr_to_string(query)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_not_in_query(
    this: *mut SqlBuilder,
    field: *mut i8,
    query: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_not_in_query(ptr_to_string(field), ptr_to_string(query)));
}
#[no_mangle]
pub extern "C" fn sql_builder_and_where_between(
    this: *mut SqlBuilder,
    field: *mut i8,
    min: *mut i8,
    max: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_between(
        ptr_to_string(field),
        ptr_to_string(min),
        ptr_to_string(max),
    ));
}

#[no_mangle]
pub extern "C" fn sql_builder_and_where_not_between(
    this: *mut SqlBuilder,
    field: *mut i8,
    min: *mut i8,
    max: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).and_where_not_between(
        ptr_to_string(field),
        ptr_to_string(min),
        ptr_to_string(max),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where(this: *mut SqlBuilder, cond: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).or_where(ptr_to_string(cond)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_eq(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_eq(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_ne(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_ne(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_gt(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_gt(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_ge(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_ge(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_lt(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_lt(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_le(
    this: *mut SqlBuilder,
    field: *mut i8,
    value: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_le(ptr_to_string(field), ptr_to_string(value)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_like(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_like(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_like_right(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_like_right(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_like_left(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_like_left(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_like_any(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_like_any(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_not_like(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_not_like(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_not_like_right(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_not_like_right(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_not_like_left(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_not_like_left(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_not_like_any(
    this: *mut SqlBuilder,
    field: *mut i8,
    mask: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_not_like_any(ptr_to_string(field), ptr_to_string(mask)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_is_null(
    this: *mut SqlBuilder,
    field: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_is_null(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_is_not_null(
    this: *mut SqlBuilder,
    field: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_is_not_null(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_in(
    this: *mut SqlBuilder,
    field: *mut i8,
    list: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_in(
        ptr_to_string(field),
        &string_array_from_double_ptr(list, length),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_in_quoted(
    this: *mut SqlBuilder,
    field: *mut i8,
    list: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_in_quoted(
        ptr_to_string(field),
        &string_array_from_double_ptr(list, length),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_not_in(
    this: *mut SqlBuilder,
    field: *mut i8,
    list: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_not_in(
        ptr_to_string(field),
        &string_array_from_double_ptr(list, length),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_not_in_quoted(
    this: *mut SqlBuilder,
    field: *mut i8,
    list: *const *mut i8,
    length: usize,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_not_in_quoted(
        ptr_to_string(field),
        &string_array_from_double_ptr(list, length),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_in_query(
    this: *mut SqlBuilder,
    field: *mut i8,
    query: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_in_query(ptr_to_string(field), ptr_to_string(query)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_not_in_query(
    this: *mut SqlBuilder,
    field: *mut i8,
    query: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_not_in_query(ptr_to_string(field), ptr_to_string(query)));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_between(
    this: *mut SqlBuilder,
    field: *mut i8,
    min: *mut i8,
    max: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_between(
        ptr_to_string(field),
        ptr_to_string(min),
        ptr_to_string(max),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_or_where_not_between(
    this: *mut SqlBuilder,
    field: *mut i8,
    min: *mut i8,
    max: *mut i8,
) -> *mut SqlBuilder {
    return rewrap(m(this).or_where_between(
        ptr_to_string(field),
        ptr_to_string(min),
        ptr_to_string(max),
    ));
}
#[no_mangle]
pub extern "C" fn sql_builder_union(this: *mut SqlBuilder, query: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).union(ptr_to_string(query)));
}
#[no_mangle]
pub extern "C" fn sql_builder_union_all(this: *mut SqlBuilder, query: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).union_all(ptr_to_string(query)));
}
#[no_mangle]
pub extern "C" fn sql_builder_order_by(
    this: *mut SqlBuilder,
    field: *mut i8,
    desc: bool,
) -> *mut SqlBuilder {
    return rewrap(m(this).order_by(ptr_to_string(field), desc));
}
#[no_mangle]
pub extern "C" fn sql_builder_order_asc(this: *mut SqlBuilder, field: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).order_asc(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_order_desc(this: *mut SqlBuilder, field: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).order_desc(ptr_to_string(field)));
}
#[no_mangle]
pub extern "C" fn sql_builder_limit(this: *mut SqlBuilder, limit: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).limit(ptr_to_string(limit)));
}
#[no_mangle]
pub extern "C" fn sql_builder_offset(this: *mut SqlBuilder, offset: *mut i8) -> *mut SqlBuilder {
    return rewrap(m(this).offset(ptr_to_string(offset)));
}

static mut RUST_ERROR: *mut u8 = std::ptr::null_mut();

#[no_mangle]
pub extern "C" fn get_error() -> *mut u8 {
    return unsafe { RUST_ERROR };
}

pub extern "C" fn set_error(err: impl ToString) {
    println!("setting error to {}", err.to_string());
    unsafe { RUST_ERROR = err.to_string().as_mut_ptr() };
}

#[no_mangle]
pub extern "C" fn sql_builder_sql(this: *mut SqlBuilder) -> *const u8 {
    let t = m(this);
    let r = t.sql();
    match r {
        Ok(a) => a.as_ptr(),
        Err(err) => {
            set_error(err);
            std::ptr::null()
        }
    }
}
#[no_mangle]
pub extern "C" fn sql_builder_subquery(this: *mut SqlBuilder) -> *const u8 {
    match m(this).subquery() {
        Ok(a) => a.as_ptr(),
        Err(err) => {
            set_error(err);
            std::ptr::null()
        }
    }
}
#[no_mangle]
pub extern "C" fn sql_builder_subquery_as(this: *mut SqlBuilder, name: *mut i8) -> *const u8 {
    match m(this).subquery_as(ptr_to_string(name)) {
        Ok(a) => a.as_ptr(),
        Err(err) => {
            set_error(err);
            std::ptr::null()
        }
    }
}
#[no_mangle]
pub extern "C" fn sql_builder_query(this: *mut SqlBuilder) -> *const u8 {
    match m(this).query() {
        Ok(a) => a.as_ptr(),
        Err(err) => {
            set_error(err);
            std::ptr::null()
        }
    }
}
#[no_mangle]
pub extern "C" fn sql_builder_query_values(this: *mut SqlBuilder) -> *const u8 {
    match m(this).query_values() {
        Ok(a) => a.as_ptr(),
        Err(err) => {
            set_error(err);
            std::ptr::null()
        }
    }
}
