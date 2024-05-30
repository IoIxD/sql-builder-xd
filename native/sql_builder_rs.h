#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct SqlBuilder
{
  void *inner;
} SqlBuilder;

extern uint8_t *RUST_ERROR;

SqlBuilder *new_sql_builder_select_from(int8_t *table);

SqlBuilder *new_sql_builder_select_values(int8_t *const *values, uintptr_t length);

SqlBuilder *new_sql_builder_insert_into(int8_t *table);

SqlBuilder *new_sql_builder_update_table(int8_t *table);

SqlBuilder *new_sql_builder_delete_from(int8_t *table);

SqlBuilder *sql_builder_and_table(SqlBuilder *this_, int8_t *table);

SqlBuilder *sql_builder_natural(SqlBuilder *this_);

SqlBuilder *sql_builder_left(SqlBuilder *this_);

SqlBuilder *sql_builder_left_outer(SqlBuilder *this_);

SqlBuilder *sql_builder_right(SqlBuilder *this_);

SqlBuilder *sql_builder_right_outer(SqlBuilder *this_);

SqlBuilder *sql_builder_inner(SqlBuilder *this_);

SqlBuilder *sql_builder_cross(SqlBuilder *this_);

SqlBuilder *sql_builder_join(SqlBuilder *this_, int8_t *table);

SqlBuilder *sql_builder_on(SqlBuilder *this_, int8_t *constraint);

SqlBuilder *sql_builder_on_eq(SqlBuilder *this_, int8_t *c1, int8_t *c2);

SqlBuilder *sql_builder_distinct(SqlBuilder *this_);

SqlBuilder *sql_builder_fields(SqlBuilder *this_,
                               int8_t *const *fields,
                               uintptr_t length);

SqlBuilder *sql_builder_set_fields(SqlBuilder *this_,
                                   int8_t *const *fields,
                                   uintptr_t length);

