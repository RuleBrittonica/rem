use proc_macro2::Span;
use quote::ToTokens;
use regex::Regex;
use std::borrow::Cow;
use std::fs;
use syn::{visit_mut::VisitMut, FnArg, Lifetime, LifetimeDef, Type};

use crate::common::{
    compile_file, format_source, repair_bounds_help, repair_iteration, CompilerError, RepairSystem,
};
use crate::repair_lifetime_simple;

pub struct Repairer {}

impl RepairSystem for Repairer {
    fn name(&self) -> &str {
        "_tightest_bounds_first_repairer"
    }

    fn repair_file(&self, file_name: &str, new_file_name: &str) -> bool {
        repair_lifetime_simple::Repairer {}.repair_file(file_name, new_file_name)
    }

    fn repair_function(&self, file_name: &str, new_file_name: &str, fn_name: &str) -> bool {
        fs::copy(file_name, &new_file_name).unwrap();
        annotate_tight_named_lifetime(&new_file_name, fn_name);
        //println!("annotated: {}", fs::read_to_string(&new_file_name).unwrap());
        let args: Vec<&str> = vec!["--error-format=json"];

        let mut compile_cmd = compile_file(&new_file_name, &args);

        let process_errors = |stderr: &Cow<str>| {
            if repair_bounds_help(stderr, new_file_name, fn_name) {
                true
            } else {
                loosen_bounds(stderr, new_file_name, fn_name)
            }
        };

        repair_iteration(&mut compile_cmd, &process_errors, true, Some(10))
    }
}

struct TightLifetimeAnnotatorTypeHelper {}

impl VisitMut for TightLifetimeAnnotatorTypeHelper {
    fn visit_type_mut(&mut self, i: &mut Type) {
        match i {
            Type::Reference(r) => {
                r.lifetime = Some(Lifetime::new("'lt0", Span::call_site()));
                self.visit_type_mut(r.elem.as_mut());
            }
            _ => (),
        }
    }
}

struct TightLifetimeAnnotatorFnArgHelper {}

impl VisitMut for TightLifetimeAnnotatorFnArgHelper {
    fn visit_fn_arg_mut(&mut self, i: &mut FnArg) {
        match i {
            FnArg::Receiver(_) => (), // don't modify receiver yet (&self)
            FnArg::Typed(t) => {
                let mut type_helper = TightLifetimeAnnotatorTypeHelper {};
                type_helper.visit_type_mut(t.ty.as_mut())
            }
        }
    }
}

struct TightLifetimeAnnotator<'a> {
    fn_name: &'a str,
    success: bool,
}

impl VisitMut for TightLifetimeAnnotator<'_> {
    fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
        let id = i.sig.ident.to_string();
        match id == self.fn_name.to_string() {
            false => (),
            true => match (&mut i.sig.inputs, &mut i.sig.generics, &mut i.sig.output) {
                (inputs, _, _) if inputs.len() == 0 => self.success = true,
                (_, gen, _)
                    if gen.params.iter().any(|x| match x {
                        syn::GenericParam::Lifetime(_) => true,
                        _ => false,
                    }) =>
                {
                    self.success = false
                }
                (inputs, gen, out) => {
                    let lifetime = Lifetime::new("'lt0", Span::call_site());
                    gen.params.push(syn::GenericParam::Lifetime(LifetimeDef {
                        attrs: vec![],
                        lifetime,
                        colon_token: None,
                        bounds: Default::default(),
                    }));
                    inputs
                        .iter_mut()
                        .map(|arg| {
                            let mut fn_arg_helper = TightLifetimeAnnotatorFnArgHelper {};
                            fn_arg_helper.visit_fn_arg_mut(arg)
                        })
                        .all(|_| true);
                    match out {
                        syn::ReturnType::Type(_, ty) => match ty.as_mut() {
                            Type::Reference(r) => {
                                r.lifetime = Some(Lifetime::new("'lt0", Span::call_site()))
                            }
                            _ => (),
                        },
                        _ => (),
                    };
                    self.success = true
                }
            },
        }
    }
}

