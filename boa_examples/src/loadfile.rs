//! This example shows how to load a JavaScript file and execute it

use boa::Context;
use std::fs::read_to_string;

pub fn main() {
    let js_file_path = "./scripts/helloworld.js";

    match read_to_string(js_file_path) {
        Ok(src) => {
            // Instantiate the execution context
            let mut context = Context::default();

            // Parse the source code
            let code_block = match context
                .parse(src)
                .map(|statement_list| context.compile(&statement_list))
            {
                Ok(res) => res,
                Err(e) => {
                    // Pretty print the error
                    eprintln!(
                        "Uncaught {}",
                        context
                            .throw_syntax_error::<_, ()>(e.to_string())
                            .expect_err("interpreter.throw_syntax_error() did not return an error")
                            .display()
                    );

                    return;
                }
            };

            // Execute the JS code read from the source file
            match context.execute(code_block) {
                Ok(v) => println!("{}", v.display()),
                Err(e) => eprintln!("Uncaught {}", e.display()),
            }
        }
        Err(msg) => eprintln!("Error: {}", msg),
    }
}
