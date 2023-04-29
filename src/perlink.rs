#![windows_subsystem = "windows"]
#[allow(warnings)]
use std::{env,rc, process::{self, ExitCode}};
use window_titles::{Connection, ConnectionTrait};
use arboard::Clipboard;
use indexmap::{IndexMap};
extern crate linkify;

use linkify::{LinkFinder, LinkKind};
// use std::option::Option;
use fltk::{
    enums::{Color, FrameType, Event, CallbackTrigger},
    app::MouseButton,
    app::{App,*},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{TextBuffer, TextDisplay},
    window::Window,
    button::{Button,CheckButton},
   input::Input,
    prelude::*, frame::Frame,
};

use serde::{Deserialize, Serialize};
use std::{process::{Command,Stdio}, sync::{Arc, Mutex}};
// use execute::{Execute, command};

use isahc::prelude::*;
// extern crate preferences;
use std::collections::HashMap;
// use abserde::*;

use std::fs::create_dir_all;
// const APP_INFO: AppInfo = AppInfo{name: "Perlink", author: "visnk"};
const appname: &str = "savetodisk-perlink";
fn eurl(t: String) -> Result<String,()> {
    // return Ok("try".to_string());
    println!("get {} val----->{}","expanding",t);
    let mut response = isahc::get(
        format!("{}{}",prefstore::getcustom(appname, "website.su", "https://unshorten.me/s/".to_string()),t).as_str()
    ).map_err(|op|{
        eprintln!("Could not get expanded url. error:{}",op)
    }).unwrap();
    // println!("get {} val----->{}","expanded url",response.text()?);

    // Print some basic info about the response to standard output.
    // println!("Status: {}", response.status());
    // println!("Headers: {:#?}", response.headers());

    // Read the response body as text into a string and print it.
   
    return Ok(response.text().unwrap());
}
#[derive(Serialize, Deserialize, Default, Debug)]
struct MyConfig {
    shortenusing: String,
	// window_width: usize,
	// window_height: usize,
	// window_x: usize,
	// window_y: usize,
	// theme: String,
    #[serde(with = "indexmap::serde_seq")]
	user_data: IndexMap<String, String>,
}

// #[derive(Default)]
// struct vars{
//     jas: String,
// }

// const ChosenBrowser: &str = "dbrowser";
// const Notimes: &str = "ntimes";
// const Isenb: &str = "isenb";
// const PREFERENCES_KEY: &str = "prefs";
fn appendfile(browsername:String,browsercommand:String){
    prefstore::savepreference(appname, browsername,browsercommand);
    }
fn reinit(){
    // my_abserde.delete().expect("");    
    // let mut pref = IndexMap::<String,String>::new();
    let mut browsers;
    let browsers_names;
    // let mut browsers = ["V:\\Firefox\\firefox.exe","chromium","waterfox","vivaldi-stable","firefox-dev","firefox-beta"];
    #[cfg(not(target_os = "macos"))]{
        browsers = ["firefox","firefox","chromium","waterfox","vivaldi-stable","firefox-dev","firefox-beta"];
        browsers_names = ["firefox private window","firefox","chromium","waterfox","vivaldi stable","firefox dev","firefox beta"];
    }
    #[cfg(target_os = "macos")]{
        browsers = ["open -a Firefox --args --private-window","open -a Firefox --args","open -a Safari --args"];
        browsers_names = ["firefox private","firefox","safari"];
    }
    // #[cfg(not(target_os = "linux"))]{

    // }
    // setup();
    let mut i=0;
    for br in browsers{
        prefstore::savepreference(appname, br,browsers_names.get(i).unwrap().to_string());
        i+=1;
    }
    prefstore::savecustom(appname,"website.su", "https://unshorten.me/s/".to_string());
            
}

