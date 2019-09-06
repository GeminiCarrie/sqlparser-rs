// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;

extern crate sqlparser;
extern crate log;
mod sqlparser_ansi;
mod sqlparser_generic;
mod sqlparser_postgres;


#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);
    rsgx_unit_tests!(
sqlparser_ansi::parse_simple_select,
sqlparser_postgres::parse_alter_table_constraint_foreign_key,
sqlparser_postgres::parse_alter_table_constraint_primary_key,
sqlparser_postgres::parse_common_insert,
sqlparser_postgres::parse_complex_insert,
sqlparser_postgres::parse_copy_example,
sqlparser_postgres::parse_create_table_from_pg_dump,
sqlparser_postgres::parse_create_table_with_defaults,
sqlparser_postgres::parse_create_table_with_inherit,
sqlparser_postgres::parse_example_value,
sqlparser_postgres::parse_function_now,
sqlparser_postgres::parse_insert_invalid,
sqlparser_postgres::parse_insert_with_columns,
sqlparser_postgres::parse_invalid_table_name,
sqlparser_postgres::parse_simple_insert,
sqlparser_postgres::parse_timestamps_example,
sqlparser_postgres::parse_timestamps_with_millis_example,
sqlparser_postgres::test_prev_index,
sqlparser_generic::parse_aggregate_with_group_by,
sqlparser_generic::parse_case_expression,
sqlparser_generic::parse_cast,
sqlparser_generic::parse_complex_join,
sqlparser_generic::parse_compound_expr_1,
sqlparser_generic::parse_compound_expr_2,
sqlparser_generic::parse_create_table,
sqlparser_generic::parse_cross_join,
sqlparser_generic::parse_delete_statement,
sqlparser_generic::parse_delete_with_semi_colon,
sqlparser_generic::parse_implicit_join,
sqlparser_generic::parse_is_not_null,
sqlparser_generic::parse_joins_on,
sqlparser_generic::parse_joins_using,
sqlparser_generic::parse_joins_using,
sqlparser_generic::parse_like,
sqlparser_generic::parse_limit_accepts_all,
sqlparser_generic::parse_literal_string,
sqlparser_generic::parse_not,
sqlparser_generic::parse_not_like,
sqlparser_generic::parse_parens,
sqlparser_generic::parse_projection_nested_type,
sqlparser_generic::parse_scalar_function_in_projection,
sqlparser_generic::parse_select_count_wildcard,
sqlparser_generic::parse_select_group_by,
sqlparser_generic::parse_select_order_by,
sqlparser_generic::parse_select_order_by_limit,
sqlparser_generic::parse_select_string_predicate,
sqlparser_generic::parse_select_version,
sqlparser_generic::parse_select_wildcard,
sqlparser_generic::parse_simple_math_expr_minus,
sqlparser_generic::parse_simple_math_expr_plus,
sqlparser_generic::parse_simple_select,
sqlparser_generic::parse_where_delete_statement,
                     );

    sgx_status_t::SGX_SUCCESS
}
