#include <stdbool.h>
#include <stdint.h>

typedef struct SqlBuilder
{
  void *inner;
} SqlBuilder;

extern uint8_t *RUST_ERROR;

struct SqlBuilder *new_sql_builder_select_from(const void *table);

struct SqlBuilder *new_sql_builder_select_values(const void *const *values,
                                                 uintptr_t length);

struct SqlBuilder *new_sql_builder_insert_into(const void *table);

struct SqlBuilder *new_sql_builder_update_table(const void *table);

struct SqlBuilder *new_sql_builder_delete_from(const void *table);

const void *new_rust_string(uint8_t *st);

struct SqlBuilder *sql_builder_and_table(struct SqlBuilder *this_, const void *table);

struct SqlBuilder *sql_builder_natural(struct SqlBuilder *this_);

struct SqlBuilder *sql_builder_left(struct SqlBuilder *this_);

struct SqlBuilder *sql_builder_left_outer(struct SqlBuilder *this_);

struct SqlBuilder *sql_builder_right(struct SqlBuilder *this_);

struct SqlBuilder *sql_builder_right_outer(struct SqlBuilder *this_);

struct SqlBuilder *sql_builder_inner(struct SqlBuilder *this_);

struct SqlBuilder *sql_builder_cross(struct SqlBuilder *this_);

struct SqlBuilder *sql_builder_join(struct SqlBuilder *this_, const void *table);

struct SqlBuilder *sql_builder_on(struct SqlBuilder *this_, const void *constraint);

struct SqlBuilder *sql_builder_on_eq(struct SqlBuilder *this_,
                                     const void *c1,
                                     const void *c2);

struct SqlBuilder *sql_builder_distinct(struct SqlBuilder *this_);

struct SqlBuilder *sql_builder_fields(struct SqlBuilder *this_,
                                      const void *const *fields,
                                      uintptr_t length);

struct SqlBuilder *sql_builder_set_fields(struct SqlBuilder *this_,
                                          const void *const *fields,
                                          uintptr_t length);

struct SqlBuilder *sql_builder_field(struct SqlBuilder *this_, const void *field);

struct SqlBuilder *sql_builder_set_field(struct SqlBuilder *this_, const void *field);

struct SqlBuilder *sql_builder_count(struct SqlBuilder *this_, const void *field);

struct SqlBuilder *sql_builder_count_as(struct SqlBuilder *this_,
                                        const void *field,
                                        const void *name);

struct SqlBuilder *sql_builder_set(struct SqlBuilder *this_,
                                   const void *field,
                                   const void *value);

struct SqlBuilder *sql_builder_set_str(struct SqlBuilder *this_,
                                       const void *field,
                                       const void *value);

struct SqlBuilder *sql_builder_values(struct SqlBuilder *this_,
                                      const void *const *values,
                                      uintptr_t length);

struct SqlBuilder *sql_builder_select(struct SqlBuilder *this_, const void *query);

struct SqlBuilder *sql_builder_returning(struct SqlBuilder *this_, const void *field);

struct SqlBuilder *sql_builder_returning_id(struct SqlBuilder *this_);

struct SqlBuilder *sql_builder_group_by(struct SqlBuilder *this_, const void *field);

struct SqlBuilder *sql_builder_having(struct SqlBuilder *this_, const void *cond);

struct SqlBuilder *sql_builder_and_where(struct SqlBuilder *this_, const void *cond);

struct SqlBuilder *sql_builder_and_where_eq(struct SqlBuilder *this_,
                                            const void *field,
                                            const void *value);

struct SqlBuilder *sql_builder_and_where_ne(struct SqlBuilder *this_,
                                            const void *field,
                                            const void *value);

struct SqlBuilder *sql_builder_and_where_gt(struct SqlBuilder *this_,
                                            const void *field,
                                            const void *value);

struct SqlBuilder *sql_builder_and_where_ge(struct SqlBuilder *this_,
                                            const void *field,
                                            const void *value);

