use prefstore::getcustom;
// Import the tiny_http and std::fs modules
use tiny_http::{Server, Response, Header, Request};
use std::{fs, io::Cursor, path::Path};

// Define a function that returns a HTML string with the list of files in a given directory
fn list_files(dir: &str) -> String {
    let path=Path::new(dir);
    if(path.is_file()){
        return r#"
                <html>
                <body>
                Don't have permission right now contact@.
                </body>
                </html>
                "#
                .to_string();
    }
    // Get the entries of the directory
    let entries = fs::read_dir(dir).expect("Could not read directory");
    
    // Create an empty HTML string
    let mut html = String::new();
    // Add a header with the directory name
    html.push_str(&format!("<h1>Files in <a href=\"{}\">{}</a></h1>", path.parent().unwrap().to_string_lossy().to_string(),path.parent().unwrap().to_string_lossy().to_string()));
    // Add an unordered list element
    html.push_str("<ul>");
    // Iterate over the entries and add a list item for each one
    for entry in entries {
        // Get the file name as a string
        let file_name = entry.expect("Could not read entry").file_name().into_string().expect("Invalid file name");
        let fileurl=format!("{}\\{}",dir,file_name);
        // Add a list item with a link to the file
        html.push_str(&format!("<li><a href=\"{}\">{}</a></li>", fileurl, file_name));
    }
    // Close the unordered list element
    html.push_str("</ul>");
    // Return the HTML string
    html
}

// Define a function that returns a HTML string with the contents of a given file
fn read_file(file: &str) -> String {
    // Read the file contents as bytes
    let contents = fs::read(file).expect("Could not read file");
    // Convert the bytes to a UTF-8 string
    let contents = String::from_utf8(contents).expect("Invalid UTF-8");
    // Create an empty HTML string
    let mut html = String::new();
    // Add a header with the file name
    html.push_str(&format!("<h1>Contents of {}</h1>", file));
    // Add a preformatted element with the file contents
    html.push_str(&format!("<pre>{}</pre>", contents));
    // Return the HTML string
    html
}
const APPNAME:&str="LogLinktoDisk";

// Define a function that takes a path to a markdown file and returns its html content
fn render_markdown_file(filename: &str) -> String {
    
    // Create a string to hold the file contents
    let mut contents = 
    if(!filename.is_empty()){

        getcustom(&APPNAME.to_string(),filename,"")
    }
    else{
        String::new()
    };
    if(!contents.is_empty()){

        // Convert the markdown string to html using the markdown crate
        markdown::to_html_with_options(
            &contents ,
            &markdown::Options::gfm()
        ).unwrap()
    }
    else{
        "No saved sessions found".to_string()
    }
}
// Define a function that returns a response based on the request path
fn handle_request(path: &str) -> Response<Cursor<Vec<u8>>> {
    let content_header=Header::from_bytes(
        "Content-Type"
        ,"text/html; charset=utf-8"
    )
    .expect("valid or not");
    // let response = 
    //         Response::from_string(
    //             list_files(".")
    //             )
    //             .with_header(content_header);
    // // Check if the path is empty or "/"
    if path.is_empty() || path == "/" 
    {
        // Return a response with the list of files in the current directory
        println!("{}",path);
        Response::from_string(list_files(prefstore::prefstore_directory(&APPNAME.to_string()).unwrap().to_str().unwrap())).with_header(content_header)
    } else if path.contains(".md")
    {
        // println!("{}",path);
        // Response::from_string(list_files(path)).with_header(content_header)
        let responsestr=render_markdown_file(
            path);
        // let response = 
        Response::from_string(
            responsestr
            )
            .with_header(content_header)
    } else if path.ends_with("last")
    {
        // println!("{}",path);
        // Response::from_string(list_files(path)).with_header(content_header)
        let responsestr=render_markdown_file(
            &prefstore::get_last_from_buffer(APPNAME, ".l5"));
        // let response = 
        Response::from_string(
            responsestr
            )
            .with_header(content_header)
    }
    // else if path.contains(".")
    // {
    //     println!("{}",path);
    //     Response::from_string(
    //         r#"
    //         <html>
    //         <body>
    //         OK
    //         </body>
    //         </html>
    //         "#
    //     ).with_header(content_header)
    // }
    else
    {
        // Return a response with the contents of the file specified by the path
        println!("{}",path);
        Response::from_string(list_files(path)).with_header(content_header)
    }
    // ;
    // request.respond(response).expect("Could not send response");
    // Ok(())
}
// Define a function that takes a path to a markdown file and serves it as html using the tiny_http crate
pub fn serve(serverurl: &str) -> Result<(), ()> {
    // Create a server on port 8000
    let server = Server::http(serverurl).unwrap();
    // Loop over incoming requests
    for request in server.incoming_requests() {
        // Print some information about the request
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );
        let content_header=Header::from_bytes(
            "Content-Type"
            ,"text/html; charset=utf-8"
        )
        .expect("valid or not");
        // Create a response with the html content and a content type header
        let path = request.url();
        // Print the request path for debugging purposes
        println!("Request: {}", path);
        // Get a response based on the request path
        let response = handle_request(path);
        // Send the response to the client
        request.respond(response).expect("Could not send response");
    }
    // Return Ok if no errors occurred
    Ok(())
}
// Define the main function
#[test]
fn serveold() {
    // Create a server on port 8000
    let server = Server::http("0.0.0.0:6996").expect("Could not create server");
    // Print a message to indicate that the server is running
    println!("Server running on http://localhost:6996");
    // Loop forever and handle incoming requests
    loop {
        // Get a request from the server
        let request = server.recv().expect("Could not receive request");
        // Get the request path as a string
        let path = request.url();
        // Print the request path for debugging purposes
        println!("Request: {}", path);
        // Get a response based on the request path
        let response = handle_request(path);
        // Send the response to the client
        request.respond(response).expect("Could not send response");
    }
}