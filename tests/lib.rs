#[macro_use]
extern crate simple_from;

#[test]
fn single_struct() {
    #[derive(SimpleFrom)]
    struct SingleStruct {
        s: String
    }

    assert!(match SingleStruct::from("qwerty".to_owned()) {
        SingleStruct{ s } => s == "qwerty"
    });
    assert!(!(match SingleStruct::from("qwerty".to_owned()) {
        SingleStruct{ s } => s == "data"
    }));
}

#[test]
fn single_tuple() {
    #[derive(SimpleFrom)]
    struct SingleTuple(String);

    assert!(match SingleTuple::from("qwerty".to_owned()) {
        SingleTuple(s) => s == "qwerty"
    });
    assert!(!(match SingleTuple::from("qwerty".to_owned()) {
        SingleTuple(s) => s == "data"
    }));
}

#[test]
fn enum_with_tuples() {
    #[derive(SimpleFrom)]
    enum EnumWithTuples {
        Str(String),
        Int(i32)
    }

    assert!(match EnumWithTuples::from(12) {
        EnumWithTuples::Int(x) => x == 12,
        _ => false
    });
    assert!(match EnumWithTuples::from("qwerty".to_owned()) {
        EnumWithTuples::Str(s) => s == "qwerty",
        _ => false
    });
}

#[test]
fn enum_with_structs() {
    #[derive(SimpleFrom)]
    enum EnumWithStructs {
        Str { s: String },
        Int { x: i32 }
    }

    assert!(match EnumWithStructs::from(12) {
        EnumWithStructs::Int { x } => x == 12,
        _ => false
    });
    assert!(match EnumWithStructs::from("qwerty".to_owned()) {
        EnumWithStructs::Str { s } => s == "qwerty",
        _ => false
    });
}

#[test]
#[allow(dead_code)]
fn enum_with_mixed() {
    #[derive(SimpleFrom)]
    enum EnumWithMixed {
        Str { s: String },
        Int(i32),
        SomeUnit
    }

    assert!(match EnumWithMixed::from(12) {
        EnumWithMixed::Int(x) => x == 12,
        _ => false
    });
    assert!(match EnumWithMixed::from("qwerty".to_owned()) {
        EnumWithMixed::Str { s } => s == "qwerty",
        _ => false
    });
}

#[test]
#[allow(dead_code)]
fn enum_with_not_generate() {
    #[derive(SimpleFrom)]
    enum EnumWithAttrs {
        Str { s: String },
        Int(i32),
        SomeUnit,
        #[not_generate_from]
        Pair(i32, i32),
        #[not_generate_from]
        Flt(f64),
        #[not_generate_from]
        Struct { a: i32, b: i32 }
    }

    assert!(match EnumWithAttrs::from(12) {
        EnumWithAttrs::Int(x) => x == 12,
        _ => false
    });
    assert!(match EnumWithAttrs::from("qwerty".to_owned()) {
        EnumWithAttrs::Str { s } => s == "qwerty",
        _ => false
    });
}

#[test]
#[allow(dead_code)]
fn enum_with_generate() {
    #[derive(SimpleFrom)]
    enum EnumWithAttrs {
        #[generate_from]
        Str { s: String },
        Text { t: String },
        #[generate_from]
        Int(i32),
        Num(i32),
        SomeUnit,
        Pair(i32, i32),
        Flt(f64),
        #[not_generate_from] // this is not required, because generate_from already exist
        Struct { a: i32, b: i32 }
    }

    assert!(match EnumWithAttrs::from(12) {
        EnumWithAttrs::Int(x) => x == 12,
        _ => false
    });
    assert!(match EnumWithAttrs::from("qwerty".to_owned()) {
        EnumWithAttrs::Str { s } => s == "qwerty",
        _ => false
    });
}