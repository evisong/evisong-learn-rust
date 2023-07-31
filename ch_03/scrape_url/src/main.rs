use std::fs;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let url = args.get(1).cloned().unwrap_or("https://www.rust-lang.org/".to_string());
  let output = args.get(2).cloned().unwrap_or("target/rust.md".to_string());

  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output.clone(), md.as_bytes()).unwrap();
  println!("Converted markdown has been saved to {}", output);
}
