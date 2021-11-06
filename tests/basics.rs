use mlua_derive_unofficial::{ToLua, FromLua};

use mlua::{ToLua,FromLua};

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
struct Named {
    is_rusty: bool,
}

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
struct Unnamed(u8, String);

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
struct Unit;

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
enum NumOrStr {
    Num(u64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
#[mlua(tag = "type")]
enum TaggedNumOrStr {
    Num(u64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq, ToLua, FromLua)]
#[mlua(tag = "type", content = "val")]
enum TaggedContentNumOrStr {
    Num(u64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq, ToLua)]
#[mlua(tag = "type", content = "val")]
enum GenericEnum<T> {
    Yes(T),
    No(T),
}

#[test]
fn basic_unnamed() {
    use mlua::Lua;
    let lua = Lua::new();

    let table = Unnamed(1,"2".to_string());

    let round_trip_table = {
        let table = table.clone();
        let lt = table.to_lua(&lua).unwrap();
        Unnamed::from_lua(lt, &lua).unwrap()
    };

    assert_eq!(table, round_trip_table);
}

#[test]
fn basic_unit() {
    use mlua::Lua;
    let lua = Lua::new();

    let table = Unit;

    let round_trip_table = {
        let table = table.clone();
        let lt = table.to_lua(&lua).unwrap();
        Unit::from_lua(lt, &lua).unwrap()
    };

    assert_eq!(table, round_trip_table);
}

#[test]
fn basic_enum_to() {
    use mlua::Lua;
    let lua = Lua::new();

    let nos = NumOrStr::Num(37);

    lua.globals().set("nos", nos).unwrap();
    lua.load(
        r#"
            assert(type(nos) == "table")
            assert(nos.num == 37)
        "#,
    )
        .exec()
        .unwrap();
}

#[test]
fn basic_enum_from() {
    use mlua::Lua;
    let lua = Lua::new();
    let nos = lua.load(
        r#"
            return {num = 63}
        "#,
    )
        .eval::<NumOrStr>()
        .unwrap();


    assert_eq!(nos, NumOrStr::Num(63));
}

#[test]
fn tagged_content_enum_to() {
    use mlua::Lua;
    let lua = Lua::new();

    let tnos = TaggedContentNumOrStr::Num(37);
    lua.globals().set("tnos", tnos).unwrap();
    lua.load(
        r#"
            assert(type(tnos) == "table", "not table")
            assert(tnos.type ~= nil, "type not specified")
            assert(tnos.type == "num", "type not correct")
            assert(type(tnos) == "table", "value table not found")
            assert(tnos.val ~= nil, "value not found")
            assert(tnos.val == 37, "value in not found")
        "#,
    )
        .exec()
        .unwrap();
}
#[test]
fn tagged_content_enum_from() {
    use mlua::Lua;
    let lua = Lua::new();

    let nos = lua.load(
        r#"
            return {type = "num", val = 63}
        "#,
    )
        .eval::<TaggedContentNumOrStr>()
        .unwrap();

    assert_eq!(nos, TaggedContentNumOrStr::Num(63));
}

#[test]
fn understand_stuff() {
    use mlua::Lua;
    let lua = Lua::new();
    lua.load(
        r#"
            return {}
        "#,
    )
        .eval::<()>()
        .unwrap();

    assert_eq!((), ());
}