pub fn annotate_tight_named_lifetime(new_file_name: &str, fn_name: &str) -> bool {
    let file_content: String = fs::read_to_string(&new_file_name).unwrap().parse().unwrap();
    let mut file = syn::parse_str::<syn::File>(file_content.as_str())
        .map_err(|e| format!("{:?}", e))
        .unwrap();
    let mut visit = TightLifetimeAnnotator {
        fn_name,
        success: false,
    };
    visit.visit_file_mut(&mut file);
    let file = file.into_token_stream().to_string();
    match visit.success {
        true => {
            fs::write(new_file_name.to_string(), format_source(&file)).unwrap();
            true
        }
        false => false,
    }
}

struct BoundsLoosener<'a> {
    fn_name: &'a str,
    arg_name: &'a str,
    success: bool,
}

struct ArgBoundLoosener<'a> {
    arg_name: &'a str,
    lt: &'a str,
    success: bool,
}

impl VisitMut for ArgBoundLoosener<'_> {
    fn visit_fn_arg_mut(&mut self, i: &mut FnArg) {
        match i {
            FnArg::Receiver(_) => (), // don't modify receiver yet (&self)
            FnArg::Typed(t) => match t.pat.as_mut() {
                syn::Pat::Ident(id) if id.ident.to_string() == self.arg_name => {
                    match t.ty.as_mut() {
                        Type::Reference(r) => {
                            r.lifetime = Some(Lifetime::new(self.lt, Span::call_site()));
                            self.success = true
                        }
                        _ => (),
                    }
                }
                _ => (),
            },
        }
    }
}

impl VisitMut for BoundsLoosener<'_> {
    fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
        let id = i.sig.ident.to_string();
        match id == self.fn_name.to_string() {
            false => (),
            true => {
                let mut lt_count = 0;
                let gen = &mut i.sig.generics;
                for i in &gen.params {
                    match i {
                        syn::GenericParam::Lifetime(LifetimeDef { .. }) => lt_count += 1,
                        _ => (),
                    }
                }
                let lt = format!("'lt{}", lt_count);
                let lifetime = Lifetime::new(lt.as_str(), Span::call_site());
                gen.params.push(syn::GenericParam::Lifetime(LifetimeDef {
                    attrs: vec![],
                    lifetime,
                    colon_token: None,
                    bounds: Default::default(),
                }));
                let mut arg_loosener = ArgBoundLoosener {
                    arg_name: self.arg_name,
                    lt: lt.as_str(),
                    success: false,
                };
                let inputs = &mut i.sig.inputs;
                inputs
                    .iter_mut()
                    .map(|arg| arg_loosener.visit_fn_arg_mut(arg))
                    .all(|_| true);
                match arg_loosener.success {
                    true => self.success = true,
                    false => (),
                }
            }
        }
    }
}

pub fn loosen_bounds(stderr: &Cow<str>, new_file_name: &str, fn_name: &str) -> bool {
    let binding = stderr.to_string();
    let deserializer = serde_json::Deserializer::from_str(binding.as_str());
    let stream = deserializer.into_iter::<CompilerError>();
    let mut helped = false;
    for item in stream {
        let rendered = item.unwrap().rendered;
        let reference_re = Regex::new(r"error.*`(?P<ref_full>\**(?P<ref>[a-z]+))`").unwrap();
        let error_lines = reference_re.captures_iter(rendered.as_str());

        for captured in error_lines {
            //println!("ref_full: {}, ref: {}", &captured["ref_full"], &captured["ref"]);
            let file_content: String = fs::read_to_string(&new_file_name).unwrap().parse().unwrap();
            let mut file = syn::parse_str::<syn::File>(file_content.as_str())
                .map_err(|e| format!("{:?}", e))
                .unwrap();
            let mut visit = BoundsLoosener {
                fn_name,
                arg_name: &captured["ref"],
                success: false,
            };
            visit.visit_file_mut(&mut file);
            let file = file.into_token_stream().to_string();
            match visit.success {
                true => {
                    fs::write(new_file_name.to_string(), format_source(&file)).unwrap();
                    helped = true
                }
                false => (),
            }
        }
    }
    helped
}
