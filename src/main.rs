use std::fs::{self, DirEntry};
use std::env;
use serde_yaml::Value;
use serde::Serialize;
use markdown::{to_html, to_html_with_options, Constructs, Options, ParseOptions};
use askama::Template;
use std::cmp::{Ordering, self, Reverse};
#[derive(Template)]
#[template(path = "blog.html")]
struct Context {
    date: String,
    title: String,
    slot: String
}



struct Blog{
    date:String,
    title: String,
    path: String,
    tag: String,
    sr_no: i64,
}


#[derive(Template)]
#[template(path = "blogs.html")]
struct BlogsPage {
   blogs: Vec<Blog>
}

fn main() {
    let path = get_path("/blogs".to_string());
    let blogs = get_all_blogs(path);
    let mut paths: Vec<Blog> = vec![];
    for blog in blogs{
        let s = fs::read_to_string(blog.path());
        match s{
            Ok(s) => {

                let mut s_slice: &str = &s;
                let mut htmlContent = markdown::to_html_with_options(s_slice, &Options{
                    parse: ParseOptions{
                        constructs: Constructs{
                            frontmatter: true,
                            ..Constructs::default()
                        },
                        ..ParseOptions::default()
                    },
                    ..Options::default()
                }).unwrap();
                htmlContent = htmlContent.replace('\n', "");
                if(s_slice.starts_with("---")){
                    let pars = s_slice.strip_prefix("---");
                    let mut ans = String::new();
                    for c in pars.unwrap().chars(){
                        if(c == '-'){
                            break;
                        }
                        else{
                            ans = format!("{}{}", ans, c);
                        }
                    }
                    let mut str_ans: &str = &ans;
                    let output = serde_yaml::from_str::<Value>(str_ans).unwrap();
                    let date:String = output.get("date").unwrap().as_str().unwrap().to_string();
                    let title:String = output.get("title").unwrap().as_str().unwrap().to_string();
                    let tag:String = output.get("tags").unwrap().as_str().unwrap().to_string();
                    let serial:i64 = output.get("serial_no").unwrap().as_i64().unwrap();
                    let context = Context{
                        date: date.clone(),
                        title: title.clone(),
                        slot: htmlContent
                    };
                    let rendered = context.render().unwrap().replace('\n', "");
                    let path = get_path(format!("/output/blog-{}.html", serial.clone()));
                    
                    fs::create_dir(get_path("/output".to_string()));
                    fs::write(path, rendered);
                    paths.push(Blog{
                        date: date.clone(),
                        title: title.clone(),
                        path: format!("blog-{}.html", serial.clone()),
                        tag: tag,
                        sr_no: serial.clone()
                    })
                    
                }
            },
            Err(_) => {}
        }
       
    }
        

    paths.sort_by_key(|w| Reverse(w.sr_no));
    
    for path in paths.iter_mut(){
        println!("{}", path.sr_no);
    }
    let rendered = BlogsPage{
        blogs: paths
    }.render().unwrap().replace('\n', "");
    let path = get_path("/output/blogs.html".to_string());
    fs::write(path, rendered);
                    
    
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn get_all_blogs( path:String) -> Vec<DirEntry>{
let entries = fs::read_dir(path);
    let mut files: Vec<DirEntry> = Vec::new();
    for entry in entries.unwrap(){
        match entry{
            Ok(entry) => {
                files.push(entry);
            },
            Err(_) => {}
        }
    }
    files
}


fn get_path(path: String) -> String{
    get_current_working_dir() + &path
}
