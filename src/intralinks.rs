//! This module has in large part been taken from `cargo-rdme` from the "rustdoc-json" branch,
//! including modifications
//!
//! Original Author: Diogo Sousa
//! PR: https://github.com/orium/cargo-rdme/pull/236
//! Commit: c0d579139660b4bf65334b86bd36580fcff8db84
//! Repo: https://github.com/orium/cargo-rdme
//! License: MIT
//!
//! MIT License
//!
//! Copyright (c) 2025 Diogo Sousa
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use core::fmt;
use std::{borrow::Cow, collections::HashMap, fmt::Display};

use cargo_metadata::Package;
use itertools::Itertools;
use rustdoc_types::{
    Crate, Enum, ExternalCrate, Id, Impl, Item, ItemEnum, ItemSummary, MacroKind, Primitive,
    ProcMacro, Struct, StructKind, Trait, Type, Union,
};
use serde::{Deserialize, Serialize};

use crate::Config;

pub fn create_intralink_resolver<'a>(
    pkg: &'a Package,
    config: &'a Config,
    krate: &'a Crate,
) -> IntralinkResolver<'a> {
    let root = krate
        .index
        .get(&krate.root)
        .expect("root crate is a valid item");

    let items_info = items_info(krate);

    let mut intralink_resolver = IntralinkResolver::new(&pkg.name, &config.docs_rs);
    for (link, item_id) in &root.links {
        let link = Link {
            raw_link: link.clone(),
        };
        let Some(item_info) = items_info.get(item_id) else {
            // We will fail when we try to create the link and will emit a warning there.
            continue;
        };

        intralink_resolver.add(&link, item_info, &krate.external_crates);
    }
    intralink_resolver
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct IntralinksDocsRsConfig {
    pub docs_rs_base_url: Option<String>,
    pub docs_rs_version: Option<String>,
}

#[derive(Debug)]
pub struct IntralinkResolver<'a> {
    link_url: HashMap<Link, String>,
    config: &'a IntralinksDocsRsConfig,
    package_name: &'a str,
}

