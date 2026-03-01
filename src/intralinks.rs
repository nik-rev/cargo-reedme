//! Resolves rustdoc intra-doc links
//!
//! This module has in large part been taken from `cargo-rdme` from the "rustdoc-json" branch,
//! including modifications
//!
//! Original Author: Diogo Sousa
//! PR: <https://github.com/orium/cargo-rdme/pull/236>
//! Commit: c0d579139660b4bf65334b86bd36580fcff8db84
//! Repo: <https://github.com/orium/cargo-rdme>
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
use eyre::Context;
use itertools::Itertools;
use rustdoc_types::{
    Crate, Enum, ExternalCrate, Id, Impl, Item, ItemEnum, ItemSummary, MacroKind, Primitive,
    ProcMacro, Struct, StructKind, Trait, Type, Union,
};
use tracing::{error, trace, warn};

use crate::{Config, World};

/// This maps link contents to link URLs.
///
/// For example, for this link:
///
/// ```md
/// [clone function](core::clone::Clone::clone)
/// ```
///
/// This map will contain:
///
/// ```ignore
/// {
///     "clone function": "https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone"
/// }
/// ```
pub type Links<'a> = HashMap<&'a str, String>;

/// Creates a map from link contents to link URLs
pub fn create_links<'a>(
    world: &World,
    pkg: &Package,
    config: &Config,
    krate: &'a Crate,
) -> Links<'a> {
    let root = krate
        .index
        .get(&krate.root)
        .expect("root crate is a valid item");

    let items_info = collect_all_items_info(krate);

    let mut links = HashMap::new();

    let mut rustdoc_html = None;

    let version = if config.use_latest_version {
        "latest".into()
    } else {
        pkg.version.to_string()
    };

    for (link, item_id) in &root.links {
        let Some(item_info) = items_info.get(item_id) else {
            // Note: Only actually compute rustdoc HTML if we have a broken link
            let rustdoc_html = rustdoc_html.get_or_insert_with(|| {
                match (world.rustdoc_html_for_crate)(&pkg.name, &world.toolchain)
                    .context("failed to generate rustdoc HTML")
                {
                    Ok(html) => html,
                    Err(err) => {
                        eprintln!("{err:#?}");
                        String::new()
                    }
                }
            });

            if let Some(href) = extract_rustdoc_link(link, rustdoc_html) {
                links.insert(
                    link.as_str(),
                    format!("{}/{1}/latest/{1}/{href}", config.base_url, pkg.name),
                );
                warn!(
                    item = link,
                    "failed to generate link; naively extracted from HTML as a last resort"
                );
            } else {
                error!(item = link, "failed to generate link");
            }

            continue;
        };

        let url = fmt::from_fn(|f| {
            item_info.url(
                f,
                &krate.external_crates,
                &config.base_url,
                &pkg.name,
                &version,
            )
        })
        .to_string();

        links.insert(link.as_str(), url);
    }

    links
}

/// Extracts link fragment from the link
///
/// ```text
/// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push
///                                                   ^^^^^^^^^^^
/// ```
pub fn link_fragment(link: &str) -> Option<&str> {
    link.strip_prefix('`')
        .unwrap_or(link)
        .find('#')
        .map(|i| link.split_at(i).1[1..].as_ref())
}

