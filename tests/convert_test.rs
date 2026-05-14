use go2rust_cli::convert_source;

#[test]
fn converts_supported_go_constructs_to_rust_skeleton() {
    let source = r#"package main

import "fmt"

func main() {
    var name string
    var count int = 1
    fmt.Println("hello")
    return count
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    let expected = r#"// Go package: main

// Go import: fmt

fn main() {
    let mut name: String;
    let mut count: i32 = 1;
    println!("hello");
    return count;
}
"#;

    assert_eq!(actual, expected);
}

#[test]
fn keeps_unsupported_lines_as_todo_comments() {
    let source = r#"package main

func main() {
    if true {
        fmt.Println("nested")
    }
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    assert!(actual.contains("// TODO(go2rust): original line: if true {"));
    assert!(actual.contains(r#"println!("nested");"#));
}
