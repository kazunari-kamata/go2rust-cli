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
    for _, value := range values {
        fmt.Println("nested")
    }
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    assert!(actual.contains("// TODO(go2rust): original line: for _, value := range values {"));
    assert!(actual.contains(r#"println!("nested");"#));
}

#[test]
fn converts_typed_variables_short_variables_and_assignments() {
    let source = r#"package main

func main() {
    var name string = "go2rust"
    var enabled bool = true
    var ratio float64 = 1.5
    count := 1
    count = count + 1
    fmt.Println(name, enabled, ratio, count)
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    let expected = r#"// Go package: main

fn main() {
    let mut name: String = "go2rust";
    let mut enabled: bool = true;
    let mut ratio: f64 = 1.5;
    let mut count = 1;
    count = count + 1;
    println!(name, enabled, ratio, count);
}
"#;

    assert_eq!(actual, expected);
}

#[test]
fn converts_simple_functions_and_if_blocks() {
    let source = r#"package main

func add(a, b int) int {
    total := a + b
    if total > 0 {
        return total
    }
    return 0
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    let expected = r#"// Go package: main

fn add(a: i32, b: i32) -> i32 {
    let mut total = a + b;
    if total > 0 {
        return total;
    }
    return 0;
}
"#;

    assert_eq!(actual, expected);
}

#[test]
fn converts_else_if_else_print_and_empty_return() {
    let source = r#"package main

import "fmt"

func describe(count int) {
    if count > 10 {
        fmt.Print("large")
        return
    } else if count > 0 {
        fmt.Println("positive")
    } else {
        fmt.Println("zero")
    }
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    let expected = r#"// Go package: main

// Go import: fmt

fn describe(count: i32) {
    if count > 10 {
        print!("large");
        return;
    } else if count > 0 {
        println!("positive");
    } else {
        println!("zero");
    }
}
"#;

    assert_eq!(actual, expected);
}

#[test]
fn converts_simple_function_call_statements() {
    let source = r#"package main

func logIfEnabled(enabled bool) {
    if !enabled {
        return
    }
}

func main() {
    logIfEnabled(true)
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    let expected = r#"// Go package: main

fn logIfEnabled(enabled: bool) {
    if !enabled {
        return;
    }
}

fn main() {
    logIfEnabled(true);
}
"#;

    assert_eq!(actual, expected);
}

#[test]
fn converts_simple_for_loops() {
    let source = r#"package main

import "fmt"

func countdown(count int) {
    for count > 0 {
        fmt.Println(count)
        count = count - 1
    }
}

func run() {
    for {
        fmt.Println("tick")
        return
    }
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    let expected = r#"// Go package: main

// Go import: fmt

fn countdown(count: i32) {
    while count > 0 {
        println!(count);
        count = count - 1;
    }
}

fn run() {
    loop {
        println!("tick");
        return;
    }
}
"#;

    assert_eq!(actual, expected);
}

#[test]
fn converts_simple_switch_blocks() {
    let source = r#"package main

import "fmt"

func describe(count int) {
    switch count {
    case 0:
        fmt.Println("zero")
    case 1, 2:
        fmt.Println("small")
    default:
        fmt.Println("many")
    }
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    let expected = r#"// Go package: main

// Go import: fmt

fn describe(count: i32) {
    match count {
        0 => {
            println!("zero");
        },
        1 | 2 => {
            println!("small");
        },
        _ => {
            println!("many");
        },
    }
}
"#;

    assert_eq!(actual, expected);
}

#[test]
fn converts_condition_switch_blocks() {
    let source = r#"package main

import "fmt"

func describe(count int) {
    switch {
    case count > 10:
        fmt.Println("large")
    case count > 0, count == -1:
        fmt.Println("known")
    default:
        fmt.Println("zero")
    }
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    let expected = r#"// Go package: main

// Go import: fmt

fn describe(count: i32) {
    match () {
        _ if count > 10 => {
            println!("large");
        },
        _ if count > 0 || count == -1 => {
            println!("known");
        },
        _ => {
            println!("zero");
        },
    }
}
"#;

    assert_eq!(actual, expected);
}

#[test]
fn keeps_fallthrough_as_todo_inside_switch_arm() {
    let source = r#"package main

import "fmt"

func describe(count int) {
    switch count {
    case 1:
        fallthrough
    default:
        fmt.Println("done")
    }
}
"#;

    let actual = convert_source(source).expect("conversion should succeed");

    assert!(actual.contains("// TODO(go2rust): original line: fallthrough"));
    assert!(actual.contains("_ => {"));
}
