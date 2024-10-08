use rem_borrower::borrow;
use std::time::Instant;
use log::{
    info,
    error
};

/// Only takes a file path as the non_local_controller has already setup the
/// final output filepath.
pub fn borrow(
    file_path:       &str,
    new_file_path:   &str,
    callee_fn_name:  &str,
    caller_fn_name:  &str,
    borrower_values: &(String, String),
) -> bool {
    let pre_extract_file_name: &str = &borrower_values.0;
    let mut_method_call_expr_file: &str = &borrower_values.1;

    let begin: Instant = Instant::now();

    // Run the borrower
    let success: bool = borrow::make_borrows(
        file_path,
        new_file_path,
        mut_method_call_expr_file,
        callee_fn_name,
        caller_fn_name,
        pre_extract_file_name
    );

    // Handle a failure
    if !success {
        error!("Bad exit value, file will be restored");
    }

    let success_string: &str = if success { "was successful " } else { "failed" };

    info!("Borrower {}, elapsed time in milliseconds: {:?}",
        success_string,
        begin.elapsed().as_millis()
    );

    success

}