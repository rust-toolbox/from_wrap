#[macro_use]
extern crate from_wrap;

#[test]
fn single_struct() {
    #[derive(FromWrap)]
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
    #[derive(FromWrap)]
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
    #[derive(FromWrap)]
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
    #[derive(FromWrap)]
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
    #[derive(FromWrap)]
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
    #[derive(FromWrap)]
    enum EnumWithAttrs {
        Str { s: String },
        Int(i32),
        SomeUnit,
        #[not_generate_from_wrap]
        Pair(i32, i32),
        #[not_generate_from_wrap]
        Flt(f64),
        #[not_generate_from_wrap]
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
    #[derive(FromWrap)]
    enum EnumWithAttrs {
        #[generate_from_wrap]
        Str { s: String },
        Text { t: String },
        #[generate_from_wrap]
        Int(i32),
        Num(i32),
        SomeUnit,
        Pair(i32, i32),
        Flt(f64),
        #[not_generate_from_wrap] // this is not required, because generate_from_wrap already exist
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