impl<'a> IntralinkResolver<'a> {
    pub fn new(package_name: &'a str, config: &'a IntralinksDocsRsConfig) -> IntralinkResolver<'a> {
        IntralinkResolver {
            link_url: HashMap::new(),
            package_name,
            config,
        }
    }

    pub fn add(
        &mut self,
        link: &Link,
        item_info: &ItemInfo,
        external_crates: &HashMap<u32, ExternalCrate>,
    ) {
        let docs_rs_base_url = self
            .config
            .docs_rs_base_url
            .as_deref()
            .unwrap_or("https://docs.rs");

        let url_path = item_info
            .path
            .segments
            .iter()
            .enumerate()
            .map(|(segments_before, segment)| {
                // How many segments needed to complete the path?
                //
                //  foo::bar
                //          ^ 0
                //  foo::bar
                //      ^ 1
                //  foo::bar
                // ^ 2
                let segments_remaining = item_info.path.segments.len() - 1 - segments_before;

                let item_kind = match segments_remaining {
                    // Reached the final component of the path
                    //
                    // foo::current
                    //      ^^^^^^^ we are here
                    0 => item_info.kind,
                    // 2nd component of the path
                    //
                    // foo::bar::current
                    //      ^^^ we are here
                    1 => item_info.parent_kind.unwrap_or(ItemKind::Module),
                    // Other components of the path
                    //
                    // foo::bar::current
                    // ^^^ we are here
                    _ => ItemKind::Module,
                };

                fmt::from_fn(move |f| item_kind.url_segment(segment, f))
            })
            .join("");

        let url = match item_info.crate_id {
            // Local crate has id 0.
            0 => {
                let version = self.config.docs_rs_version.as_deref().unwrap_or("latest");
                let package_name = &self.package_name;

                make_url(docs_rs_base_url, package_name, version, &url_path)
            }
            // External crate
            _ => {
                let Some(external_crate) = external_crates.get(&item_info.crate_id) else {
                    return;
                };

                match external_crate.html_root_url.as_deref() {
                    Some(base_url) => {
                        let base_url = match is_stdlib_crate(external_crate) {
                            true => {
                                // TODO Once we are able to use the stable version we can remove this
                                //      (https://github.com/rust-lang/rust/issues/76578).
                                base_url
                                    .strip_suffix("/nightly/")
                                    .map_or_else(|| base_url.to_owned(), |p| format!("{p}/stable/"))
                            }
                            false => base_url.to_owned(),
                        };

                        format!("{base_url}{url_path}")
                    }
                    None => {
                        let crate_name = &external_crate.name;

                        // TODO We are using the crate name instead of the package name: that means that
                        //      we might generate a wrong url. In most cases the crate name matches the
                        //      package name. When it doesn't it is often because underscores in the
                        //      crate name becomes dashes in the package name. Fortunately `docs.rs`
                        //      will redirect in that case (e.g. https://docs.rs/tower_service/ will
                        //      redirect to https://docs.rs/tower-service/latest/tower_service/).
                        // TODO We shouldn't hardcode "latest" here: we should get that information from
                        //      the version rustdoc determined the crate was using.
                        make_url(docs_rs_base_url, crate_name, "latest", &url_path)
                    }
                }
            }
        };

        self.link_url.insert(link.clone(), url);
    }

    pub fn resolve_link(&self, link: &Link) -> Option<&str> {
        self.link_url.get(link).map(String::as_str)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Link {
    pub raw_link: String,
}

impl Link {
    fn split_link_fragment(&self) -> (&str, &str) {
        fn strip_last_backtick(strip_backtick_end: bool, s: &str) -> &str {
            match strip_backtick_end {
                true => s.strip_suffix('`').unwrap_or(s),
                false => s,
            }
        }

        let strip_backtick_end: bool = self.raw_link.starts_with('`');
        let link = self.raw_link.strip_prefix('`').unwrap_or(&self.raw_link);

        match link.find('#') {
            None => (strip_last_backtick(strip_backtick_end, link), ""),
            Some(i) => {
                let (l, f) = link.split_at(i);
                (
                    strip_last_backtick(strip_backtick_end, l),
                    strip_last_backtick(strip_backtick_end, &f[1..]),
                )
            }
        }
    }

    pub fn link_fragment(&self) -> Option<&str> {
        match self.split_link_fragment().1 {
            "" => None,
            f => Some(f),
        }
    }
}

fn is_stdlib_crate(external_crate: &ExternalCrate) -> bool {
    external_crate
        .html_root_url
        .as_deref()
        .is_some_and(|base_url| base_url.starts_with("https://doc.rust-lang.org/"))
}

fn make_url(base_url: &str, package_name: &str, version: &str, url_path: &str) -> String {
    format!("{base_url}/{package_name}/{version}/{url_path}")
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ItemKind {
    Module,
    ExternCrate,
    Use,
    Struct,
    StructField,
    Union,
    Enum,
    Variant,
    Function,
    TypeAlias,
    Constant,
    Trait,
    TraitAlias,
    Impl,
    Static,
    ExternType,
    Macro,
    ProcAttribute,
    ProcDerive,
    AssocConst,
    AssocType,
    Primitive,
    Keyword,
    Attribute,

    // Kinds that do not exist in rustdoc_types::ItemKind:
    Method,
    TyMethod,
}

impl ItemKind {
    pub fn url_segment(self, name: &str, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = match self {
            Self::Module => format_args!("{name}/"),
            Self::Struct => format_args!("struct.{name}.html"),
            Self::StructField => format_args!("#structfield.{name}"),
            Self::Union => format_args!("union.{name}.html"),
            Self::Enum => format_args!("enum.{name}.html"),
            Self::Variant => format_args!("#variant.{name}"),
            Self::Function => format_args!("fn.{name}.html"),
            Self::Method => format_args!("#method.{name}"),
            Self::TyMethod => format_args!("#tymethod.{name}"),
            Self::TypeAlias => format_args!("type.{name}.html"),
            Self::Constant => format_args!("const.{name}.html"),
            Self::Trait => format_args!("trait.{name}.html"),
            Self::TraitAlias => format_args!("traitalias.{name}.html"),
            Self::Static => format_args!("static.{name}.html"),
            Self::Macro => format_args!("macro.{name}.html"),
            Self::ProcAttribute => format_args!("attr.{name}.html"),
            Self::ProcDerive => format_args!("derive.{name}.html"),
            Self::AssocConst => format_args!("#associatedconstant.{name}"),
            Self::AssocType => format_args!("#associatedtype.{name}"),
            Self::Primitive => format_args!("primitive.{name}.html"),

            Self::Keyword
            | Self::ExternCrate
            | Self::Use
            | Self::Impl
            | Self::ExternType
            | Self::Attribute => {
                unreachable!("items of kind {self:?} cannot be intralinked to");
            }
        };

        f.write_fmt(fmt)
    }

    fn from_rustdoc_item_kind(
        kind: rustdoc_types::ItemKind,
        item_context: ItemContext,
    ) -> ItemKind {
        match kind {
            rustdoc_types::ItemKind::Module => ItemKind::Module,
            rustdoc_types::ItemKind::ExternCrate => ItemKind::ExternCrate,
            rustdoc_types::ItemKind::Use => ItemKind::Use,
            rustdoc_types::ItemKind::Struct => ItemKind::Struct,
            rustdoc_types::ItemKind::StructField => ItemKind::StructField,
            rustdoc_types::ItemKind::Union => ItemKind::Union,
            rustdoc_types::ItemKind::Enum => ItemKind::Enum,
            rustdoc_types::ItemKind::Variant => ItemKind::Variant,
            rustdoc_types::ItemKind::Function => match item_context {
                ItemContext::Normal => ItemKind::Function,
                ItemContext::Impl => ItemKind::Method,
                ItemContext::Trait => ItemKind::TyMethod,
            },
            rustdoc_types::ItemKind::TypeAlias => ItemKind::TypeAlias,
            rustdoc_types::ItemKind::Constant => ItemKind::Constant,
            rustdoc_types::ItemKind::Trait => ItemKind::Trait,
            rustdoc_types::ItemKind::TraitAlias => ItemKind::TraitAlias,
            rustdoc_types::ItemKind::Impl => ItemKind::Impl,
            rustdoc_types::ItemKind::Static => ItemKind::Static,
            rustdoc_types::ItemKind::ExternType => ItemKind::ExternType,
            rustdoc_types::ItemKind::Macro => ItemKind::Macro,
            rustdoc_types::ItemKind::ProcAttribute => ItemKind::ProcAttribute,
            rustdoc_types::ItemKind::ProcDerive => ItemKind::ProcDerive,
            rustdoc_types::ItemKind::AssocConst => ItemKind::AssocConst,
            rustdoc_types::ItemKind::AssocType => ItemKind::AssocType,
            rustdoc_types::ItemKind::Primitive => ItemKind::Primitive,
            rustdoc_types::ItemKind::Keyword => ItemKind::Keyword,
            rustdoc_types::ItemKind::Attribute => ItemKind::Attribute,
        }
    }

    fn of_item(item: &Item, item_context: ItemContext) -> ItemKind {
        match item.inner {
            ItemEnum::Module(_) => ItemKind::Module,
            ItemEnum::ExternCrate { .. } => ItemKind::ExternCrate,
            ItemEnum::Use(_) => ItemKind::Use,
            ItemEnum::Union(_) => ItemKind::Union,
            ItemEnum::Struct(_) => ItemKind::Struct,
            ItemEnum::StructField(_) => ItemKind::StructField,
            ItemEnum::Enum(_) => ItemKind::Enum,
            ItemEnum::Variant(_) => ItemKind::Variant,
            ItemEnum::Function(_) => match item_context {
                ItemContext::Normal => ItemKind::Function,
                ItemContext::Impl => ItemKind::Method,
                ItemContext::Trait => ItemKind::TyMethod,
            },
            ItemEnum::Trait(_) => ItemKind::Trait,
            ItemEnum::TraitAlias(_) => ItemKind::TraitAlias,
            ItemEnum::Impl(_) => ItemKind::Impl,
            ItemEnum::TypeAlias(_) => ItemKind::TypeAlias,
            ItemEnum::Constant { .. } => ItemKind::Constant,
            ItemEnum::Static(_) => ItemKind::Static,
            ItemEnum::ExternType => ItemKind::ExternType,
            ItemEnum::Macro(_) => ItemKind::Macro,
            ItemEnum::ProcMacro(ProcMacro {
                kind: MacroKind::Bang,
                ..
            }) => ItemKind::Macro,
            ItemEnum::ProcMacro(ProcMacro {
                kind: MacroKind::Derive,
                ..
            }) => ItemKind::ProcDerive,
            ItemEnum::ProcMacro(ProcMacro {
                kind: MacroKind::Attr,
                ..
            }) => ItemKind::ProcAttribute,
            ItemEnum::Primitive(_) => ItemKind::Primitive,
            ItemEnum::AssocConst { .. } => ItemKind::AssocConst,
            ItemEnum::AssocType { .. } => ItemKind::AssocType,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ItemContext {
    Normal,
    Impl,
    Trait,
}

#[derive(Debug, Clone)]
pub struct ItemInfo<'a> {
    pub crate_id: u32,
    pub path: ItemPath<'a>,
    pub kind: ItemKind,
    pub parent_kind: Option<ItemKind>,
}

impl<'a> ItemInfo<'a> {
    pub fn new(
        item_summary: &'a ItemSummary,
        parent_kind: Option<ItemKind>,
        item_context: ItemContext,
    ) -> ItemInfo<'a> {
        ItemInfo {
            crate_id: item_summary.crate_id,
            path: ItemPath::new(&item_summary.path),
            kind: ItemKind::from_rustdoc_item_kind(item_summary.kind, item_context),
            parent_kind,
        }
    }
}

impl<'a> ItemPath<'a> {
    fn new(segments: &'a [String]) -> ItemPath<'a> {
        debug_assert!(!segments.is_empty(), "path item must not be empty");

        ItemPath {
            segments: Cow::Borrowed(segments),
        }
    }

    fn add(&self, segment: String) -> ItemPath<'static> {
        let mut segments = self.segments.clone().into_owned();

        segments.push(segment);

        ItemPath {
            segments: Cow::Owned(segments),
        }
    }
}

impl Display for ItemPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iter = Itertools::intersperse(self.segments.iter().map(String::as_str), "::");

        for s in iter {
            f.write_str(s)?;
        }

        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ItemPath<'a> {
    pub segments: Cow<'a, [String]>,
}

pub fn items_info(krate: &Crate) -> HashMap<Id, ItemInfo<'_>> {
    let mut items_info: HashMap<Id, ItemInfo<'_>> = HashMap::with_capacity(krate.index.len());

    for (&item_id, item_summary) in &krate.paths {
        let item_info = ItemInfo::new(item_summary, None, ItemContext::Normal);

        transitive_items(
            item_id,
            &item_info,
            ItemContext::Normal,
            krate,
            &mut items_info,
        );
    }

    items_info
}

fn transitive_items<'a>(
    item_id: Id,
    item_info: &ItemInfo<'a>,
    item_context: ItemContext,
    krate: &'a Crate,
    items_info: &mut HashMap<Id, ItemInfo<'a>>,
) {
    if item_info.kind != ItemKind::Impl {
        items_info
            .entry(item_id)
            .and_modify(|existing_item_info| {
                if let Some(parent_kind) = item_info.parent_kind
                    && existing_item_info.parent_kind.is_none()
                {
                    existing_item_info.parent_kind = Some(parent_kind);
                }
            })
            .or_insert_with(|| item_info.clone());
    }

    let Some(item) = krate.index.get(&item_id) else {
        // This item is not in the index for some reason...
        return;
    };

    let inner_item_context = match item.inner {
        ItemEnum::Trait(_) => ItemContext::Trait,
        ItemEnum::Impl(_) => ItemContext::Impl,
        _ => item_context,
    };

    for inner_item_id in child_item_ids(item) {
        // The inner_item_parent_kind is not just `item_info.kind` because we need to skip
        // kinds like `impl` blocks.
        let inner_item_parent_kind = match item.name {
            Some(_) => Some(item_info.kind),
            None => item_info.parent_kind,
        };

        let inner_item_info = {
            let parent_path: &ItemPath<'a> = &item_info.path;
            match krate.paths.get(&inner_item_id) {
                Some(item_summary) => Some(ItemInfo::new(
                    item_summary,
                    inner_item_parent_kind,
                    inner_item_context,
                )),
                None => krate.index.get(&inner_item_id).map(|item| {
                    let path = match item.name.as_ref() {
                        None => parent_path.clone(),
                        Some(name) => parent_path.add(name.clone()),
                    };
                    let item_kind = ItemKind::of_item(item, inner_item_context);

                    ItemInfo {
                        crate_id: item.crate_id,
                        path,
                        kind: item_kind,
                        parent_kind: inner_item_parent_kind,
                    }
                }),
            }
        };

        if let Some(inner_item_info) = inner_item_info {
            transitive_items(
                inner_item_id,
                &inner_item_info,
                inner_item_context,
                krate,
                items_info,
            );
        }
    }
}