struct SqlBuilder *sql_builder_and_where_lt(struct SqlBuilder *this_,
                                            const void *field,
                                            const void *value);

struct SqlBuilder *sql_builder_and_where_le(struct SqlBuilder *this_,
                                            const void *field,
                                            const void *value);

struct SqlBuilder *sql_builder_and_where_like(struct SqlBuilder *this_,
                                              const void *field,
                                              const void *mask);

struct SqlBuilder *sql_builder_and_where_like_right(struct SqlBuilder *this_,
                                                    const void *field,
                                                    const void *mask);

struct SqlBuilder *sql_builder_and_where_like_left(struct SqlBuilder *this_,
                                                   const void *field,
                                                   const void *mask);

struct SqlBuilder *sql_builder_and_where_like_any(struct SqlBuilder *this_,
                                                  const void *field,
                                                  const void *mask);

struct SqlBuilder *sql_builder_and_where_not_like(struct SqlBuilder *this_,
                                                  const void *field,
                                                  const void *mask);

struct SqlBuilder *sql_builder_and_where_not_like_right(struct SqlBuilder *this_,
                                                        const void *field,
                                                        const void *mask);

struct SqlBuilder *sql_builder_and_where_not_like_left(struct SqlBuilder *this_,
                                                       const void *field,
                                                       const void *mask);

struct SqlBuilder *sql_builder_and_where_not_like_any(struct SqlBuilder *this_,
                                                      const void *field,
                                                      const void *mask);

struct SqlBuilder *sql_builder_and_where_is_null(struct SqlBuilder *this_,
                                                 const void *field);

struct SqlBuilder *sql_builder_and_where_is_not_null(struct SqlBuilder *this_,
                                                     const void *field);

struct SqlBuilder *sql_builder_and_where_in(struct SqlBuilder *this_,
                                            const void *field,
                                            const void *const *list,
                                            uintptr_t length);

struct SqlBuilder *sql_builder_and_where_in_quoted(struct SqlBuilder *this_,
                                                   const void *field,
                                                   const void *const *list,
                                                   uintptr_t length);

struct SqlBuilder *sql_builder_and_where_not_in(struct SqlBuilder *this_,
                                                const void *field,
                                                const void *const *list,
                                                uintptr_t length);

struct SqlBuilder *sql_builder_and_where_not_in_quoted(struct SqlBuilder *this_,
                                                       const void *field,
                                                       const void *const *list,
                                                       uintptr_t length);

struct SqlBuilder *sql_builder_and_where_in_query(struct SqlBuilder *this_,
                                                  const void *field,
                                                  const void *query);

struct SqlBuilder *sql_builder_and_where_not_in_query(struct SqlBuilder *this_,
                                                      const void *field,
                                                      const void *query);

struct SqlBuilder *sql_builder_and_where_between(struct SqlBuilder *this_,
                                                 const void *field,
                                                 const void *min,
                                                 const void *max);

struct SqlBuilder *sql_builder_and_where_not_between(struct SqlBuilder *this_,
                                                     const void *field,
                                                     const void *min,
                                                     const void *max);

struct SqlBuilder *sql_builder_or_where(struct SqlBuilder *this_, const void *cond);

struct SqlBuilder *sql_builder_or_where_eq(struct SqlBuilder *this_,
                                           const void *field,
                                           const void *value);

struct SqlBuilder *sql_builder_or_where_ne(struct SqlBuilder *this_,
                                           const void *field,
                                           const void *value);

struct SqlBuilder *sql_builder_or_where_gt(struct SqlBuilder *this_,
                                           const void *field,
                                           const void *value);

struct SqlBuilder *sql_builder_or_where_ge(struct SqlBuilder *this_,
                                           const void *field,
                                           const void *value);

struct SqlBuilder *sql_builder_or_where_lt(struct SqlBuilder *this_,
                                           const void *field,
                                           const void *value);

struct SqlBuilder *sql_builder_or_where_le(struct SqlBuilder *this_,
                                           const void *field,
                                           const void *value);

