wit_bindgen::generate!({
    path: "../../tests/runtime/flavorful",
});

use exports::test::flavorful::test::*;

struct Component;

export!(Component);

impl Guest for Component {
    fn test_imports() {
        use test::flavorful::test::*;

        let _guard = test_rust_wasm::guard();

        f_list_in_record1(&ListInRecord1 {
            a: "list_in_record1".to_string(),
        });
        assert_eq!(f_list_in_record2().a, "list_in_record2");

        assert_eq!(
            f_list_in_record3(&ListInRecord3 {
                a: "list_in_record3 input".to_string()
            })
            .a,
            "list_in_record3 output"
        );

        assert_eq!(
            f_list_in_record4(&ListInAlias {
                a: "input4".to_string()
            })
            .a,
            "result4"
        );

        f_list_in_variant1(&Some("foo".to_string()), &Err("bar".to_string()));
        assert_eq!(f_list_in_variant2(), Some("list_in_variant2".to_string()));
        assert_eq!(
            f_list_in_variant3(&Some("input3".to_string())),
            Some("output3".to_string())
        );

        assert!(errno_result().is_err());
        MyErrno::A.to_string();
        _ = format!("{:?}", MyErrno::A);
        fn assert_error<T: std::error::Error>() {}
        assert_error::<MyErrno>();

        assert!(errno_result().is_ok());

        let (a, b) = list_typedefs(&"typedef1".to_string(), &vec!["typedef2".to_string()]);
        assert_eq!(a, b"typedef3");
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], "typedef4");

        let (a, b, c) = list_of_variants(
            &[true, false],
            &[Ok(()), Err(())],
            &[MyErrno::Success, MyErrno::A],
        );
        assert_eq!(a, [false, true]);
        assert_eq!(b, [Err(()), Ok(())]);
        assert_eq!(c, [MyErrno::A, MyErrno::B]);
    }
}

impl exports::test::flavorful::test::Guest for Component {
    fn f_list_in_record1(ty: ListInRecord1) {
        assert_eq!(ty.a, "list_in_record1");
    }

    fn f_list_in_record2() -> ListInRecord2 {
        ListInRecord2 {
            a: "list_in_record2".to_string(),
        }
    }

    fn f_list_in_record3(a: ListInRecord3) -> ListInRecord3 {
        assert_eq!(a.a, "list_in_record3 input");
        ListInRecord3 {
            a: "list_in_record3 output".to_string(),
        }
    }

    fn f_list_in_record4(a: ListInAlias) -> ListInAlias {
        assert_eq!(a.a, "input4");
        ListInRecord4 {
            a: "result4".to_string(),
        }
    }

    fn f_list_in_variant1(a: ListInVariant1V1, b: ListInVariant1V2) {
        assert_eq!(a.unwrap(), "foo");
        assert_eq!(b.unwrap_err(), "bar");
    }

    fn f_list_in_variant2() -> Option<String> {
        Some("list_in_variant2".to_string())
    }

    fn f_list_in_variant3(a: ListInVariant3) -> Option<String> {
        assert_eq!(a.unwrap(), "input3");
        Some("output3".to_string())
    }

    fn errno_result() -> Result<(), MyErrno> {
        MyErrno::A.to_string();
        _ = format!("{:?}", MyErrno::A);
        fn assert_error<T: std::error::Error>() {}
        assert_error::<MyErrno>();
        Err(MyErrno::B)
    }

    fn list_typedefs(a: ListTypedef, b: ListTypedef3) -> (ListTypedef2, ListTypedef3) {
        assert_eq!(a, "typedef1");
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], "typedef2");
        (b"typedef3".to_vec(), vec!["typedef4".to_string()])
    }

    fn list_of_variants(
        a: Vec<bool>,
        b: Vec<Result<(), ()>>,
        c: Vec<MyErrno>,
    ) -> (Vec<bool>, Vec<Result<(), ()>>, Vec<MyErrno>) {
        (a, b, c)
    }
}
