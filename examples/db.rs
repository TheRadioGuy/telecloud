// fn main(){
//     let root = Element::from_reader(r#"<?xml version="1.0"?>
// <root xmlns="tag:myns" xmlns:foo="tag:otherns">
//     <list a="1" b="2" c="3">
//         <item foo:attr="foo1"/>
//         <item foo:attr="foo2"/>
//         <item foo:attr="foo3"/>
//     </list>
// </root>
// "#.as_bytes()).unwrap();
// let list = root.find("{tag:myns}list").unwrap();
// for child in list.find_all("{tag:myns}item") {
//     println!("attribute: {}", child.get_attr("{tag:otherns}attr").unwrap());
// }

// }

use elementtree::Element;

fn main() {
    // create_dir_if_dont_exist!("fff", "ffff").await;
    let root = Element::from_reader(
        r#"<?xml version="1.0" encoding="utf-8"?>
    <propfind xmlns="DAV:"><prop>
    <getcontentlength xmlns="DAV:"/>
    <getlastmodified xmlns="DAV:"/>
    <executable xmlns="http://apache.org/dav/props/"/>
    <resourcetype xmlns="DAV:"/>
    <checked-in xmlns="DAV:"/>
    <checked-out xmlns="DAV:"/>
    </prop></propfind>
    "#
        .as_bytes(),
    )
    .unwrap();
    let list = root.find("{DAV:}prop").unwrap();
    for child in root.children() {
        println!("attribute: {:?}", child);
    }
}

#[macro_export]
macro_rules! create_dir_if_dont_exist {
    ( $( $x:expr ),* ) => {
        async move {
            use tokio::fs::create_dir;
            $(
                create_dir($x).await;
            )*
        }
    };
}
