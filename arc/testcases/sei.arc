struct StructName {
    field1 i64,
    field2 i64,
    field3 i32,
    field4 SomeStruct,
    field5 SomeEnum,
}

struct SomeStruct {
    one i64,
    two f64,
}

enum EnumName {
    Option1,
    Option2(i32),
    Option3,
}

impl SomeStruct {
    fx len(&self) ~ i32 {
        let length = add(&self.one, &self.two);
        return length;
    }
}

impl string {
    fx len(&self) ~ i32 {
        let some_struct = SomeStruct {
            one: 1,
            two: 2.0,
        };
        let some_enum = EnumName->Option2;
        let length = some_struct.one;
        return length;
    }
}
