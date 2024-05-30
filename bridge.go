package main

// #cgo CFLAGS: -g -Wall
// #cgo LDFLAGS: ./native/libnative.a -ldl
// #include "native/sql_builder_rs.h"
// #include <stdlib.h>
import "C"
import (
	"unsafe"
)

type SqlBuilderStart struct{}

func NewSqlBuilder() *SqlBuilderStart {
	return &SqlBuilderStart{}
}

type SqlBuilder struct {
	ptr *C.SqlBuilder
}

func (*SqlBuilderStart) SelectFrom(table string) *SqlBuilder {
	st, _ := C.CString(table)
	s := C.new_sql_builder_select_from((*C.int8_t)(unsafe.Pointer(st)))
	C.free(unsafe.Pointer(st))
	return &SqlBuilder{s}
}

func (*SqlBuilderStart) SelectValues(values []string) *SqlBuilder {
	vals := make([]*C.int8_t, len(values))
	for i, v := range values {
		st, _ := C.CString(v)
		vals[i] = (*C.int8_t)(st)
	}
	s := C.new_sql_builder_select_values((**C.int8_t)(&vals[0]), (C.ulong)(len(values)))
	for _, v := range vals {
		C.free(unsafe.Pointer(v))

	}
	return &SqlBuilder{s}
}

/*

pub fn select_from<S: ToString>(table: S) -> Self
[src][+]
pub fn and_table<S: ToString>(&mut self, table: S) -> &mut Self
[src][+]
pub fn select_values<S: ToString>(values: &[S]) -> Self
[src][+]
pub fn insert_into<S: ToString>(table: S) -> Self
[src][+]
pub fn update_table<S: ToString>(table: S) -> Self
[src][+]
pub fn delete_from<S: ToString>(table: S) -> Self
[src][+]
pub fn natural(&mut self) -> &mut Self
[src][+]
pub fn left(&mut self) -> &mut Self
[src][+]
pub fn left_outer(&mut self) -> &mut Self
[src][+]
pub fn right(&mut self) -> &mut Self
[src][+]
pub fn right_outer(&mut self) -> &mut Self
[src][+]
pub fn inner(&mut self) -> &mut Self
[src][+]
pub fn cross(&mut self) -> &mut Self
[src][+]
pub fn join<S: ToString>(&mut self, table: S) -> &mut Self
[src][+]
pub fn on<S: ToString>(&mut self, constraint: S) -> &mut Self
[src][+]
pub fn on_eq<S: ToString, T: ToString>(&mut self, c1: S, c2: T) -> &mut Self
[src][+]
pub fn distinct(&mut self) -> &mut Self
[src][+]
pub fn fields<S: ToString>(&mut self, fields: &[S]) -> &mut Self
[src][+]
pub fn set_fields<S: ToString>(&mut self, fields: &[S]) -> &mut Self
[src][+]
pub fn field<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn set_field<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn count<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn count_as<S, T>(&mut self, field: S, name: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn set<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn set_str<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn values<S: ToString>(&mut self, values: &[S]) -> &mut Self
[src][+]
pub fn select<S: ToString>(&mut self, query: S) -> &mut Self
[src][+]
pub fn returning<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn returning_id(&mut self) -> &mut Self
[src][+]
pub fn group_by<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn having<S: ToString>(&mut self, cond: S) -> &mut Self
[src][+]
pub fn and_where<S: ToString>(&mut self, cond: S) -> &mut Self
[src][+]
pub fn and_where_eq<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_ne<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_gt<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_ge<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_lt<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_le<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_like<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_like_right<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_like_left<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_like_any<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_not_like<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_not_like_right<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_not_like_left<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_not_like_any<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_is_null<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn and_where_is_not_null<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn and_where_in<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_in_quoted<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_not_in<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_not_in_quoted<S, T>(
    &mut self,
    field: S,
    list: &[T]
) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_in_query<S, T>(&mut self, field: S, query: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_not_in_query<S, T>(&mut self, field: S, query: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn and_where_between<S, T, U>(
    &mut self,
    field: S,
    min: T,
    max: U
) -> &mut Self
where
    S: ToString,
    T: ToString,
    U: ToString,
[src][+]
pub fn and_where_not_between<S, T, U>(
    &mut self,
    field: S,
    min: T,
    max: U
) -> &mut Self
where
    S: ToString,
    T: ToString,
    U: ToString,
[src][+]
pub fn or_where<S: ToString>(&mut self, cond: S) -> &mut Self
[src][+]
pub fn or_where_eq<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_ne<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_gt<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_ge<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_lt<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_le<S, T>(&mut self, field: S, value: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_like<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_like_right<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_like_left<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_like_any<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_not_like<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_not_like_right<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_not_like_left<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_not_like_any<S, T>(&mut self, field: S, mask: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_is_null<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn or_where_is_not_null<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn or_where_in<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_in_quoted<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_not_in<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_not_in_quoted<S, T>(
    &mut self,
    field: S,
    list: &[T]
) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_in_query<S, T>(&mut self, field: S, query: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_not_in_query<S, T>(&mut self, field: S, query: T) -> &mut Self
where
    S: ToString,
    T: ToString,
[src][+]
pub fn or_where_between<S, T, U>(
    &mut self,
    field: S,
    min: T,
    max: U
) -> &mut Self
where
    S: ToString,
    T: ToString,
    U: ToString,
[src][+]
pub fn or_where_not_between<S, T, U>(
    &mut self,
    field: S,
    min: T,
    max: U
) -> &mut Self
where
    S: ToString,
    T: ToString,
    U: ToString,
[src][+]
pub fn union<S: ToString>(&mut self, query: S) -> &mut Self
[src][+]
pub fn union_all<S: ToString>(&mut self, query: S) -> &mut Self
[src][+]
pub fn order_by<S: ToString>(&mut self, field: S, desc: bool) -> &mut Self
[src][+]
pub fn order_asc<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn order_desc<S: ToString>(&mut self, field: S) -> &mut Self
[src][+]
pub fn limit<S: ToString>(&mut self, limit: S) -> &mut Self
[src][+]
pub fn offset<S: ToString>(&mut self, offset: S) -> &mut Self
[src][+]
pub fn sql(&self) -> Result<String>
[src][+]
pub fn subquery(&self) -> Result<String>
[src][+]
pub fn subquery_as<S: ToString>(&self, name: S) -> Result<String>
[src][+]
pub fn query(&self) -> Result<String>
[src][+]
pub fn query_values(&self) -> Result<String>
[src][+]
*/
