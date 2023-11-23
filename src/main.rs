/* use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: &'static str,
}

#[get("/message")]
async fn get_message() -> impl Responder {
    let message = Message {
        message: "Hello from Rust!",
    };

    web::Json(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_message)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
 */

use std::os::raw::c_int;
use std::slice;

// This attribute prevents name mangling, ensuring that the function can be called from C
#[no_mangle]
// This annotation specifies the C-compatible function signature
pub extern "C" fn sort_numbers(numbers: *const c_int, len: usize) -> Vec<c_int> {
    // Unsafe block allows for low-level operations
    unsafe {
        // Convert the raw pointer and length to a slice
        let numbers_slice = slice::from_raw_parts(numbers, len);
        // Create a mutable copy of the slice
        let mut sorted_numbers = numbers_slice.to_vec();
        // Sort the mutable vector
        sorted_numbers.sort();
        // Return the sorted vector
        sorted_numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test function to check if the sort_numbers function produces the correct result
    #[test]
    fn test_sort_numbers() {
        // Input vector
        let numbers = vec![5, 2, 8, 1, 7];
        // Call the sort_numbers function
        let sorted_numbers = sort_numbers(numbers.as_ptr(), numbers.len());
        // Assert that the result is correct
        assert_eq!(sorted_numbers, vec![1, 2, 5, 7, 8]);
    }
}
