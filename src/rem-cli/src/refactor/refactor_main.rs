use log::{
    info,
    error
};

use rem_repairer::common::RepairSystem;

use crate::refactor::{
    borrow::borrow,
    non_local_controller::non_local_controller,
    repair_lifetime::repair_lifetime,
};

/// Calls out to rem-controller, then rem-borrower, then rem-repairer to fix up
/// the extracted method.
///
/// # Args
/// * `file_path` - The path to the original file. Must contain a new method with the signature extracted and the original code inside it
/// * `new_file_path` - The path to the new file. If it is the same as the original file, then we will overwrite the existing file.
/// * `calle_fn_name` - The function that has been extracted
/// * `caller_fn_name` - The function that contains the call to calle_fn_name
/// * `opt` - The program option that has been set (which part of the program to run)
///
/// # Returns
/// * bool - True if extraction was successful.
pub fn extract_function(
    file_path:       &str,
    new_file_path:   &str,
    callee_fn_name:  &str,
    caller_fn_name:  &str,

) -> bool {
    // Change this to return a result. 
    return true;
}

pub fn extract_function_generic(
    file_path:       &str,
    new_file_path:   &str,
    callee_fn_name:  &str,
    caller_fn_name:  &str,
) -> bool {
    todo!()
}

pub fn extract_function_async(
    file_path:       &str,
    new_file_path:   &str,
    callee_fn_name:  &str,
    caller_fn_name:  &str,
) -> bool {
    todo!()
}