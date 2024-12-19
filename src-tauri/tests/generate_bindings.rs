use std::collections::HashSet;

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn rust_type_to_ts(rust_type: &syn::Type) -> (String, HashSet<String>) {
        match rust_type {
            syn::Type::Path(type_path) if type_path.qself.is_none() => {
                let segments = &type_path.path.segments;
                let last_segment = segments.last().unwrap();
                let ident = &last_segment.ident;
                match ident.to_string().as_str() {
                    "str" | "String" => ("string".to_owned(), HashSet::new()),
                    "()" => ("void".to_owned(), HashSet::new()),
                    "bool" => ("boolean".to_owned(), HashSet::new()),
                    "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64"
                    | "usize" | "f32" | "f64" => ("number".to_owned(), HashSet::new()),
                    "Vec" | "HashSet" => {
                        if let syn::PathArguments::AngleBracketed(angle_bracketed_data) =
                            &last_segment.arguments
                        {
                            if let Some(syn::GenericArgument::Type(ty)) =
                                angle_bracketed_data.args.first()
                            {
                                let (inner_type, inner_imports) = rust_type_to_ts(ty);
                                (format!("{}[]", inner_type), inner_imports)
                            } else {
                                ("unknown[]".to_owned(), HashSet::new())
                            }
                        } else {
                            ("unknown[]".to_owned(), HashSet::new())
                        }
                    }
                    "HashMap" => {
                        if let syn::PathArguments::AngleBracketed(angle_bracketed_data) =
                            &last_segment.arguments
                        {
                            let args: Vec<_> = angle_bracketed_data.args.iter().collect();
                            if args.len() == 2 {
                                if let (
                                    syn::GenericArgument::Type(key_ty),
                                    syn::GenericArgument::Type(value_ty),
                                ) = (&args[0], &args[1])
                                {
                                    let (key_type, mut key_imports) = rust_type_to_ts(key_ty);
                                    let (value_type, value_imports) = rust_type_to_ts(value_ty);
                                    key_imports.extend(value_imports);
                                    (format!("Record<{}, {}>", key_type, value_type), key_imports)
                                } else {
                                    ("Record<unknown, unknown>".to_owned(), HashSet::new())
                                }
                            } else {
                                ("Record<unknown, unknown>".to_owned(), HashSet::new())
                            }
                        } else {
                            ("Record<unknown, unknown>".to_owned(), HashSet::new())
                        }
                    }
                    "Value" => ("null|undefined".to_owned(), HashSet::new()),
                    "Result" => {
                        if let syn::PathArguments::AngleBracketed(angle_bracketed_data) =
                            &last_segment.arguments
                        {
                            let args: Vec<_> = angle_bracketed_data.args.iter().collect();
                            if let syn::GenericArgument::Type(ok_type) = &args[0] {
                                rust_type_to_ts(ok_type)
                            } else {
                                ("unknown".to_owned(), HashSet::new())
                            }
                        } else {
                            ("unknown".to_owned(), HashSet::new())
                        }
                    }
                    _ => {
                        // Handle generic types
                        if !last_segment.arguments.is_empty() {
                            if let syn::PathArguments::AngleBracketed(angle_bracketed_data) =
                                &last_segment.arguments
                            {
                                let mut all_imports = HashSet::new();
                                let type_args: Vec<String> = angle_bracketed_data
                                    .args
                                    .iter()
                                    .filter_map(|arg| {
                                        if let syn::GenericArgument::Type(ty) = arg {
                                            let (ts_type, imports) = rust_type_to_ts(ty);
                                            all_imports.extend(imports);
                                            Some(ts_type)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                all_imports.insert(ident.to_string());
                                (format!("{}<{}>", ident, type_args.join(", ")), all_imports)
                            } else {
                                let mut imports = HashSet::new();
                                imports.insert(ident.to_string());
                                (ident.to_string(), imports)
                            }
                        } else {
                            let mut imports = HashSet::new();
                            imports.insert(ident.to_string());
                            (ident.to_string(), imports)
                        }
                    }
                }
            }
            syn::Type::Reference(type_reference) => rust_type_to_ts(&type_reference.elem),
            syn::Type::Tuple(tuple_type) if tuple_type.elems.is_empty() => {
                ("void".to_owned(), HashSet::new())
            }
            _ => ("unknown".to_owned(), HashSet::new()),
        }
    }

    fn to_camel_case(s: &str) -> String {
        let mut camel_case = String::new();
        let mut capitalize_next = false;
        for c in s.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                camel_case.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                camel_case.push(c);
            }
        }
        camel_case
    }

    fn process_file(file_path: &Path) -> (Vec<String>, HashSet<String>) {
        let contents = fs::read_to_string(file_path).unwrap();
        let ast = syn::parse_file(&contents).unwrap();
        let mut commands = Vec::new();
        let mut all_imports = HashSet::new();

        for item in ast.items {
            if let syn::Item::Fn(item_fn) = item {
                let tauri_command_attr = item_fn.attrs.iter().find(|attr| {
                    attr.path()
                        .segments
                        .iter()
                        .map(|seg| seg.ident.to_string())
                        .collect::<Vec<_>>()
                        == ["tauri", "command"]
                });

                if tauri_command_attr.is_some() {
                    let command_name = &item_fn.sig.ident.to_string();

                    let mut arg_types = Vec::new();
                    for arg in &item_fn.sig.inputs {
                        if let syn::FnArg::Typed(pat_type) = arg {
                            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                let ty_string = quote::quote! {#pat_type.ty}.to_string();
                                if !ty_string.contains("State") && !ty_string.contains("AppHandle")
                                {
                                    let (ts_type, imports) = rust_type_to_ts(&pat_type.ty);
                                    all_imports.extend(imports);
                                    arg_types.push(format!("{}: {}", pat_ident.ident, ts_type));
                                }
                            }
                        }
                    }

                    let (return_type, return_imports) =
                        if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output {
                            rust_type_to_ts(ty)
                        } else {
                            ("void".to_string(), HashSet::new())
                        };
                    all_imports.extend(return_imports);

                    let args_type = if arg_types.is_empty() {
                        "undefined".to_string()
                    } else {
                        format!("{{ {} }}", arg_types.join(", "))
                    };

                    let command_definition = format!(
                        "    {}: {{\n        returns: {},\n        args: {}\n    }}",
                        command_name, return_type, args_type
                    );
                    commands.push(command_definition);
                }
            }
        }

        (commands, all_imports)
    }

    #[test]
    fn build_command_type_definitions() {
        let command_dir = Path::new("src/commands");
        let mut all_commands = Vec::new();
        let mut all_imports = HashSet::new();

        for entry in fs::read_dir(command_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap_or_default() == "rs" {
                let (commands, imports) = process_file(&path);
                all_commands.extend(commands);
                all_imports.extend(imports);
            }
        }

        // Generate imports
        let imports = all_imports
            .iter()
            .map(|import| format!("export * from \'./{}\'", import))
            .collect::<Vec<_>>()
            .join("\n");

        // Generate file content
        let warning_header = "// THIS FILE IS AUTO-GENERATED BY CARGO TESTS! DO NOT EDIT!";
        let invoke_import = "import { core } from \'@tauri-apps/api\';";
        let tauri_commands = format!(
            "export type TauriCommands = {{\n{}\n}};",
            all_commands.join(",\n")
        );
        let invoke_fn = indoc::indoc! {"
            export function invoke<T extends keyof TauriCommands>(
                cmd: T,
                args?: TauriCommands[T]['args']
            ): Promise<TauriCommands[T]['returns']> {
                return core.invoke(cmd, args);
            }
        "};

        let output = format!(
            "{}\n\n{}\n\n{}\n\n{}",
            warning_header, invoke_import, tauri_commands, invoke_fn
        );

        // Write to file
        let file_path = "../src/types/tauri_commands.ts";
        let file = std::fs::File::create(file_path).unwrap();
        std::io::Write::write_all(&mut std::io::BufWriter::new(file), output.as_bytes()).unwrap();

        let file_path = "../src/types/bindings/index.ts";
        let file = std::fs::File::create(file_path).unwrap();
        std::io::Write::write_all(&mut std::io::BufWriter::new(file), imports.as_bytes()).unwrap();

        println!("Generated TypeScript bindings at: {}", file_path);
    }
}