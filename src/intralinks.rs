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
    rustdoc_json: &'a Crate,
) -> IntralinkResolver<'a> {
    let root = rustdoc_json.index.get(&rustdoc_json.root).unwrap();

    let items_info = items_info(&rustdoc_json);
    let links_items_id = &root.links;

    let mut intralink_resolver = IntralinkResolver::new(&pkg.name, &config.docs_rs);
    for (link, item_id) in links_items_id {
        let link = Link {
            raw_link: link.clone(),
        };
        let Some(item_info) = items_info.get(item_id) else {
            // We will fail when we try to create the link and will emit a warning there.
            continue;
        };

        intralink_resolver.add(&link, item_info, &rustdoc_json.external_crates);
    }
    intralink_resolver
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct IntralinksDocsRsConfig {
    pub docs_rs_base_url: Option<String>,
    pub docs_rs_version: Option<String>,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct IntralinksConfig {
    pub docs_rs: IntralinksDocsRsConfig,
    pub strip_links: Option<bool>,
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

    pub fn url_segment(kind: ItemKind, name: &str) -> String {
        match kind {
            ItemKind::Module => format!("{name}/"),
            ItemKind::Struct => format!("struct.{name}.html"),
            ItemKind::StructField => format!("#structfield.{name}"),
            ItemKind::Union => format!("union.{name}.html"),
            ItemKind::Enum => format!("enum.{name}.html"),
            ItemKind::Variant => format!("#variant.{name}"),
            ItemKind::Function => format!("fn.{name}.html"),
            ItemKind::Method => format!("#method.{name}"),
            ItemKind::TyMethod => format!("#tymethod.{name}"),
            ItemKind::TypeAlias => format!("type.{name}.html"),
            ItemKind::Constant => format!("const.{name}.html"),
            ItemKind::Trait => format!("trait.{name}.html"),
            ItemKind::TraitAlias => format!("traitalias.{name}.html"),
            ItemKind::Static => format!("static.{name}.html"),
            ItemKind::Macro => format!("macro.{name}.html"),
            ItemKind::ProcAttribute => format!("attr.{name}.html"),
            ItemKind::ProcDerive => format!("derive.{name}.html"),
            ItemKind::AssocConst => {
                format!("#associatedconstant.{name}")
            }
            ItemKind::AssocType => format!("#associatedtype.{name}"),
            ItemKind::Primitive => format!("primitive.{name}.html"),

            ItemKind::Keyword
            | ItemKind::ExternCrate
            | ItemKind::Use
            | ItemKind::Impl
            | ItemKind::ExternType
            | ItemKind::Attribute => {
                unreachable!("items of kind {:?} cannot be intralinked to", kind);
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

        let path_segment_kind = |i: usize| match item_info.path.len() - i {
            1 => item_info.kind,
            2 => item_info.parent_kind.unwrap_or(ItemKind::Module),
            _ => ItemKind::Module,
        };
        let url_path = item_info
            .path
            .segments()
            .enumerate()
            .map(|(i, segment)| (segment, path_segment_kind(i)))
            .map(|(segment, item_kind)| IntralinkResolver::url_segment(item_kind, segment))
            .join("");

        let url = match item_info.crate_id {
            // Local crate has id 0.
            0 => {
                let version = self.config.docs_rs_version.as_deref().unwrap_or("latest");
                let package_name = &self.package_name;

                Self::make_url(docs_rs_base_url, package_name, version, &url_path)
            }
            // External crate
            _ => {
                let Some(external_crate) = external_crates.get(&item_info.crate_id) else {
                    return;
                };

                match external_crate.html_root_url.as_deref() {
                    Some(base_url) => {
                        let base_url = match Self::is_stdlib_crate(external_crate) {
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
                        Self::make_url(docs_rs_base_url, crate_name, "latest", &url_path)
                    }
                }
            }
        };

        self.link_url.insert(link.clone(), url);
    }

    pub fn resolve_link(&self, link: &Link) -> Option<&str> {
        self.link_url.get(link).map(String::as_str)
    }

    pub fn is_intralink(link: &Link) -> bool {
        let has_lone_colon = || link.raw_link.replace("::", "").contains(':');

        !link.symbol().is_empty() && !link.raw_link.contains('/') && !has_lone_colon()
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

    pub fn symbol(&self) -> &str {
        self.split_link_fragment().0
    }

    pub fn link_fragment(&self) -> Option<&str> {
        match self.split_link_fragment().1 {
            "" => None,
            f => Some(f),
        }
    }
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

#[derive(Debug, Clone)]
pub struct ItemInfo<'a> {
    pub crate_id: u32,
    pub path: ItemPath<'a>,
    pub kind: ItemKind,
    pub parent_kind: Option<ItemKind>,
}

#[derive(Clone, Copy, Debug)]
enum ItemContext {
    Normal,
    Impl,
    Trait,
}

impl<'a> ItemInfo<'a> {
    pub fn new(
        crate_id: u32,
        path: ItemPath<'a>,
        kind: ItemKind,
        parent_kind: Option<ItemKind>,
    ) -> ItemInfo<'a> {
        ItemInfo {
            crate_id,
            path,
            kind,
            parent_kind,
        }
    }

    pub fn from(
        item_summary: &'a ItemSummary,
        parent_kind: Option<ItemKind>,
        item_context: ItemContext,
    ) -> ItemInfo<'a> {
        ItemInfo::new(
            item_summary.crate_id,
            ItemPath::new(&item_summary.path),
            ItemKind::from_rustdoc_item_kind(item_summary.kind, item_context),
            parent_kind,
        )
    }

    /// Merges all the information of both items.
    pub fn merge(&self, other: &ItemInfo<'a>) -> Option<ItemInfo<'a>> {
        if self.crate_id != other.crate_id {
            return None;
        }
        if self.path != other.path {
            return None;
        }
        if self.kind != other.kind {
            return None;
        }

        if self
            .parent_kind
            .zip(other.parent_kind)
            .is_some_and(|(s, o)| s != o)
        {
            return None;
        }

        let merged = ItemInfo {
            crate_id: self.crate_id,
            path: self.path.clone(),
            kind: self.kind,
            parent_kind: self.parent_kind.or(other.parent_kind),
        };

        Some(merged)
    }
}

impl<'a> ItemPath<'a> {
    fn new(segments: &'a [String]) -> ItemPath<'a> {
        assert!(!segments.is_empty(), "path item must not be empty");

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

    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.segments.iter().map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.segments.len()
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

pub fn items_info(rustdoc_crate: &Crate) -> HashMap<Id, ItemInfo<'_>> {
    let mut items_info: HashMap<Id, ItemInfo<'_>> =
        HashMap::with_capacity(rustdoc_crate.index.len());

    for (&item_id, item_summary) in &rustdoc_crate.paths {
        let item_info = ItemInfo::from(item_summary, None, ItemContext::Normal);

        transitive_items(
            item_id,
            &item_info,
            ItemContext::Normal,
            rustdoc_crate,
            &mut items_info,
        );
    }

    items_info
}

fn transitive_items<'a>(
    item_id: Id,
    item_info: &ItemInfo<'a>,
    item_context: ItemContext,
    rustdoc_crate: &'a Crate,
    items_info: &mut HashMap<Id, ItemInfo<'a>>,
) {
    if item_info.kind != ItemKind::Impl {
        items_info
            .entry(item_id)
            .and_modify(|existing_item_info| {
                *existing_item_info = existing_item_info
                    .merge(item_info)
                    .expect("unmergeable item info");
            })
            .or_insert_with(|| item_info.clone());
    }

    let Some(item) = rustdoc_crate.index.get(&item_id) else {
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

        let inner_item_info = get_item_info(
            inner_item_id,
            &item_info.path,
            inner_item_parent_kind,
            inner_item_context,
            rustdoc_crate,
        );

        if let Some(inner_item_info) = inner_item_info {
            transitive_items(
                inner_item_id,
                &inner_item_info,
                inner_item_context,
                rustdoc_crate,
                items_info,
            );
        }
    }
}

fn child_item_ids<'a>(item: &'a Item) -> Box<dyn Iterator<Item = Id> + 'a> {
    match &item.inner {
        ItemEnum::Struct(Struct { kind, impls, .. }) => {
            let fields_ids: Box<dyn Iterator<Item = Id>> = match kind {
                StructKind::Unit => Box::new(std::iter::empty()),
                StructKind::Tuple(ids) => Box::new(ids.iter().copied().flatten()),
                StructKind::Plain { fields, .. } => Box::new(fields.iter().copied()),
            };

            Box::new(fields_ids.chain(impls.iter().copied()))
        }
        ItemEnum::Impl(Impl {
            trait_: Some(_), ..
        }) => Box::new(std::iter::empty()),
        ItemEnum::Impl(Impl {
            items: item_ids,
            for_,
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
        ItemEnum::Trait(Trait { items, .. }) => {
            // We ignore the implementations of the trait as their items are not part of the trait
            // itself.
            Box::new(items.iter().copied())
        }

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

fn get_item_info<'a>(
    item_id: Id,
    parent_path: &ItemPath<'a>,
    parent_kind: Option<ItemKind>,
    item_context: ItemContext,
    rustdoc_crate: &'a Crate,
) -> Option<ItemInfo<'a>> {
    match rustdoc_crate.paths.get(&item_id) {
        Some(item_summary) => Some(ItemInfo::from(item_summary, parent_kind, item_context)),
        None => rustdoc_crate.index.get(&item_id).map(|item| {
            let path = match item.name.as_ref() {
                None => parent_path.clone(),
                Some(name) => parent_path.add(name.clone()),
            };
            let item_kind = ItemKind::of_item(item, item_context);

            ItemInfo::new(item.crate_id, path, item_kind, parent_kind)
        }),
    }
}

/// If this markdown fence language can be considered to be a "rust" language
///
/// All attributes: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes
pub fn is_rust_code_block(tags: &str) -> bool {
    tags.split(',').all(|tag| {
        tag.is_empty()
            || matches!(
                tag,
                "should_panic"
                    | "no_run"
                    | "ignore"
                    | "allow_fail"
                    | "rust"
                    | "rs"
                    | "test_harness"
                    | "standalone_crate"
                    | "compile_fail"
            )
            || tag.starts_with("ignore-")
            || tag.starts_with("edition")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_rust_code_block() {
        let pass = [
            "ignore",
            "should_panic",
            "no_run",
            "compile_fail",
            "edition2018",
            "rust",
            "rs",
            "standalone_crate",
            "ignore-x86_64",
            "ignore-x86_64,ignore-windows",
            "ignore,ignore-x86_64",
        ]
        .into_iter()
        .all(super::is_rust_code_block);

        assert!(pass);
    }

    #[test]
    fn is_rust_code_block_fail() {
        let fail = ["py", "js", "ts", "toml"]
            .into_iter()
            .all(|s| !super::is_rust_code_block(s));

        assert!(fail);
    }
}