// }
pub fn link_finder_str(input: &str) -> Vec<String> {
    let mut links_str = Vec::new();
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let links: Vec<_> = finder.links(input).collect();

    for link in links.iter() {
        links_str.push(link.as_str().to_string());
    }
    links_str
}
pub fn showui(state:Arc<Mutex<String>>) {
    // app_center::start!("522f2740-e466-4804-9e8e-8d975869d4dd");
    human_panic::setup_panic!(human_panic::Metadata {
        version: env!("CARGO_PKG_VERSION").into(),
        name: env!("CARGO_PKG_NAME").into(),
        authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        path_to_save_log_to: prefstore::prefstore_directory(&appname.to_string()).unwrap(),
    });
    // let my_abserde = Abserde {
    //     app: appname.to_string(),
    //     location: Location::Auto,
    //     format: Format::Toml,
    // };
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        
        Some(val) => match val {
            val => {
                println!("{}----------->",val);

                if val == "reinit"{
                    println!("Reinitilizing config file.");
                    reinit();
                    process::exit(0);

                }if val == "add"{
                    println!("Added new browser.");
                    appendfile(args.get(2).unwrap().to_string(),args.get(3).unwrap().to_string());
                    process::exit(0);

                }
                if val == "clear"{
                    println!("Cleared browser list.");
                    prefstore::clearall(appname,"txt");
                    process::exit(0);

                }
            }
            _ =>{
                
            },
            // Message::Stop => rlist(),
        },
        None => {
            
        },
    }
    // Create a new preferences key-value map
    // (Under the hood: HashMap<String, String>)
    // let mut faves: PreferencesMap<String> = PreferencesMap::new();

    // Edit the preferences (std::collections::HashMap)
    
    // faves.insert("programming language".into(), "Rust".into());

    // Store the user's preferences
    // let prefs_key = "tests/docs/basic-example";
    // let save_result = faves.save(&APP_INFO, &PREFERENCES_KEY);
    // assert!(save_result.is_ok());

    // ... Then do some stuff ...

    // Retrieve the user's preferences
    // let mut load_result = get_token(Notimes.to_string());
    // let mut load_result = get_token(ChosenBrowser.to_string());
    // assert!(load_result.is_ok());
    // println!("{}",load_result.unwrap().to_string());
    // let dbrowser = format!("{:?}",load_result.unwrap());

    // let mut browserbs:PreferencesMap<String> = setup();
    // for  (k, v) in browserbs.iter(){
    //     println!("{:?}",k);
    //     println!("{:?}",v);
    // } 
    //PreferencesMap::new();
    // browserbs.clear();
    // browserbs.insert("browsername".into(), "commandtoopen".to_string());
    // browserbs.remove("browsername");
    // setup();
    // i=0;
    
    // let save_result = browserbs.save(&APP_INFO, PREFERENCES_KEY);

    // match PreferencesMap::<String>::load(&APP_INFO, &PREFERENCES_KEY) {
        
    //     Ok(map) => {
    //         for  (k, v) in map.iter(){
    //             println!("{:?}",k);
    //             println!("{:?}",v);
    //         } 
    // //         let bnc= map.clone();
    // //         let bcc= map.clone();
    // //         let bn: Vec<String> = bnc.into_values().collect();
    // //         let bc: Vec<String> = bcc.into_keys().collect();
    // //         let size=bn.length();
    // // let mut expandedurl = bn.get(1).unwrap().to_string() ;

    // //         // let mut k= map.clone();
    // //         // map.keys().
    // //         for i in map.keys(){
    // //             println!("{:?}",i);
    // //             // println!("{:?}",map.entry(String::from(i)));
    // //         }
    // //         for i in map.values(){
    // //             println!("{:?}",i);
    // //         }
          
    //     }
    //     Err(e) => {
    //       // warn!("Error while loading preferences: {:?}", e);
    //     //   None         
    //       println!("None");

    //     }
    //   }
    //   println!("config saved to {:?}", prefs_base_dir().unwrap());


    //add customise browsr option
    //add timed use of a particular browser
    
    let mut WIDGET_PADDING: i32 = 20;
    let mut WIDGET_WIDTH: i32 = 420;
    let mut WIDGET_HEIGHT: i32 = 400;  
    let args: Vec<String> = env::args().collect();
    let mut expandedurl = "".to_string();
    let mut ourl = "".to_string();
    // let mut sourl = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    // let mut sourl= vars{jas:"".to_string()};
    
    let mut strtoshow="";
    

    // let mut ourl = args.get(1).unwrap().to_string() ;
    // expandedurl = sk;
    // let (s, r) = fltk::app::channel();
            let mut app = App::default();
            
            let mut win = Window::default().with_size(WIDGET_WIDTH, WIDGET_HEIGHT).with_label("Choose browser");
            win.handle(move |f, ev|{
                // println!("{}----->{}",ev,fltk::app::event_text());
             match ev {
                // Event::Paste => {
                    
                //     true
                // }
                // fltk::enums::Event::Resize => {       
                //     println!("A resize happening: x:{}, y:{}, w:{}, h:{}", f.x(), f.y(), f.width(), f.height());
                //     true
                // }
                fltk::enums::Event::KeyDown => {
                     if fltk::app::event_key() == fltk::enums::Key::from_char('f') {
                        // win.fullscreen(!win.fullscreen_active());
                    } else if fltk::app::event_key() == fltk::enums::Key::from_char('q') {
                        fltk::app::quit();
                    };
        
                    true
                }
                ,
                 _ => {
                     false
                 }
             }
});
let (s, r) = fltk::app::channel();
//             win.handle(move |f, ev| {
//                 println!("{:?}",ev);
//                 println!("{:?}",event_text());
//                 match ev {
//                 fltk::enums::Event::Resize => {
                    
