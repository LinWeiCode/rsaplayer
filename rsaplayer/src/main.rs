fn main() {
    println!("Welcome");
    println!("VER---{}",reslib::VER);
    println!("拖入歌曲到窗口");

    let mut s=String::new();
    std::io::stdin().read_line(&mut s).expect("err");
    let s=s.replace("\"", "");
    let s=s.trim().to_string();
    //let y=format!("{}",s);
    let r=reslib::songs::Songs{path_name:s};
    r.play();

    let mut ss=String::new();
    std::io::stdin().read_line(&mut ss).unwrap();

}
