use super::*;

#[track_caller]
fn test_link(link: &str, item: &str, result: &str) -> Result<()> {
    test_link_full(
        link,
        item,
        &format!("https://docs.rs/test_case/latest/test_case/{result}"),
    )
}

#[track_caller]
fn test_link_full(link: &str, item: &str, result: &str) -> Result<()> {
    test(Case {
        lib_rs: Some(&docstr!(format!
            /// //! {link}
            ///
            /// {item}
        )),
        generated: &docstr!(format!
            /// {link}({result})
        ),
        ..
    })
}

#[test]
fn kind_field() -> Result<()> {
    test_link(
        "[X::y]",
        "struct X { y: i32 }",
        "struct.X.html#structfield.y",
    )?;
    test_link("[X::y]", "union X { y: () }", "union.X.html#structfield.y")?;
    Ok(())
}

#[test]
fn kind_method() -> Result<()> {
    test_link(
        "[X::y]",
        "struct X; impl X { fn y(&self) {} }",
        "struct.X.html#method.y",
    )?;
    test_link(
        "[X::y]",
        "enum X { A } impl X { fn y(&self) {} }",
        "enum.X.html#method.y",
    )?;
    Ok(())
}

#[test]
fn trait_items() -> Result<()> {
    // The trait method definition
    test_link(
        "[X::y]",
        "trait X { fn y(&self); }",
        "trait.X.html#tymethod.y",
    )?;
    // An associated type within a trait
    test_link(
        "[X::Y]",
        "trait X { type Y; }",
        "trait.X.html#associatedtype.Y",
    )?;
    // An associated constant within a trait
    test_link(
        "[X::Y]",
        "trait X { const Y: i32; }",
        "trait.X.html#associatedconstant.Y",
    )?;
    Ok(())
}

#[test]
fn associated_fn() -> Result<()> {
    test_link(
        "[X::y]",
        "struct X; impl X { fn y() {} }",
        "struct.X.html#method.y",
    )?;
    Ok(())
}

#[test]
fn link_enum_variant() -> Result<()> {
    test_link("[E::A]", "enum E { A }", "enum.E.html#variant.A")?;
    Ok(())
}

#[test]
fn link_struct_field() -> Result<()> {
    test_link(
        "[S::f]",
        "struct S { f: i32 }",
        "struct.S.html#structfield.f",
    )?;
    Ok(())
}

#[test]
fn link_union_field() -> Result<()> {
    test_link("[U::f]", "union U { f: i32 }", "union.U.html#structfield.f")?;
    Ok(())
}

#[test]
fn link_method() -> Result<()> {
    test_link(
        "[S::m]",
        "struct S; impl S { fn m(&self) {} }",
        "struct.S.html#method.m",
    )?;
    Ok(())
}

#[test]
fn link_assoc_fn() -> Result<()> {
    test_link(
        "[S::f]",
        "struct S; impl S { fn f() {} }",
        "struct.S.html#method.f",
    )?;
    Ok(())
}

#[test]
fn link_assoc_const() -> Result<()> {
    test_link(
        "[S::C]",
        "struct S; impl S { const C: i32 = 1; }",
        "struct.S.html#associatedconstant.C",
    )?;
    Ok(())
}

#[test]
fn link_trait_method() -> Result<()> {
    test_link(
        "[T::m]",
        "trait T { fn m(&self); }",
        "trait.T.html#tymethod.m",
    )?;
    Ok(())
}

#[test]
fn link_trait_assoc_type() -> Result<()> {
    test_link(
        "[T::A]",
        "trait T { type A; }",
        "trait.T.html#associatedtype.A",
    )?;
    Ok(())
}

#[test]
fn resolve_ambiguity_fn() -> Result<()> {
    let code = "fn x() {} struct x {}";
    test_link("[fn@x]", code, "fn.x.html")?;
    Ok(())
}

#[test]
fn resolve_ambiguity_struct() -> Result<()> {
    let code = "fn x() {} struct x {}";
    test_link("[struct@x]", code, "struct.x.html")?;
    Ok(())
}

#[test]
fn resolve_ambiguity_macro() -> Result<()> {
    let code = "fn x() {} #[macro_export] macro_rules! x { () => {} }";
    test_link("[x!]", code, "macro.x.html")?;
    test_link("[macro@x]", code, "macro.x.html")?;
    Ok(())
}

#[test]
fn resolution_by_suffix() -> Result<()> {
    test_link("[x()]", "fn x() {}", "fn.x.html")?;
    test_link(
        "[x!]",
        "#[macro_export] macro_rules! x { () => {} }",
        "macro.x.html",
    )?;
    Ok(())
}

#[test]
fn link_module() -> Result<()> {
    test_link("[m]", "mod m {}", "m/")?;
    Ok(())
}

#[test]
fn link_nested_item() -> Result<()> {
    test_link("[m::x]", "mod m { pub fn x() {} }", "m/fn.x.html")?;
    Ok(())
}

#[test]
fn primitive_link() -> Result<()> {
    test_link_full(
        "[u32]",
        "",
        "https://doc.rust-lang.org/stable/std/primitive.u32.html",
    )?;
    test_link_full(
        "[slice]",
        "",
        "https://doc.rust-lang.org/stable/std/primitive.slice.html",
    )?;
    Ok(())
}

#[test]
fn relative_paths() -> Result<()> {
    test_link("[crate::x]", "fn x() {}", "fn.x.html")?;
    test_link("[self::x]", "fn x() {}", "fn.x.html")?;
    Ok(())
}

#[test]
fn tuple_struct_field() -> Result<()> {
    test_link("[S::0]", "struct S(i32);", "struct.S.html#structfield.0")?;
    Ok(())
}

#[test]
fn nested_assoc_item() -> Result<()> {
    let code = "mod m { pub struct S; impl S { pub fn f() {} } }";
    test_link("[m::S::f]", code, "m/struct.S.html#method.f")?;
    Ok(())
}

#[test]
fn generic_path_ignoring() -> Result<()> {
    test_link(
        "[Vec<T>::push]",
        "struct Vec<T> { a: [T] } impl<T> Vec<T> { fn push(&mut self, x: T) {} }",
        "struct.Vec.html#method.push",
    )?;
    Ok(())
}
