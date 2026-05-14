use crate::error::Result;
use crate::ir::IrItem;
use regex::Regex;

pub fn parse(source: &str) -> Result<Vec<IrItem>> {
    let package_re = Regex::new(r"^\s*package\s+(\w+)\s*$")?;
    let import_re = Regex::new(r#"^\s*import\s+"([^"]+)"\s*$"#)?;
    let func_main_re = Regex::new(r"^\s*func\s+main\s*\(\s*\)\s*\{\s*$")?;
    let println_re = Regex::new(r#"^\s*fmt\.Println\((.*)\)\s*$"#)?;
    let var_string_re = Regex::new(r"^\s*var\s+([A-Za-z_]\w*)\s+string\s*$")?;
    let var_int_init_re = Regex::new(r"^\s*var\s+([A-Za-z_]\w*)\s+int\s*=\s*(.+)\s*$")?;
    let return_re = Regex::new(r"^\s*return\s+(.+)\s*$")?;
    let block_end_re = Regex::new(r"^\s*}\s*$")?;

    let mut items = Vec::new();
    let mut unsupported_block_depth = 0usize;

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            items.push(IrItem::Empty);
        } else if let Some(caps) = package_re.captures(line) {
            items.push(IrItem::Comment(format!("Go package: {}", &caps[1])));
        } else if let Some(caps) = import_re.captures(line) {
            items.push(IrItem::Comment(format!("Go import: {}", &caps[1])));
        } else if func_main_re.is_match(line) {
            items.push(IrItem::FunctionStart {
                name: "main".to_string(),
            });
        } else if let Some(caps) = println_re.captures(line) {
            items.push(IrItem::Println(caps[1].trim().to_string()));
        } else if let Some(caps) = var_string_re.captures(line) {
            items.push(IrItem::VarDecl {
                name: caps[1].to_string(),
                rust_type: "String".to_string(),
                value: None,
            });
        } else if let Some(caps) = var_int_init_re.captures(line) {
            items.push(IrItem::VarDecl {
                name: caps[1].to_string(),
                rust_type: "i32".to_string(),
                value: Some(caps[2].trim().to_string()),
            });
        } else if let Some(caps) = return_re.captures(line) {
            items.push(IrItem::Return(caps[1].trim().to_string()));
        } else if block_end_re.is_match(line) && unsupported_block_depth > 0 {
            unsupported_block_depth -= 1;
            items.push(IrItem::Todo(trimmed.to_string()));
        } else if block_end_re.is_match(line) {
            items.push(IrItem::BlockEnd);
        } else {
            if trimmed.ends_with('{') {
                unsupported_block_depth += 1;
            }
            items.push(IrItem::Todo(trimmed.to_string()));
        }
    }

    Ok(items)
}