//                     true
//                 }
//                 fltk::enums::Event::KeyDown => {
//                      if fltk::app::event_key() == fltk::enums::Key::from_char('f') {
//                         // win.fullscreen(!win.fullscreen_active());
//                     } else if fltk::app::event_key() == fltk::enums::Key::from_char('q') {
//                         fltk::app::quit();
//                     };
        
//                     true
//                 }
//                 ,
//                  _ => {
//                      false
//                  }
//              }
// });

            // let mut text_buffer = TextBuffer::default();
            // text_buffer.set_text(&expandedurl);
           
            let mut vpack=fltk::group::Pack::new(WIDGET_PADDING,
                WIDGET_PADDING,
                WIDGET_WIDTH - 40,
                WIDGET_HEIGHT - 40,"");
                win.resizable(&vpack);
                // let mut url = Input::new(100,25,300,25, "Enter URL");
                // url.set_trigger(CallbackTrigger::Changed);
                // let mut uc=false;
                // url.set_callback(move |input_c: &mut Input| {
                //         ourl=input_c.value();
                //         println!("thevalis----->{}",ourl);
                //         uc=true;
                //     });
                    
                // url.emit(s.clone(),"frominput".to_string());
            // let mut tbpack=fltk::group::Pack::default().with_size(250,60).center_of(&win);    
                let mut framet = fltk::frame::Frame::default()
                .with_size(800,60)
                // .center_of(&win)
                .with_label("Loading");
                // let mut kz = Button::new(0,0,70,20,"test");
                //                     // .with_align(Align::Left | Align::Inside)
                //                     kz.emit(s.clone(),"b.label()");
                
            framet.set_label_size(12);
            // let mut rt= fltk::frame::Frame::default().with_size(20, 10);
            // rt.set_label("sd");
            // match args.get(1) {
            //     Some(val) => match val {
            //         val => {
            //             expandedurl=val.to_string();
            //             ourl=val.to_string();
            //             setframe(&mut framet,&val);
            //             // rt.set_label("");
            //         }
            //         _ =>{
            //             expandedurl=" ".to_string();
            //             ourl=" ".to_string();
            //             setframe(&mut framet,&"invalid url".to_string());
            //         },
            //         // Message::Stop => rlist(),
            //     },
            //     None => {
                    expandedurl=" ".to_string();
                    // let k=vars{jas:"".to_string()};
                    ourl=" ".to_string();
                    println!("here");
                    let connection = Connection::new().unwrap();
                    // let mut pref = HashMap::<String,String>::new();
                    // let mut lks = vec!["", "New York"];
                    // let mut links: Vec<_>=;
                    for i in connection.window_titles().unwrap(){
                        // println!("{}",i.to_lowercase());
                        for kj in link_finder_str(&i){
                            let ss: String = kj.chars().skip(0).take(40).collect();
                            let mut b = Button::default()
                                    .with_size(70, 20)
                                    .with_label(&ss)
                                    // .with_align(Align::Left | Align::Inside)
                                    ;
                                    b.set_tooltip(&kj);
                                    b.emit(s.clone(),kj);
                                b.set_down_frame(FrameType::FlatBox);
                                b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
                                b.clear_visible_focus();
                                // b.set_label_size(app::font_size() - 3);
                                // b.draw(move |b| {
                                //     if b.value() {
                                //         expandedurl=b.label();
                                //         ourl=kj.to_string();
                                //     }
                                // });
                                b.set_frame(FrameType::FlatBox);
            //                     b.handle(move|b, ev| match ev {
            //                             fltk::enums::Event::Push => {
            //                                 expandedurl=b.label();
            // //                 println!("{}",val);
            //                                 true
            //                             }
            //                             _ => false,
            //                         });
                                
                                
                                // b.set_trigger(CallbackTrigger::Changed);
                        //         b.set_callback({
                        //             frame=frame.clone();
                        //             // let mut expandedurl=expandedurl.clone();
                        //             // let mut ourl=ourl.clone();                                    
                        //             move |b| {
                        //                 frame.set_label(b.label());
                        //                 // expandedurl=b.label();
                        //                 // frame.set_label(&b.label());
                        //                 // sourl.replace(b.label());
                        //                 println!("{}",b.label());
                        //                 // sourl=vars{jas:b.label()};
                        //                 // ourl=b.label();
                        //         }
                        // });
                                
                            // println!("{}",kj);
                        }
                    }
                    let mut clipboard = Clipboard::new().unwrap();
                    match clipboard.get_text() {
                    Ok(sk) => { 
                        for kj in link_finder_str(&sk){
                            let ss: String = kj.chars().skip(0).take(40).collect();
                            let mut b = Button::default()
                                .with_size(70, 20)
                                .with_label(&ss);
                            b.emit(s.clone(),kj.to_string());
                            b.set_tooltip(&kj);
                            b.set_down_frame(FrameType::FlatBox);
                            b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
                            b.clear_visible_focus();
                            b.set_frame(FrameType::FlatBox);
                        println!("{}",kj);
                        }
                        
                        // fltk::dialog::message(90, 90, &sk);{
                            // let mut res = std::process::Command::new(format!("/home/roger/Downloads/waterfox/waterfox {}",sk)).output();
                        // }
                        
                        // ... use sk ...
                    },
                    Err(e) => {
                        println!("Error Clipboard");
                        // setframe(&mut framet,"Error");
                        // ... sk is not available, and e explains why ...
                    },
                // }
                                       
                // }
                    
            // ,
            }
            
            println!("{}",ourl);
            // let mut bframe = fltk::frame::Frame::default()
            //     .with_size(20, 10);
            
            // let mut bframee = fltk::frame::Frame::default().with_size(200, 60);
            
                    
                    // tbpack.end();
                    // tbpack.set_type(fltk::group::PackType::Horizontal);
                fltk::frame::Frame::default().with_size(20, 10);
            //-------------------------------------------From here------------------------------------
            //-------------------------------------------From here------------------------------------
            //-------------------------------------------From here------------------------------------
            //-------------------------------------------From here------------------------------------
            //-------------------------------------------From here------------------------------------
            //-------------------------------------------From here------------------------------------
            //-------------------------------------------From here------------------------------------
            //-------------------------------------------From here------------------------------------
            //-------------------------------------------From here------------------------------------
            //-------------------------------------------From here------------------------------------
            // let mut fnt=fltk::group::Pack::default()
            //     .with_size(250,30);
            // let mut cb1 = CheckButton::default().with_size(220,30);
            // cb1.set_label("Use the same browser for next");
            // if(get_token(Isenb.to_string()).unwrap().contains("true")){
            //     cb1.set_checked(true);
            // }
            // else{
            //     cb1.set_checked(false);
            // }
            // // cb1.set_checked();
            // //-----------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------
            // cb1.handle(move|b, ev| match ev {
            //     fltk::enums::Event::Push => {
            //         // if(cb1.is_checked()){
            //         //     set_token(Notimes.to_string(),format!("{:?}",i1.value().to_string()));
            //         // }
                    
            //         // println!("{}",format!("{} {}",browser,expandedurl));
            //         if(!b.is_checked()){
            //         set_token(Isenb.to_string(),"start".to_string());
            //         }
            //         else{
            //             set_token(Isenb.to_string(),"stop".to_string());
            //         }
                    

            //         // let mut res = Command::new(format!("{}",browser))
            //         // .arg(format!("{}",expandedurl))
            //         // .output();
            //         true
            //     }
            //     _ => false,
            // }); 
            
            // let mut i1 = Input::default().with_size(30,30);
            // i1.set_value("10");
            // i1.set_trigger(CallbackTrigger::Changed);
            // i1.set_callback(move |input_c: &mut Input| {
            //     let cbx=cb1.clone();
            //     let name = input_c.value();
            //     // let mut lbl = label_c.lock();
            //     if(cbx.is_checked()){
            //         set_token(Notimes.to_string(),input_c.value());
            //     }
            //     else{
            //         set_token(Notimes.to_string(),"0".to_string());
            //     }
            // });
            // //-----------------------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------------------
            // //-----------------------------------------------------------------------------------------------------
           
            
            // // cb1.emit(s,i1.value());
            // // while app.wait() {
            // //     match r.recv() {
            // //         Some(val) => match val {
            // //             val => {
            // //                 if(cb1.is_checked()){
            // //                     println!("setval---->{}",val);
            // //                     set_token(Notimes.to_string(),format!("{}",val));
            // //                 }
            // //                 else{
            // //                     set_token(Notimes.to_string(),"0".to_string());
            // //                 }
            // //             },
            // //             // Message::Stop => rlist(),
            // //         },
            // //         None => (),
            // //     }
            // // }
        
            
            
            // // i1.set_trigger(CallbackTrigger::Changed);
            // fnt.end();
            // fnt.set_type(fltk::group::PackType::Horizontal);

