#![allow(warnings)] 
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::{fmt, thread};
use std::fs::File;
use std::future::Pending;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use chrono::Local;
mod fsovernet;
// use alloc::task;
use prefstore::{savepreference, ems, getcustom, savecustom, appendcustom, prefstore_directory};
use regex::Regex;
use serde_json::Value;
// use regex::Regex;
// use tera::{Tera, Context};
use tiny_http::{Server, Response, Header, Request, Method, StatusCode};
use url::form_urlencoded;
mod perlink;
const APPNAME:&str="LogLinktoDisk";

fn redirect(request:Request, path:&str)->Result<(),()>{
    let url=request.url().to_string();
    // let content_header=Header::from_bytes("Location",path)
    //     .expect("valid or not");
    
    let response= Response::empty(302)
                .with_data("".as_bytes(), Some(0))
                .with_header(Header::from_bytes("Location", path.as_bytes()).unwrap());
        request.respond(response).map_err(|err|{
            print!("couldn't respond to a req from {} error:{}",url,err)
        })?;
        Ok(())
}

fn url2str(input:&str)->Vec<String>{
    form_urlencoded::parse(input.as_bytes())
        .map(|(_, v)| {
            // todo!();
            
            format!("{}", v)
        })
        .collect::<Vec<_>>()
}
fn logevent(url:&str,addr:&str,method:&str){
    // let ls=getcustom("todo","events.log","");
    // let tosave=(ls.toi32()+1);
    let date = Local::now();
    let current_date = date.format("%Y-%m-%d %H:%M:%S").to_string();
    let log=addr.to_string()+"--"+&method.to_string() +"--"+current_date.as_str()+": "+url+"\n";
    appendcustom(APPNAME,"events.log",log);
    
}
fn remove_markdown_tags(str: &str) -> String {
    // Define a regular expression that matches common markdown tags
    // such as #, *, _, ~, [, ], (, ), <, >, etc.
    // Note: this is not a comprehensive regex for all markdown syntax
    let regex = Regex::new(r#"[#*_~\[\]()<>`\\\-]+"#).unwrap();
    // Replace the matched tags with an empty string
    let result = regex.replace_all(str, "");
   let result=result.replace("|"," ");
    println!("{}",result);

    // Return the result as a String
    result
}
fn savelink(linkurl:&str,linktitle:&str,url:&str,addr:&str,foldername:&str)->String{
    // let ls=getcustom("todo","events.log","");
    // let tosave=(ls.toi32()+1);
    let date = Local::now();
    let current_date = date.format("%Y-%m-%d %H:%M:%S").to_string();
    let start="| ".to_string();
    prefstore::initcustomfile(
        APPNAME,
         foldername,
        "|Title|URL|\n|-----|----|\n"
        );
    let log=
    start
    +&remove_markdown_tags(linktitle)+" | "
    +linkurl+" |\n";
   log
    
}
fn handle_client(mut request:Request,state:Arc<Mutex<String>>)->Result<(),()> {
    
    logevent(request.url(),
    request.remote_addr().unwrap().ip().to_string().as_str(),
    request.method().to_string().as_str());
    match(request.method(),request.url()){
        // (Method::Post,"/")=>{
        _=>{
            
            let mut buf=Vec::new();
            request.as_reader().read_to_end(&mut buf);
            let body= std::str::from_utf8(&buf).map_err(|err|{
                eprintln!("error: couldn't interpret body as UTF-8:{err}")
            })?;
            
            // println!("addnote body: {:?}",body);
            let v:Value=serde_json::from_str(&body).unwrap();
            let url = v["url"].as_str().unwrap();
            let i=url.clone().to_string();
            // let i=url.clone();
            let title = v["title"].as_str().unwrap();
            let foldername = v["folder"].as_str().unwrap();
            println!("addnote body: {:?}",v.clone());
            let mut state = state.lock().unwrap();
            *state=url.to_string();
            // thread::spawn(move||{
                
        //     // serve_markdown_file(&servemd);
            //     fsovernet::serve(&servemd);
                //    perlink::samplewindow();
                // perlink::showui(i);
            // });
            // let savelocation=if foldername.is_empty(){
            //     format!("links.md")
            // }else{
                
            //     format!("{}/links.md",foldername)
            // };
            
            // // println!("addnote body: {:?}",url2str(body));
            // // let mut ret:Vec<String>=vec![];
            // // ret=serde_json::from_str(body).unwrap();
            // prefstore::appendcustom(
            //     APPNAME,
            //      savelocation.clone(), 
            //      savelink(
            //         url,
            //         title,
            //         request.url(),
            //         request.remote_addr().unwrap().ip().to_string().as_str(),
            //         savelocation.clone().as_str()
            //     )
            // );
            // prefstore::savebuffer(APPNAME
            //     , ".l5"
            //     , savelocation.clone()
            //     , 5);
            // println!("{}",prefstore::get_last_from_buffer(APPNAME,".l5"));
            let h="Ok";
            // drop(request);
            // redirect(request,"/")?;
            request.respond(
                Response::from_string(
                    serde_json::to_string(&h).unwrap()
                ).
                with_status_code(StatusCode(200))).
                map_err(
                    |err|{
                        eprintln!("could not serve request error {}",err);
                    }
                )?;
            
        }
    }
    
    Ok(())
}


// // Define a function that takes a path to a markdown file and serves it as html using the tiny_http crate
// fn serve_markdown_file(serverurl: &str) -> Result<(), ()> {
//     // Create a server on port 8000
//     let server = Server::http(serverurl).unwrap();
//     // Loop over incoming requests
//     for request in server.incoming_requests() {
//         // Print some information about the request
//         println!("received request! method: {:?}, url: {:?}, headers: {:?}",
//             request.method(),
//             request.url(),
//             request.headers()
//         );
//         let content_header=Header::from_bytes(
//             "Content-Type"
//             ,"text/html; charset=utf-8"
//         )
//         .expect("valid or not");
//         // Create a response with the html content and a content type header
//         let responsestr=render_markdown_file(
//             &prefstore::get_last_from_buffer(APPNAME, ".l5"));
//         let response = 
//         Response::from_string(
//             responsestr
//             )
//             .with_header(content_header);
//         // Send the response to the client
//         request.respond(response).unwrap();
//     }
//     // Return Ok if no errors occurred
//     Ok(())
// }

// Call the function with a sample markdown file path

fn main() ->Result<(),()>{
    let address="127.0.0.1".to_string();
    let port="8080".to_string();
    let serveurl=format!("{}:{}",address,port);
    let servemd=format!("{}:{}",address,"7890");
    let state = Arc::new(Mutex::new(String::new()));

    // spawn a thread to handle incoming requests
    let state_clone = state.clone();
    thread::spawn(move||{
    //     // serve_markdown_file(&servemd);
    //     fsovernet::serve(&servemd);
        let server=Server::http(&serveurl).map_err(|err|{
                eprintln!("{err}")
            }).unwrap();
        println!("listening @ {}",serveurl);
        for request in server.incoming_requests(){
                handle_client(request,state.clone());
            }
        }
    );
    perlink::showui(state_clone);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_client() {
    // Your test code goes here
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let request = "POST / HTTP/1.1\r\nContent-Type: application/x-www-form-urlencoded\r\n\r\nname=John&age=42";
    stream.write(request.as_bytes()).unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..]);
    assert!(response.contains("Received name: Some(\"John\"), age: Some(42)"));
    }
}

// #[cfg(test)]
// mod benchmarks {
//     use super::*;
//     use test::Bencher;

//     #[bench]
//     fn bench_handle_client(b: &mut Bencher) {
//         let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
//         let request = "POST / HTTP/1.1\r\nContent-Type: application/x-www-form-urlencoded\r\n\r\nname=John&age=42";
//         b.iter(|| {
//             stream.write(request.as_bytes()).unwrap();
//             let mut buffer = [0; 1024];
//             stream.read(&mut buffer).unwrap();
//             let response = String::from_utf8_lossy(&buffer[..]);
//             assert!(response.contains("Received name: Some(\"John\"), age: Some(42)"));
//         });
//     }
// }