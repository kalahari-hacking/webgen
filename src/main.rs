use pages::academy;
use pages::blog;
use pages::contact;
use pages::electronics;
use pages::landing;
use pages::shop;
use pages::topics;
use pages::zebra;

pub mod pages;
pub mod widgets;

fn main() {
    let landing = landing::page();
    let academy = academy::page();
    let blog = blog::page();
    let contact = contact::page();
    let shop = shop::page();
    let zebra = zebra::page();

    let electronics_index = electronics::index::page();
    let electronics_repair_index = electronics::repairs::index::page();
    let electronics_rep0 = electronics::repairs::rep0::page();
    let electronics_rep1 = electronics::repairs::rep1::page();

    let topics_index = topics::index::page();
    let topics_sony = topics::sony::page();
    let topics_ps3 = topics::ps3::page();
    let topics_hardware = topics::hardware::page();
    let topics_software = topics::software::page();
    let topics_microsoft = topics::microsoft::page();
    let topics_xbox360 = topics::xbox360::page();

    println!("--= Generating landing page...");
    std::fs::write("www/index.html", landing.as_bytes()).unwrap();

    println!("--= Generating academy page...");
    std::fs::write("www/academy.html", academy.as_bytes()).unwrap();

    println!("--= Generating blog page...");
    std::fs::write("www/blog.html", blog.as_bytes()).unwrap();

    println!("--= Generating contact page...");
    std::fs::write("www/contact.html", contact.as_bytes()).unwrap();

    println!("--= Generating shop page...");
    std::fs::write("www/shop.html", shop.as_bytes()).unwrap();

    println!("--= Generating zebra page...");
    std::fs::write("www/zebra.html", zebra.as_bytes()).unwrap();

    println!("--= Generating electronics index page...");
    std::fs::write("www/electronics/index.html", electronics_index.as_bytes()).unwrap();

    println!("--= Generating electronics/repairs/index page...");
    std::fs::write(
        "www/electronics/repairs.html",
        electronics_repair_index.as_bytes(),
    )
    .unwrap();

    println!("--= Generating electronics/repairs/0 page...");
    std::fs::write(
        "www/electronics/repairs/0.html",
        electronics_rep0.as_bytes(),
    )
    .unwrap();

    println!("--= Generating electronics/repairs/1 page...");
    std::fs::write(
        "www/electronics/repairs/1.html",
        electronics_rep1.as_bytes(),
    )
    .unwrap();

    println!("--= Generating topics/index page...");
    std::fs::write("www/topics/index.html", topics_index.as_bytes()).unwrap();

    println!("--= Generating topics/sony page...");
    std::fs::write("www/topics/sony.html", topics_sony.as_bytes()).unwrap();

    println!("--= Generating topics/ps3 page...");
    std::fs::write("www/topics/ps3.html", topics_ps3.as_bytes()).unwrap();

    println!("--= Generating topics/hardware page...");
    std::fs::write("www/topics/hardware.html", topics_hardware.as_bytes()).unwrap();

    println!("--= Generating topics/software page...");
    std::fs::write("www/topics/software.html", topics_software.as_bytes()).unwrap();

    println!("--= Generating topics/microsoft page...");
    std::fs::write("www/topics/microsoft.html", topics_microsoft.as_bytes()).unwrap();

    println!("--= Generating topics/xbox360 page...");
    std::fs::write("www/topics/xbox360.html", topics_xbox360.as_bytes()).unwrap();
}
