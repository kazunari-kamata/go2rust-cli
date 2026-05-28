use crate::ir::IrItem;

pub fn generate(items: &[IrItem]) -> String {
    let mut lines = Vec::new();
    let mut indent_level = 0usize;

    for item in items {
        match item {
            IrItem::Comment(text) => lines.push(format!("// {text}")),
            IrItem::FunctionStart {
                name,
                params,
                return_type,
            } => {
                let params = params
                    .iter()
                    .map(|param| format!("{}: {}", param.name, param.rust_type))
                    .collect::<Vec<_>>()
                    .join(", ");
                let return_type = return_type
                    .as_ref()
                    .map(|rust_type| format!(" -> {rust_type}"))
                    .unwrap_or_default();
                lines.push(format!("fn {name}({params}){return_type} {{"));
                indent_level += 1;
            }
            IrItem::IfStart(condition) => {
                lines.push(format!("{}if {condition} {{", indent(indent_level)));
                indent_level += 1;
            }
            IrItem::ElseIfStart(condition) => {
                indent_level = indent_level.saturating_sub(1);
                lines.push(format!("{}}} else if {condition} {{", indent(indent_level)));
                indent_level += 1;
            }
            IrItem::ElseStart => {
                indent_level = indent_level.saturating_sub(1);
                lines.push(format!("{}}} else {{", indent(indent_level)));
                indent_level += 1;
            }
            IrItem::BlockEnd => {
                indent_level = indent_level.saturating_sub(1);
                lines.push(format!("{}}}", indent(indent_level)));
            }
            IrItem::Print(args) => {
                lines.push(format!("{}print!({args});", indent(indent_level)));
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
            IrItem::ShortVarDecl { name, value } => {
                lines.push(format!("{}let mut {name} = {value};", indent(indent_level)));
            }
            IrItem::Assignment { name, value } => {
                lines.push(format!("{}{name} = {value};", indent(indent_level)));
            }
            IrItem::ExpressionStmt(expression) => {
                lines.push(format!("{}{expression};", indent(indent_level)));
            }
            IrItem::Return(value) => {
                let line = match value {
                    Some(value) => format!("return {value};"),
                    None => "return;".to_string(),
                };
                lines.push(format!("{}{}", indent(indent_level), line));
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
