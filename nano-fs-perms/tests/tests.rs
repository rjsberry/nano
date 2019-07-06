use nano_fs_perms::Perms;

use std::convert::TryFrom;

struct TestCase<'a> {
    perms: Perms,
    literal: u32,
    display: &'a str,
}

#[test]
fn perms() {
    let mut cases = Vec::new();

    cases.push(TestCase {
        perms: Perms::NONE,
        literal: 0,
        display: "---------",
    });

    cases.push(TestCase {
        perms: Perms::OWNER_READ | Perms::GROUP_READ | Perms::OTHERS_READ,
        literal: 0o444,
        display: "r--r--r--",
    });

    cases.push(TestCase {
        perms: Perms::OWNER_WRITE | Perms::GROUP_WRITE | Perms::OTHERS_WRITE,
        literal: 0o222,
        display: "-w--w--w-",
    });

    cases.push(TestCase {
        perms: Perms::OWNER_EXEC | Perms::GROUP_EXEC | Perms::OTHERS_EXEC,
        literal: 0o111,
        display: "--x--x--x",
    });

    cases.push(TestCase {
        perms: Perms::OWNER_READ | Perms::OWNER_WRITE | Perms::OWNER_EXEC,
        literal: 0o700,
        display: "rwx------",
    });

    cases.push(TestCase {
        perms: Perms::GROUP_READ | Perms::GROUP_WRITE | Perms::GROUP_EXEC,
        literal: 0o70,
        display: "---rwx---",
    });

    cases.push(TestCase {
        perms: Perms::OTHERS_READ | Perms::OTHERS_WRITE | Perms::OTHERS_EXEC,
        literal: 0o7,
        display: "------rwx",
    });

    cases.push(TestCase {
        perms: Perms::ALL,
        literal: 0o777,
        display: "rwxrwxrwx",
    });

    cases.push(TestCase {
        perms: Perms::ALL | Perms::SET_UID | Perms::SET_GID | Perms::STICKY_BIT,
        literal: 0o7777,
        display: "rwsrwsrwt",
    });

    cases.push(TestCase {
        perms: Perms::SET_UID | Perms::SET_GID | Perms::STICKY_BIT,
        literal: 0o7000,
        display: "--S--S--T",
    });

    for TestCase {
        perms,
        literal,
        display,
    } in cases.into_iter()
    {
        assert_eq!(
            perms,
            Perms::try_from(literal).expect(display),
            "{}",
            display
        );
        assert_eq!(&format!("{}", perms), display, "{}", display);
    }
}