//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------
//-----------------------------Till here for the checkbutton---------------------------------------

                    // b1.emit(s, "refresh".to_string());
                    // let mut hpack=hpack.clone();
                    // b1.handle(move|b, ev| match ev {
                    //     fltk::enums::Event::Push => {
                    //         // println!("{}",format!("{} {}",browser,expandedurl));
                    //         set_token(browser.to_string());
                    //         let mut res = Command::new(format!("{}",browser))
                    //         .arg(format!("{}",expandedurl))
                    //         .output();
                    //         true
                    //     }
                    //     _ => false,
                    // });
            // disp.set_buffer(text_buffer);
            let mut ttb=fltk::group::Pack::default().with_size(
                10,
                40) ;
                fltk::frame::Frame::default().with_size(20, 30);
            
            let mut eub = Button::default().with_size(120,30);
            eub.set_label("expand url");
            eub.emit(s.clone(),"expandurl".to_string());
            // b1.emit(s, "refresh".to_string());
            // let mut hpack=hpack.clone();
            // eub.handle(move|b, ev| match ev {
            //     fltk::enums::Event::Push => {
            //         // frame.set_label(&"Expanding URL, Please wait!".to_string());
            //         match eurl(args.get(1).unwrap().to_string()) {
            //             Ok(sk) => { 
            //                 if(sk.to_lowercase().contains("invalid")){
            //                     setframe(&mut framet,args.get(1).unwrap());
            //                     // rt.set_label("");
            //                 }
            //                 else{
            //                     setframe(&mut framet, &sk);
            //                 }
                            
            //                 // fltk::dialog::message(90, 90, &sk);{
            //                     // let mut res = std::process::Command::new(format!("/home/roger/Downloads/waterfox/waterfox {}",sk)).output();
            //                 // }
                            
            //                 // ... use sk ...
            //             },
            //             Err(e) => {
            //                 setframe(&mut framet,"Error");
            //                 // ... sk is not available, and e explains why ...
            //             },
            //         }
            //         true
            //     }
            //     _ => false,
            // });
            fltk::frame::Frame::default().with_size(10, 10);
            let mut but = Button::default().with_size( 70, 30);
            but.set_label("Refresh");
            but.emit(s.clone(),"refresh".to_string());
            // fltk::frame::Frame::default().with_size(10, 10);
            fltk::frame::Frame::default().with_size(10, 10);
            // let mut bframe1 = fltk::frame::Frame::default().with_size(300, 60);
            let mut b11 = Button::default().with_size(120,30);
            b11.set_label("All browsers");
            // b1.emit(s, "refresh".to_string());
            // let mut hpack=hpack.clone();
            b11.emit(s.clone(),"all".to_string());
            // b11.handle(move|b, ev| match ev {
            //     fltk::enums::Event::Push => {
            //         // i1.value();
            //         // if(cb1.is_checked()){
            //         //     set_token(Notimes.to_string(),format!("{:?}",i1.value().to_string()));
            //         // }
                    
            //         // println!("{}",format!("{} {}",browser,expandedurl));
            //         for (k,v) in setup(){
            //             let mut res = Command::new(format!("{}",v))
            //                             .arg(format!("{}",ourl))
            //                             .output();
            //         }
            //         // set_token(ChosenBrowser.to_string(),browser.to_string());
                    
            //         fltk::app::quit();
            //         true
            //     }
            //     _ => false,
            // });

            ttb.end();
            ttb.set_type(fltk::group::PackType::Horizontal);
            
            fltk::frame::Frame::default().with_size(10, 10);
            let mut ttb=fltk::group::Pack::default().with_size(
                10,
                40) ;
                
                
                fltk::frame::Frame::default().with_size(20, 30);
            
                let mut svw = Button::default().with_size(150,30);
                svw.set_label("share via web");
                svw.emit(s.clone(),"svw".to_string());
                
            // b1.emit(s, "refresh".to_string());
            // let mut hpack=hpack.clone();
            // eub.handle(move|b, ev| match ev {
            //     fltk::enums::Event::Push => {
            //         // frame.set_label(&"Expanding URL, Please wait!".to_string());
            //         true
            //     },
            //     _ => false,
            // });
            fltk::frame::Frame::default().with_size(20, 10);
            
            // let mut bframe1 = fltk::frame::Frame::default().with_size(300, 60);
            let mut svc = Button::default().with_size(150,30);
                svc.set_label("copy to clipboard");
                svc.emit(s.clone(),"svc".to_string());
    

            ttb.end();
            ttb.set_type(fltk::group::PackType::Horizontal);
            fltk::frame::Frame::default().with_size(20, 30);
            let mut hpack=fltk::group::Pack::default().with_size(250,40) .center_of(&win);
                // let i=0;

                // browsers=browsers.clone();
                let mut i=0;
                // let mut bl:PreferencesMap<String> = setup();
                if(prefstore::getall(appname).is_empty()){
                    reinit();
                }
                
                for (k,v) in prefstore::getall(appname) {
                    let expandedurl=expandedurl.clone();
                    fltk::frame::Frame::default().with_size(20, 10);
                    let k: String = k.chars().skip(0).take(10).collect();
                    // let cc = k.chars().count();
                    // let sz=cc*9;
                    // let mut b1 = Button::default().with_size(sz.try_into().unwrap(),60);
                    let mut b1 = Button::default().with_size(90,60);
                    
                    b1.set_label(&format!("{}",k));
                    b1.emit(s.clone(),v);
                    // b1.emit(s, "refresh".to_string());
                    // let mut hpack=hpack.clone();
                    // b1.handle(move|b, ev| match ev {
                    //     fltk::enums::Event::Push => {
                    //         // i1.value();
                    //         // if(cb1.is_checked()){
                    //         //     set_token(Notimes.to_string(),format!("{:?}",i1.value().to_string()));
                    //         // }
                            
                    //         // println!("{}",format!("{} {}",browser,expandedurl));
                    //         // set_token(ChosenBrowser.to_string(),browser.to_string());
                           
                    //     //     if cfg!(windows){
                    //     //     if(v.contains("exe")){
                    //     //         let mut res = Command::new(format!("{}",v))
                    //     //                     .arg(format!("{}",expandedurl))
                    //     //                     .output();
                    //     //                     fltk::app::quit();
                    //     //     }
                    //     //     else{
                    //     //          fltk::dialog::message(90, 90, "Please setup config before use. You can find it at D");{
                    //     //     }
                    //     //     }
                    //     // }
                    //     // else
                    //     {
                    //             let mut res = Command::new(format!("{}",v))
                    //                             .arg(format!("{}",expandedurl))
                    //                             .output();
                    //                             println!("oepning----->{}",expandedurl);
                    //                             fltk::app::quit();
                    //         }
                        
                    //     true
                    //     }
                    //     _ => false,
                    // });
                    i+=1;
                    if(i%3 ==0){
                        println!("i value--------->{}",i);
                        hpack.end();
                    hpack.set_type(fltk::group::PackType::Horizontal);
                    fltk::frame::Frame::default().with_size(20, 10);
                    hpack=fltk::group::Pack::default().with_size(250,40) .center_of(&win);
                    }
                }
                // let browsers = "";

            hpack.end();
            hpack.set_type(fltk::group::PackType::Horizontal);
            win.make_resizable(true);
            // win.resizable(&vpack);

            vpack.end();    
            vpack.set_type(fltk::group::PackType::Vertical);
            
            win.show_with_env_args();

            win.end();
            win.show();
            // but.set_callback(move |_| {
                
            // });
            // let mut frame1 =frame.clone();
            while app.wait() {
                // setframe(&mut frame, "");
                // frame=frame.clone();
                match r.recv() {
                    
                    Some(val) => 
                    match val {
                        val => {
                            // if(val == "frominput"){
                            //             ourl=url.value();
                            //     }
                            // let mut str=val;
                            if(val.contains("//")){
                                // let k= format!("{}",val);
                                // frame.set_label(&k);
                                setframe(&mut framet, &val);
                                println!("//------------->");

                                println!("{}",format!("{}",val));
                            ourl=format!("{}",val);
                            expandedurl=val;
                            // rt.set_label("title");
                            // frame.set_label("");
                            // setframe(&mut frame,"");
                            
                            true;
                            }
                            else if val == "refresh"{
                                let state = state.lock().unwrap();
                                framet.set_label_size(12);
                                let val=&state.to_string();
                                expandedurl=val.to_string();
                                ourl=val.to_string();
                                setframe(&mut framet,&state);
                            }
                            else if val == "expandurl"{
                                match eurl(ourl.clone()) {
                                    Ok(sk) => { 
                                        if(sk.to_lowercase().contains("invalid")){
                                            setframe(&mut framet,args.get(1).unwrap());
                                            // rt.set_label("");
                                        }
                                        else{
                                            setframe(&mut framet, &sk);
                                        }
                                        
                                        // fltk::dialog::message(90, 90, &sk);{
                                            // let mut res = std::process::Command::new(format!("/home/roger/Downloads/waterfox/waterfox {}",sk)).output();
                                        // }
                                        
                                        // ... use sk ...
                                    },
                                    Err(e) => {
                                        setframe(&mut framet,"Error");
                                        // ... sk is not available, and e explains why ...
                                    },
                                }
                            }
                            else if(val == "all"){
                                println!("all------------->");
                                // if ourl==" "{
                                //     ourl=url.value(); 
                                //  }
                                if(prefstore::getall(appname).is_empty()){
                                    reinit();
                                }
                                let(hmap)=prefstore::getall(appname);
                                for (_,v) in hmap{
                                    open(&v,&ourl);
                                }
                                true;
                            }
                            else if(val == "svc"){
                                let mut clipboard = Clipboard::new().unwrap();
                                println!("{}",&ourl);
                                #[cfg(target_os = "linux")]{
                                    clipboard.set().wait().text(&ourl).unwrap();
                                }
                                #[cfg(not(target_os = "linux"))]{
                                    clipboard.set_text(&ourl).unwrap();
                                }
                                // clipboard.set_text("abc".to_string()).unwrap();
                                println!("{}",clipboard.get_text().unwrap());
                            }else if(val == "svw"){
                                // ada
                            }
                            // else 
                            else{
                                // if ourl==" "{
                                //     ourl=url.value(); 
                                //  }
                                 
                                println!("{}------------->r{}r",val,expandedurl);
                                
                                open(&val,&expandedurl);
                                                println!("opening----->{}",expandedurl);
                                                // fltk::app::quit();
                                                true;
                            }
                            
                            // frame.set_label(&val);
                            
                        },
                        // Message::Stop => rlist(),
                    },
                    None => ({
                        // println!("stop");
                    })
                }
                // let frame=win.frame.clone();
                // frame.set_label("&val");
            }
            // app.run().unwrap();    
            
}
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
const DAEMONIZE_ARG: &str = "__internal_daemonize";
// fn sendtoclip(msg:&String) -> Result<()> {
// 	#[cfg(target_os = "linux")]
// 	if env::args().nth(1).as_deref() == Some(DAEMONIZE_ARG) {
// 		Clipboard::new()?.set().wait().text(msg)?;
// 		return Ok(());
// 	}

