use crate::error::Result;
use crate::ir::{IrItem, Parameter};
use regex::Regex;

pub fn parse(source: &str) -> Result<Vec<IrItem>> {
    let package_re = Regex::new(r"^\s*package\s+(\w+)\s*$")?;
    let import_re = Regex::new(r#"^\s*import\s+"([^"]+)"\s*$"#)?;
    let import_block_start_re = Regex::new(r"^\s*import\s*\(\s*$")?;
    let import_block_spec_re = Regex::new(r#"^\s*"([^"]+)"\s*$"#)?;
    let import_block_end_re = Regex::new(r"^\s*\)\s*$")?;
    let func_re =
        Regex::new(r"^\s*func\s+([A-Za-z_]\w*)\s*\(([^)]*)\)\s*([A-Za-z_]\w*)?\s*\{\s*$")?;
    let print_re = Regex::new(r#"^\s*fmt\.Print\((.*)\)\s*$"#)?;
    let println_re = Regex::new(r#"^\s*fmt\.Println\((.*)\)\s*$"#)?;
    let var_re = Regex::new(r"^\s*var\s+([A-Za-z_]\w*)\s+([A-Za-z_]\w*)\s*(?:=\s*(.+))?\s*$")?;
    let short_var_re = Regex::new(r"^\s*([A-Za-z_]\w*)\s*:=\s*(.+)\s*$")?;
    let assignment_re = Regex::new(r"^\s*([A-Za-z_]\w*)\s*=\s*(.+)\s*$")?;
    let if_re = Regex::new(r"^\s*if\s+(.+)\s*\{\s*$")?;
    let else_if_re = Regex::new(r"^\s*}\s*else\s+if\s+(.+)\s*\{\s*$")?;
    let else_re = Regex::new(r"^\s*}\s*else\s*\{\s*$")?;
    let infinite_for_re = Regex::new(r"^\s*for\s*\{\s*$")?;
    let three_clause_for_re = Regex::new(r"^\s*for\s*(.*?)\s*;\s*(.*?)\s*;\s*(.*?)\s*\{\s*$")?;
    let conditional_for_re = Regex::new(r"^\s*for\s+([^;]+)\s*\{\s*$")?;
    let increment_re = Regex::new(r"^\s*([A-Za-z_]\w*)\s*\+\+\s*$")?;
    let decrement_re = Regex::new(r"^\s*([A-Za-z_]\w*)\s*--\s*$")?;
    let condition_switch_re = Regex::new(r"^\s*switch\s*\{\s*$")?;
    let switch_re = Regex::new(r"^\s*switch\s+(.+)\s*\{\s*$")?;
    let case_re = Regex::new(r"^\s*case\s+(.+):\s*$")?;
    let default_re = Regex::new(r"^\s*default:\s*$")?;
    let return_re = Regex::new(r"^\s*return(?:\s+(.+))?\s*$")?;
    let function_call_re = Regex::new(r"^\s*([A-Za-z_]\w*\(.*\))\s*$")?;
    let block_end_re = Regex::new(r"^\s*}\s*$")?;

    let mut items = Vec::new();
    let mut unsupported_block_depth = 0usize;
    let mut blocks = Vec::new();
    let mut switches = Vec::new();
    let mut in_import_block = false;

    for line in source.lines() {
        let trimmed = line.trim();

        if in_import_block {
            if trimmed.is_empty() {
                items.push(IrItem::Empty);
            } else if import_block_end_re.is_match(line) {
                in_import_block = false;
            } else if let Some(caps) = import_block_spec_re.captures(line) {
                items.push(IrItem::Comment(format!("Go import: {}", &caps[1])));
            } else {
                items.push(IrItem::Todo(trimmed.to_string()));
            }
        } else if trimmed.is_empty() {
            items.push(IrItem::Empty);
        } else if let Some(caps) = package_re.captures(line) {
            items.push(IrItem::Comment(format!("Go package: {}", &caps[1])));
        } else if let Some(caps) = import_re.captures(line) {
            items.push(IrItem::Comment(format!("Go import: {}", &caps[1])));
        } else if import_block_start_re.is_match(line) {
            in_import_block = true;
        } else if let Some(caps) = func_re.captures(line) {
            items.push(IrItem::FunctionStart {
                name: caps[1].to_string(),
                params: parse_parameters(&caps[2]),
                return_type: caps
                    .get(3)
                    .map(|go_type| rust_type(go_type.as_str()).to_string()),
            });
            blocks.push(ParseBlockKind::General);
        } else if let Some(caps) = print_re.captures(line) {
            items.push(IrItem::Print(translate_expression(caps[1].trim())));
        } else if let Some(caps) = println_re.captures(line) {
            items.push(IrItem::Println(translate_expression(caps[1].trim())));
        } else if let Some(caps) = var_re.captures(line) {
            items.push(IrItem::VarDecl {
                name: caps[1].to_string(),
                rust_type: rust_type(&caps[2]).to_string(),
                value: caps
                    .get(3)
                    .map(|value| translate_expression(value.as_str().trim())),
            });
        } else if let Some(caps) = short_var_re.captures(line) {
            items.push(IrItem::ShortVarDecl {
                name: caps[1].to_string(),
                value: translate_expression(caps[2].trim()),
            });
        } else if let Some(caps) = assignment_re.captures(line) {
            items.push(IrItem::Assignment {
                name: caps[1].to_string(),
                value: translate_expression(caps[2].trim()),
            });
        } else if let Some(caps) = if_re.captures(line) {
            items.push(IrItem::IfStart(translate_expression(caps[1].trim())));
            blocks.push(ParseBlockKind::General);
        } else if let Some(caps) = else_if_re.captures(line) {
            blocks.pop();
            items.push(IrItem::ElseIfStart(translate_expression(caps[1].trim())));
            blocks.push(ParseBlockKind::General);
        } else if else_re.is_match(line) {
            blocks.pop();
            items.push(IrItem::ElseStart);
            blocks.push(ParseBlockKind::General);
        } else if infinite_for_re.is_match(line) {
            items.push(IrItem::LoopStart);
            blocks.push(ParseBlockKind::General);
        } else if let Some(caps) = three_clause_for_re.captures(line) {
            let initializer = caps[1].trim();
            let condition = caps[2].trim();
            let post = caps[3].trim();

            if condition.is_empty() {
                items.push(IrItem::Todo(trimmed.to_string()));
                blocks.push(ParseBlockKind::Unsupported);
            } else if let Some(post) =
                parse_for_post(post, &assignment_re, &increment_re, &decrement_re)
            {
                if let Some(initializer) =
                    parse_for_initializer(initializer, &var_re, &short_var_re, &assignment_re)
                {
                    items.push(initializer);
                    items.push(IrItem::WhileStart(translate_expression(condition)));
                    blocks.push(ParseBlockKind::ForClause { post });
                } else if initializer.is_empty() {
                    items.push(IrItem::WhileStart(translate_expression(condition)));
                    blocks.push(ParseBlockKind::ForClause { post });
                } else {
                    items.push(IrItem::Todo(trimmed.to_string()));
                    blocks.push(ParseBlockKind::Unsupported);
                }
            } else {
                items.push(IrItem::Todo(trimmed.to_string()));
                blocks.push(ParseBlockKind::Unsupported);
            }
        } else if let Some(caps) = conditional_for_re.captures(line) {
            let condition = caps[1].trim();
            if condition.contains("range") || condition.contains(":=") {
                items.push(IrItem::Todo(trimmed.to_string()));
                blocks.push(ParseBlockKind::Unsupported);
            } else {
                items.push(IrItem::WhileStart(translate_expression(condition)));
                blocks.push(ParseBlockKind::General);
            }
        } else if condition_switch_re.is_match(line) {
            items.push(IrItem::SwitchStart("()".to_string()));
            blocks.push(ParseBlockKind::Switch);
            switches.push(SwitchKind::Condition);
        } else if let Some(caps) = switch_re.captures(line) {
            items.push(IrItem::SwitchStart(translate_expression(caps[1].trim())));
            blocks.push(ParseBlockKind::Switch);
            switches.push(SwitchKind::Expression);
        } else if let Some(caps) = case_re.captures(line) {
            if let Some(kind) = switches.last().copied() {
                items.push(IrItem::CaseStart(translate_case(caps[1].trim(), kind)));
            } else {
                items.push(IrItem::Todo(trimmed.to_string()));
            }
        } else if default_re.is_match(line) {
            if switches.last().is_some() {
                items.push(IrItem::DefaultCaseStart);
            } else {
                items.push(IrItem::Todo(trimmed.to_string()));
            }
        } else if let Some(caps) = return_re.captures(line) {
            items.push(IrItem::Return(
                caps.get(1)
                    .map(|value| translate_expression(value.as_str().trim())),
            ));
        } else if let Some(caps) = function_call_re.captures(line) {
            items.push(IrItem::ExpressionStmt(translate_expression(caps[1].trim())));
        } else if block_end_re.is_match(line)
            && matches!(blocks.last(), Some(ParseBlockKind::Unsupported))
        {
            blocks.pop();
            items.push(IrItem::Todo(trimmed.to_string()));
        } else if block_end_re.is_match(line) && unsupported_block_depth > 0 {
            unsupported_block_depth -= 1;
            items.push(IrItem::Todo(trimmed.to_string()));
        } else if block_end_re.is_match(line) {
            if let Some(ParseBlockKind::ForClause { post }) = blocks.last() {
                items.push(post.clone());
            }
            if matches!(blocks.pop(), Some(ParseBlockKind::Switch)) {
                switches.pop();
            }
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

fn parse_parameters(source: &str) -> Vec<Parameter> {
    let mut params = Vec::new();
    let mut pending_names = Vec::new();

    for group in source
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
    {
        let mut parts = group.split_whitespace().collect::<Vec<_>>();

        if parts.len() == 1 {
            pending_names.push(parts[0].to_string());
            continue;
        }

        let go_type = parts.pop().expect("len checked above");
        let rust_type = rust_type(go_type).to_string();

        params.extend(pending_names.drain(..).map(|name| Parameter {
            name,
            rust_type: rust_type.clone(),
        }));
        params.extend(parts.into_iter().map(|name| Parameter {
            name: name.to_string(),
            rust_type: rust_type.clone(),
        }));
    }

    params
}

fn rust_type(go_type: &str) -> &str {
    match go_type {
        "string" => "String",
        "int" => "i32",
        "int64" => "i64",
        "float32" => "f32",
        "float64" => "f64",
        "bool" => "bool",
        other => other,
    }
}

fn translate_expression(expression: &str) -> String {
    expression.trim().to_string()
}

fn parse_for_initializer(
    initializer: &str,
    var_re: &Regex,
    short_var_re: &Regex,
    assignment_re: &Regex,
) -> Option<IrItem> {
    if let Some(caps) = var_re.captures(initializer) {
        Some(IrItem::VarDecl {
            name: caps[1].to_string(),
            rust_type: rust_type(&caps[2]).to_string(),
            value: caps
                .get(3)
                .map(|value| translate_expression(value.as_str().trim())),
        })
    } else if let Some(caps) = short_var_re.captures(initializer) {
        Some(IrItem::ShortVarDecl {
            name: caps[1].to_string(),
            value: translate_expression(caps[2].trim()),
        })
    } else {
        assignment_re
            .captures(initializer)
            .map(|caps| IrItem::Assignment {
                name: caps[1].to_string(),
                value: translate_expression(caps[2].trim()),
            })
    }
}

fn parse_for_post(
    post: &str,
    assignment_re: &Regex,
    increment_re: &Regex,
    decrement_re: &Regex,
) -> Option<IrItem> {
    if let Some(caps) = increment_re.captures(post) {
        Some(IrItem::Assignment {
            name: caps[1].to_string(),
            value: format!("{} + 1", &caps[1]),
        })
    } else if let Some(caps) = decrement_re.captures(post) {
        Some(IrItem::Assignment {
            name: caps[1].to_string(),
            value: format!("{} - 1", &caps[1]),
        })
    } else {
        assignment_re.captures(post).map(|caps| IrItem::Assignment {
            name: caps[1].to_string(),
            value: translate_expression(caps[2].trim()),
        })
    }
}

#[derive(Clone)]
enum ParseBlockKind {
    General,
    Switch,
    ForClause { post: IrItem },
    Unsupported,
}

#[derive(Clone, Copy)]
enum SwitchKind {
    Expression,
    Condition,
}

fn translate_case(patterns: &str, kind: SwitchKind) -> String {
    let parts = patterns
        .split(',')
        .map(str::trim)
        .filter(|pattern| !pattern.is_empty())
        .collect::<Vec<_>>();

    match kind {
        SwitchKind::Expression => parts.join(" | "),
        SwitchKind::Condition => format!(
            "_ if {}",
            parts
                .into_iter()
                .map(translate_expression)
                .collect::<Vec<_>>()
                .join(" || ")
        ),
    }
}
