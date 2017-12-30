extern crate jsonxf;

#[test]
fn pretty_print_passes_test_cases() {
  // \t will be used as our indent, to make these examples a bit clearer
  let test_cases: Vec<[&str; 2]> = vec![
    [ // one object, one k-v pair
      "{\"hello\":\"world\"}",
      "{\n\t\"hello\": \"world\"\n}\n",
    ],

    [ // one object, several k-v pairs
      " { \"hello\": \"world2\",\r\n  \"wow\": \"cool\"  } \r\n",
      "{\n\t\"hello\": \"world2\",\n\t\"wow\": \"cool\"\n}\n",
    ],

    [ // simple array
      "[1,2,3]",
      "[\n\t1,\n\t2,\n\t3\n]\n",
    ],

    [ // one object per line, with blank lines and missing newlines
      " { \"hello\": \"world3\"}\r\n\n\n  { \"wow\": \"cool\"  }{\"a\":\"b\"}",
      "{\n\t\"hello\": \"world3\"\n}\n{\n\t\"wow\": \"cool\"\n}\n{\n\t\"a\": \"b\"\n}\n",
    ],

    [ // one array per line, with blank lines and missing newlines
      "[1, 2, \"omg\"]\n\n\n[\"whee\", {}, 22][]",
      "[\n\t1,\n\t2,\n\t\"omg\"\n]\n[\n\t\"whee\",\n\t{},\n\t22\n]\n[]\n",
    ],

    [ // nested empty array
      " { \"hello\": [\n\n] }",
      "{\n\t\"hello\": []\n}\n",
    ],

    [ // nested empty object
      " { \"hello\": {} }",
      "{\n\t\"hello\": {}\n}\n",
    ],

    [ // nested structures
      " { \"hello\": [ \"world5\" , 22 ] } \r\n",
      "{\n\t\"hello\": [\n\t\t\"world5\",\n\t\t22\n\t]\n}\n",
    ],

    [ // empty object special case
      " { \n\r\n\t } ",
      "{}\n",
    ],
  ];

  for case in test_cases {
    let input = case[0];
    let output = case[1];
    assert_eq!(jsonxf::pretty_print(input, "\t").unwrap(), output);
  }
}