// 	// SimpleLogger::new().init().unwrap();

// 	if cfg!(target_os = "linux") {
// 		process::Command::new(env::current_exe()?)
// 			.arg(DAEMONIZE_ARG)
// 			.stdin(process::Stdio::null())
// 			.stdout(process::Stdio::null())
// 			.stderr(process::Stdio::null())
// 			.current_dir("/")
// 			.spawn()?;
// 	} else {
// 		Clipboard::new()?.set_text("Hello, world!")?;
// 	}

// 	Ok(())
// }
fn setframe(f:&mut Frame,s: &str){
    let ss: String = s.chars().skip(0).take(40).collect();
    f.set_label(&ss);
}
fn open(v: &String,ourl: &String){
    // #[cfg(target_os = "linux")]
    // #[cfg(target_os = "windows")]
    { 
    println!("test--->{}",ourl);
    println!("browser--->{}",v);
    
    let strings:Vec<String> = v.split_whitespace().map(str::to_string).collect();
    let mut res = Command::new(format!("{}",strings[0]));
    let slice = &strings[1..strings.len()];

    for k in slice{ 
        res.arg(k);
    }

    let tte=res.arg(format!("{}",ourl))
                        .spawn()
                        .expect("failed to execute process");
    eprintln!("{:?}",tte);

        // #[cfg(target_os = "linux")]
    }
    // #[cfg(target_os = "macos")]{
    //     let mut res =  Command::new("open");
    //     let strings:Vec<String> = v.split_whitespace().map(str::to_string).collect();
    //     res.arg("-a");
    //     for k in strings{ 
    //         res.arg(k);
    //     }
    //     res.arg(&ourl).spawn();
    // }
    // expandedurl=url.value();
   
    // process::exit(0);

    // let mut command = command(format!("{} {}",v,ourl));
    //             command.stdout(Stdio::piped());
    //     let output = command.execute_output().unwrap();
    //     println!("{}", String::from_utf8(output.stdout).unwrap());
}