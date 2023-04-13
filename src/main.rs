#![allow(warnings)] 
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::future::Pending;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use chrono::Local;
// use alloc::task;
use prefstore::{savepreference, ems, getcustom, savecustom, appendcustom};
use serde_json::Value;
// use regex::Regex;
// use tera::{Tera, Context};
use tiny_http::{Server, Response, Header, Request, Method, StatusCode};
use url::form_urlencoded;

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
fn savelink(linkurl:&str,linktitle:&str,url:&str,addr:&str)->String{
    // let ls=getcustom("todo","events.log","");
    // let tosave=(ls.toi32()+1);
    let date = Local::now();
    let current_date = date.format("%Y-%m-%d %H:%M:%S").to_string();
    let log=addr.to_string()+"--"
    +current_date.as_str()+": "
    +url+"---\t\t"
    +linktitle+"---"
    +linkurl+"\n";
   log
    
}
fn handle_client(mut request:Request)->Result<(),()> {
    
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
            let title = v["title"].as_str().unwrap();
            println!("addnote body: {:?}",v);
            // println!("addnote body: {:?}",url2str(body));
            // let mut ret:Vec<String>=vec![];
            // ret=serde_json::from_str(body).unwrap();
            prefstore::appendcustom(
                APPNAME,
                 "links.txt", 
                 savelink(
                    url,
                    title,
                    request.url(),
                    request.remote_addr().unwrap().ip().to_string().as_str(),
                )
                );
            // redirect(request,"/")?;
            request.respond(Response::from_string("OK").with_status_code(StatusCode(200))).map_err(|err|{
                eprintln!("could not serve request error {}",err);
            })?;
            
        }
    }
    
    Ok(())
}

fn main() ->Result<(),()>{
    let address="127.0.0.1:8080".to_string();
    
    let server=Server::http(&address).map_err(|err|{
        eprintln!("{err}")
    })?;
    println!("listening @ {}",address);
    for request in server.incoming_requests(){
        handle_client(request);
    }
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