fn child_item_ids<'a>(item: &'a Item) -> Box<dyn Iterator<Item = Id> + 'a> {
    match &item.inner {
        ItemEnum::Struct(Struct { kind, impls, .. }) => {
            let fields: Box<dyn Iterator<Item = Id>> = match kind {
                StructKind::Unit => Box::new(std::iter::empty()),
                StructKind::Tuple(fields) => Box::new(fields.iter().copied().flatten()),
                StructKind::Plain { fields, .. } => Box::new(fields.iter().copied()),
            };

            Box::new(fields.chain(impls.iter().copied()))
        }
        ItemEnum::Impl(Impl {
            trait_: Some(_), ..
        }) => Box::new(std::iter::empty()),
        ItemEnum::Impl(Impl {
            trait_: None,
            for_,
            items: item_ids,
            ..
        }) => match for_ {
            Type::ResolvedPath(_) => Box::new(item_ids.iter().copied()),
            _ => Box::new(std::iter::empty()),
        },
        ItemEnum::Union(Union { fields, impls, .. }) => {
            Box::new(fields.iter().chain(impls.iter()).copied())
        }
        ItemEnum::Enum(Enum {
            variants, impls, ..
        }) => Box::new(variants.iter().chain(impls.iter()).copied()),
        ItemEnum::Primitive(Primitive { impls, .. }) => Box::new(impls.iter().copied()),
        ItemEnum::Trait(Trait { items, .. }) => Box::new(items.iter().copied()),

        ItemEnum::Function(_)
        | ItemEnum::ExternCrate { .. }
        | ItemEnum::Use(_)
        | ItemEnum::Module(_)
        | ItemEnum::Constant { .. }
        | ItemEnum::Static(_)
        | ItemEnum::Macro(_)
        | ItemEnum::ProcMacro(_)
        | ItemEnum::AssocConst { .. }
        | ItemEnum::AssocType { .. }
        | ItemEnum::StructField(_)
        | ItemEnum::Variant(_)
        | ItemEnum::ExternType
        | ItemEnum::TraitAlias(_)
        | ItemEnum::TypeAlias(_) => Box::new(std::iter::empty()),
    }
}