struct SqlBuilder *sql_builder_or_where_like(struct SqlBuilder *this_,
                                             const void *field,
                                             const void *mask);

struct SqlBuilder *sql_builder_or_where_like_right(struct SqlBuilder *this_,
                                                   const void *field,
                                                   const void *mask);

struct SqlBuilder *sql_builder_or_where_like_left(struct SqlBuilder *this_,
                                                  const void *field,
                                                  const void *mask);

struct SqlBuilder *sql_builder_or_where_like_any(struct SqlBuilder *this_,
                                                 const void *field,
                                                 const void *mask);

struct SqlBuilder *sql_builder_or_where_not_like(struct SqlBuilder *this_,
                                                 const void *field,
                                                 const void *mask);

struct SqlBuilder *sql_builder_or_where_not_like_right(struct SqlBuilder *this_,
                                                       const void *field,
                                                       const void *mask);

struct SqlBuilder *sql_builder_or_where_not_like_left(struct SqlBuilder *this_,
                                                      const void *field,
                                                      const void *mask);

struct SqlBuilder *sql_builder_or_where_not_like_any(struct SqlBuilder *this_,
                                                     const void *field,
                                                     const void *mask);

struct SqlBuilder *sql_builder_or_where_is_null(struct SqlBuilder *this_,
                                                const void *field);

struct SqlBuilder *sql_builder_or_where_is_not_null(struct SqlBuilder *this_,
                                                    const void *field);

struct SqlBuilder *sql_builder_or_where_in(struct SqlBuilder *this_,
                                           const void *field,
                                           const void *const *list,
                                           uintptr_t length);

struct SqlBuilder *sql_builder_or_where_in_quoted(struct SqlBuilder *this_,
                                                  const void *field,
                                                  const void *const *list,
                                                  uintptr_t length);

struct SqlBuilder *sql_builder_or_where_not_in(struct SqlBuilder *this_,
                                               const void *field,
                                               const void *const *list,
                                               uintptr_t length);

struct SqlBuilder *sql_builder_or_where_not_in_quoted(struct SqlBuilder *this_,
                                                      const void *field,
                                                      const void *const *list,
                                                      uintptr_t length);

struct SqlBuilder *sql_builder_or_where_in_query(struct SqlBuilder *this_,
                                                 const void *field,
                                                 const void *query);

struct SqlBuilder *sql_builder_or_where_not_in_query(struct SqlBuilder *this_,
                                                     const void *field,
                                                     const void *query);

struct SqlBuilder *sql_builder_or_where_between(struct SqlBuilder *this_,
                                                const void *field,
                                                const void *min,
                                                const void *max);

struct SqlBuilder *sql_builder_or_where_not_between(struct SqlBuilder *this_,
                                                    const void *field,
                                                    const void *min,
                                                    const void *max);

struct SqlBuilder *sql_builder_union(struct SqlBuilder *this_, const void *query);

struct SqlBuilder *sql_builder_union_all(struct SqlBuilder *this_, const void *query);

struct SqlBuilder *sql_builder_order_by(struct SqlBuilder *this_,
                                        const void *field,
                                        bool desc);

struct SqlBuilder *sql_builder_order_asc(struct SqlBuilder *this_, const void *field);

struct SqlBuilder *sql_builder_order_desc(struct SqlBuilder *this_, const void *field);

struct SqlBuilder *sql_builder_limit(struct SqlBuilder *this_, const void *limit);

struct SqlBuilder *sql_builder_offset(struct SqlBuilder *this_, const void *offset);

const uint8_t *sql_builder_sql(struct SqlBuilder *this_);

const uint8_t *sql_builder_subquery(struct SqlBuilder *this_);

const uint8_t *sql_builder_subquery_as(struct SqlBuilder *this_, const void *name);

const uint8_t *sql_builder_query(struct SqlBuilder *this_);

const uint8_t *sql_builder_query_values(struct SqlBuilder *this_);
