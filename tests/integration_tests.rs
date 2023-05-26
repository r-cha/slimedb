use slimedb::process;
use slimedb::table::Table;
use capture::capture_output;


#[test]
fn test_insert_and_select() {
    let mut table = Table::default();
    process("insert 1 a a".to_string(), &mut table);
    process("insert 2 b b".to_string(), &mut table);
    process("insert 3 c c".to_string(), &mut table);

    let output = capture_output(|| {
        process("select".to_string(), &mut table);
    });

    let expected_output = "\
Row { id: 1, username: \"a\", email: \"a\" }\n\
Row { id: 2, username: \"b\", email: \"b\" }\n\
Row { id: 3, username: \"c\", email: \"c\" }\n";

    assert_eq!(output, expected_output);
}