SqlBuilder *sql_builder_field(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_set_field(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_count(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_count_as(SqlBuilder *this_, int8_t *field, int8_t *name);

SqlBuilder *sql_builder_set(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_set_str(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_values(SqlBuilder *this_,
                               int8_t *const *values,
                               uintptr_t length);

SqlBuilder *sql_builder_select(SqlBuilder *this_, int8_t *query);

SqlBuilder *sql_builder_returning(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_returning_id(SqlBuilder *this_);

SqlBuilder *sql_builder_group_by(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_having(SqlBuilder *this_, int8_t *cond);

SqlBuilder *sql_builder_and_where(SqlBuilder *this_, int8_t *cond);

SqlBuilder *sql_builder_and_where_eq(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_and_where_ne(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_and_where_gt(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_and_where_ge(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_and_where_lt(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_and_where_le(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_and_where_like(SqlBuilder *this_,
                                       int8_t *field,
                                       int8_t *mask);

SqlBuilder *sql_builder_and_where_like_right(SqlBuilder *this_,
                                             int8_t *field,
                                             int8_t *mask);

SqlBuilder *sql_builder_and_where_like_left(SqlBuilder *this_,
                                            int8_t *field,
                                            int8_t *mask);

SqlBuilder *sql_builder_and_where_like_any(SqlBuilder *this_,
                                           int8_t *field,
                                           int8_t *mask);

SqlBuilder *sql_builder_and_where_not_like(SqlBuilder *this_,
                                           int8_t *field,
                                           int8_t *mask);

SqlBuilder *sql_builder_and_where_not_like_right(SqlBuilder *this_,
                                                 int8_t *field,
                                                 int8_t *mask);

SqlBuilder *sql_builder_and_where_not_like_left(SqlBuilder *this_,
                                                int8_t *field,
                                                int8_t *mask);

SqlBuilder *sql_builder_and_where_not_like_any(SqlBuilder *this_,
                                               int8_t *field,
                                               int8_t *mask);

SqlBuilder *sql_builder_and_where_is_null(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_and_where_is_not_null(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_and_where_in(SqlBuilder *this_,
                                     int8_t *field,
                                     int8_t *const *list,
                                     uintptr_t length);

SqlBuilder *sql_builder_and_where_in_quoted(SqlBuilder *this_,
                                            int8_t *field,
                                            int8_t *const *list,
                                            uintptr_t length);

SqlBuilder *sql_builder_and_where_not_in(SqlBuilder *this_,
                                         int8_t *field,
                                         int8_t *const *list,
                                         uintptr_t length);

SqlBuilder *sql_builder_and_where_not_in_quoted(SqlBuilder *this_,
                                                int8_t *field,
                                                int8_t *const *list,
                                                uintptr_t length);

SqlBuilder *sql_builder_and_where_in_query(SqlBuilder *this_,
                                           int8_t *field,
                                           int8_t *query);

SqlBuilder *sql_builder_and_where_not_in_query(SqlBuilder *this_,
                                               int8_t *field,
                                               int8_t *query);

SqlBuilder *sql_builder_and_where_between(SqlBuilder *this_,
                                          int8_t *field,
                                          int8_t *min,
                                          int8_t *max);

SqlBuilder *sql_builder_and_where_not_between(SqlBuilder *this_,
                                              int8_t *field,
                                              int8_t *min,
                                              int8_t *max);

SqlBuilder *sql_builder_or_where(SqlBuilder *this_, int8_t *cond);

SqlBuilder *sql_builder_or_where_eq(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_or_where_ne(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_or_where_gt(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_or_where_ge(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_or_where_lt(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_or_where_le(SqlBuilder *this_, int8_t *field, int8_t *value);

SqlBuilder *sql_builder_or_where_like(SqlBuilder *this_, int8_t *field, int8_t *mask);

SqlBuilder *sql_builder_or_where_like_right(SqlBuilder *this_,
                                            int8_t *field,
                                            int8_t *mask);

SqlBuilder *sql_builder_or_where_like_left(SqlBuilder *this_,
                                           int8_t *field,
                                           int8_t *mask);

SqlBuilder *sql_builder_or_where_like_any(SqlBuilder *this_,
                                          int8_t *field,
                                          int8_t *mask);

SqlBuilder *sql_builder_or_where_not_like(SqlBuilder *this_,
                                          int8_t *field,
                                          int8_t *mask);

SqlBuilder *sql_builder_or_where_not_like_right(SqlBuilder *this_,
                                                int8_t *field,
                                                int8_t *mask);

SqlBuilder *sql_builder_or_where_not_like_left(SqlBuilder *this_,
                                               int8_t *field,
                                               int8_t *mask);

SqlBuilder *sql_builder_or_where_not_like_any(SqlBuilder *this_,
                                              int8_t *field,
                                              int8_t *mask);

SqlBuilder *sql_builder_or_where_is_null(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_or_where_is_not_null(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_or_where_in(SqlBuilder *this_,
                                    int8_t *field,
                                    int8_t *const *list,
                                    uintptr_t length);

SqlBuilder *sql_builder_or_where_in_quoted(SqlBuilder *this_,
                                           int8_t *field,
                                           int8_t *const *list,
                                           uintptr_t length);

SqlBuilder *sql_builder_or_where_not_in(SqlBuilder *this_,
                                        int8_t *field,
                                        int8_t *const *list,
                                        uintptr_t length);

SqlBuilder *sql_builder_or_where_not_in_quoted(SqlBuilder *this_,
                                               int8_t *field,
                                               int8_t *const *list,
                                               uintptr_t length);

SqlBuilder *sql_builder_or_where_in_query(SqlBuilder *this_,
                                          int8_t *field,
                                          int8_t *query);

SqlBuilder *sql_builder_or_where_not_in_query(SqlBuilder *this_,
                                              int8_t *field,
                                              int8_t *query);

SqlBuilder *sql_builder_or_where_between(SqlBuilder *this_,
                                         int8_t *field,
                                         int8_t *min,
                                         int8_t *max);

SqlBuilder *sql_builder_or_where_not_between(SqlBuilder *this_,
                                             int8_t *field,
                                             int8_t *min,
                                             int8_t *max);

SqlBuilder *sql_builder_union(SqlBuilder *this_, int8_t *query);

SqlBuilder *sql_builder_union_all(SqlBuilder *this_, int8_t *query);

SqlBuilder *sql_builder_order_by(SqlBuilder *this_, int8_t *field, bool desc);

SqlBuilder *sql_builder_order_asc(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_order_desc(SqlBuilder *this_, int8_t *field);

SqlBuilder *sql_builder_limit(SqlBuilder *this_, int8_t *limit);

SqlBuilder *sql_builder_offset(SqlBuilder *this_, int8_t *offset);

const uint8_t *sql_builder_sql(SqlBuilder *this_);

const uint8_t *sql_builder_subquery(SqlBuilder *this_);

const uint8_t *sql_builder_subquery_as(SqlBuilder *this_, int8_t *name);

const uint8_t *sql_builder_query(SqlBuilder *this_);

const uint8_t *sql_builder_query_values(SqlBuilder *this_);