/// Whether this crate is part of the standard library
///
/// This is `true` for the `std`, `core`, `alloc` and `proc_macro` crates,
/// as well as internal crates such as `std_detect` or `test`
fn is_stdlib_crate(external_crate: &ExternalCrate) -> bool {
    external_crate
        .html_root_url
        .as_deref()
        .is_some_and(|base_url| base_url.starts_with("https://doc.rust-lang.org/"))
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ItemKind {
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
    fn url_segment(self, name: &str, f: &mut fmt::Formatter) -> fmt::Result {
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

    fn from_rustdoc_item_parent(
        kind: rustdoc_types::ItemKind,
        item_context: Option<ItemParent>,
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
                Some(ItemParent::Impl) => ItemKind::Method,
                Some(ItemParent::Trait) => ItemKind::TyMethod,
                None => ItemKind::Function,
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

    fn of_item(item: &Item, item_parent: Option<ItemParent>) -> ItemKind {
        match item.inner {
            ItemEnum::Module(_) => ItemKind::Module,
            ItemEnum::ExternCrate { .. } => ItemKind::ExternCrate,
            ItemEnum::Use(_) => ItemKind::Use,
            ItemEnum::Union(_) => ItemKind::Union,
            ItemEnum::Struct(_) => ItemKind::Struct,
            ItemEnum::StructField(_) => ItemKind::StructField,
            ItemEnum::Enum(_) => ItemKind::Enum,
            ItemEnum::Variant(_) => ItemKind::Variant,
            ItemEnum::Function(_) => match item_parent {
                Some(ItemParent::Impl) => ItemKind::Method,
                Some(ItemParent::Trait) => ItemKind::TyMethod,
                None => ItemKind::Function,
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
enum ItemParent {
    Impl,
    Trait,
}

/// Information about a single item
///
/// This information is used to compute the HTML URL to the
/// item's documentation
#[derive(Debug, Clone)]
struct ItemInfo<'a> {
    /// The crate that this item comes from
    crate_id: u32,
    /// Path to the item.
    ///
    /// For [`std::clone::Clone`], this will be ["std", "clone", "Clone"]
    path: ItemPath<'a>,
    /// Kind of item (e.g. function, method, module..)
    kind: ItemKind,
    /// Kind of item's parent. This is only `Some` if the
    /// item has a parent, for example: methods's parent is their impl block
    parent_kind: Option<ItemKind>,
}

impl<'a> ItemInfo<'a> {
    fn new(
        item_summary: &'a ItemSummary,
        parent_kind: Option<ItemKind>,
        item_parent: Option<ItemParent>,
    ) -> ItemInfo<'a> {
        ItemInfo {
            crate_id: item_summary.crate_id,
            path: ItemPath::new(&item_summary.path),
            kind: ItemKind::from_rustdoc_item_parent(item_summary.kind, item_parent),
            parent_kind,
        }
    }

    /// If this item comes from the current crate, NOT any external crates
    fn is_from_current_crate(&self) -> bool {
        // Local crate has id 0.
        self.crate_id == 0
    }

    /// Creates HTML URL to this item's documentation
    fn url(
        &self,
        f: &mut fmt::Formatter,
        external_crates: &HashMap<u32, ExternalCrate>,
        base_url: &str,
        package_name: &str,
        package_version: &str,
    ) -> fmt::Result {
        if self.is_from_current_crate() {
            f.write_fmt(format_args!("{base_url}/{package_name}/{package_version}/"))?;
            self.url_path(f)?;
        }
        // External crate
        else if let Some(external_crate) = external_crates.get(&self.crate_id) {
            match external_crate.html_root_url.as_deref() {
                Some(base_url) => {
                    if is_stdlib_crate(external_crate) {
                        // TODO Once we are able to use the stable version we can remove this
                        //      (https://github.com/rust-lang/rust/issues/76578).
                        if let Some(base_url) = base_url.strip_suffix("/nightly/") {
                            f.write_fmt(format_args!("{base_url}/stable/"))?;
                        } else {
                            f.write_str(base_url)?;
                        }
                    } else {
                        f.write_str(base_url)?;
                    }
                    self.url_path(f)?;
                }
                None => {
                    // TODO We are using the crate name instead of the package name: that means that
                    //      we might generate a wrong url. In most cases the crate name matches the
                    //      package name. When it doesn't it is often because underscores in the
                    //      crate name becomes dashes in the package name. Fortunately `docs.rs`
                    //      will redirect in that case (e.g. https://docs.rs/tower_service/ will
                    //      redirect to https://docs.rs/tower-service/latest/tower_service/).
                    // TODO We shouldn't hardcode "latest" here: we should get that information from
                    //      the version rustdoc determined the crate was using.
                    let version: &str = "latest";
                    f.write_fmt(format_args!(
                        "{base_url}/{}/{version}/",
                        external_crate.name
                    ))?;
                    self.url_path(f)?;
                }
            }
        } else {
            error!(
                item = ?self,
                "failed to determine which crate this item belongs to",
            );
        };

        Ok(())
    }

    /// Path to this item in the URL
    fn url_path(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (segments_before, segment) in self.path.segments.iter().enumerate() {
            // How many segments needed to complete the path?
            //
            //  foo::bar
            //          ^ 0
            //  foo::bar
            //      ^ 1
            //  foo::bar
            // ^ 2
            let segments_remaining = self.path.segments.len() - 1 - segments_before;

            let item_kind = match segments_remaining {
                // Reached the final component of the path
                //
                // foo::current
                //      ^^^^^^^ we are here
                0 => self.kind,
                // 2nd component of the path
                //
                // foo::bar::current
                //      ^^^ we are here
                1 => self.parent_kind.unwrap_or(ItemKind::Module),
                // Other components of the path
                //
                // foo::bar::current
                // ^^^ we are here
                _ => ItemKind::Module,
            };

            item_kind.url_segment(segment, f)?;
        }

        Ok(())
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
struct ItemPath<'a> {
    segments: Cow<'a, [String]>,
}

/// Collects information about every item from the crate, which
/// can then be used to generate a URL to the item's page
fn collect_all_items_info(krate: &Crate) -> HashMap<Id, ItemInfo<'_>> {
    let mut items_info: HashMap<Id, ItemInfo<'_>> = HashMap::with_capacity(krate.index.len());

    for (&item_id, item_summary) in &krate.paths {
        let item_info = ItemInfo::new(item_summary, None, None);

        transitive_items(item_id, &item_info, None, krate, &mut items_info);
    }

    items_info
}

fn transitive_items<'a>(
    item_id: Id,
    item_info: &ItemInfo<'a>,
    item_parent: Option<ItemParent>,
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
        trace!("item ID not found in the crate index: `{item_id:?}`");
        return;
    };

    let item_parent = match item.inner {
        ItemEnum::Trait(_) => Some(ItemParent::Trait),
        ItemEnum::Impl(_) => Some(ItemParent::Impl),
        _ => item_parent,
    };

    for item_id in child_item_ids(item) {
        // Not just `item_info.kind` because we need to skip kinds like `impl` blocks.
        let item_parent_kind = match item.name {
            Some(_) => Some(item_info.kind),
            None => item_info.parent_kind,
        };

        let parent_path: &ItemPath<'a> = &item_info.path;
        let item_info = match krate.paths.get(&item_id) {
            Some(item_summary) => Some(ItemInfo::new(item_summary, item_parent_kind, item_parent)),
            None => krate.index.get(&item_id).map(|item| {
                let path = match item.name.as_ref() {
                    None => parent_path.clone(),
                    Some(name) => parent_path.add(name.clone()),
                };
                let item_kind = ItemKind::of_item(item, item_parent);

                ItemInfo {
                    crate_id: item.crate_id,
                    path,
                    kind: item_kind,
                    parent_kind: item_parent_kind,
                }
            }),
        };

        if let Some(inner_item_info) = item_info {
            transitive_items(item_id, &inner_item_info, item_parent, krate, items_info);
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

/// This does a naive extraction of the href= value of a link
/// with the given markdown `link` content
///
/// This should only be used as a last resort, when it is impossible
/// to figure out the link URL from rustdoc JSON!
fn extract_rustdoc_link(link: &str, rustdoc_html: &str) -> Option<String> {
    let link = link
        .strip_prefix("`")
        .and_then(|link| link.strip_suffix("`"))
        .map(|link| format!("<code>{link}</code>"))
        .unwrap_or_else(|| link.into());

    let unknown_link_content_start = rustdoc_html.find(&link)?;

    let link_start_token = "<a href=\"";
    let unknown_link_start = rustdoc_html[..unknown_link_content_start].rfind(link_start_token)?;

    let rustdoc_html = &rustdoc_html[unknown_link_start + link_start_token.len()..];

    Some(rustdoc_html.chars().take_while(|ch| *ch != '"').collect())
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_rustdoc_link() {
        assert_eq!(
            super::extract_rustdoc_link(
                "`SHAPE`",
                r#"<p>facet provides reflection for Rust: it gives types a <a href="trait.Facet.html#associatedconstant.SHAPE" title="associated constant facet::Facet::SHAPE"><code>SHAPE</code></a> associated"#,
            ).unwrap(),
            "trait.Facet.html#associatedconstant.SHAPE"
        );
    }
}
