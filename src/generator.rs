use crate::ir::IrItem;

pub fn generate(items: &[IrItem]) -> String {
    let mut lines = Vec::new();
    let mut indent_level = 0usize;

    for item in items {
        match item {
            IrItem::Comment(text) => lines.push(format!("// {text}")),
            IrItem::FunctionStart { name } => {
                lines.push(format!("fn {name}() {{"));
                indent_level += 1;
            }
            IrItem::BlockEnd => {
                indent_level = indent_level.saturating_sub(1);
                lines.push("}".to_string());
            }
            IrItem::Println(args) => {
                lines.push(format!("{}println!({args});", indent(indent_level)));
            }
            IrItem::VarDecl {
                name,
                rust_type,
                value,
            } => {
                let line = match value {
                    Some(value) => format!("let mut {name}: {rust_type} = {value};"),
                    None => format!("let mut {name}: {rust_type};"),
                };
                lines.push(format!("{}{}", indent(indent_level), line));
            }
            IrItem::Return(value) => {
                lines.push(format!("{}return {value};", indent(indent_level)));
            }
            IrItem::Todo(original) => {
                lines.push(format!(
                    "{}// TODO(go2rust): original line: {original}",
                    indent(indent_level)
                ));
            }
            IrItem::Empty => lines.push(String::new()),
        }
    }

    let mut output = lines.join("\n");
    output.push('\n');
    output
}

fn indent(level: usize) -> String {
    "    ".repeat(level)
}
