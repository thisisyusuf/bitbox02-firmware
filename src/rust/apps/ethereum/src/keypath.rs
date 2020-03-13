use util::bip32::HARDENED;

static ACCOUNT_MAX: u32 = 99; // 100 accounts

/// Does limit checks the keypath, whitelisting bip44 purpose, account and change.
/// Only allows the well-known xpub of m'/44'/60'/0'/0 for now.
/// Since ethereum doesn't use the "change" path part it is always 0 and have become part of the
/// xpub keypath.
/// @return true if the keypath is valid, false if it is invalid.
pub fn is_valid_keypath_xpub(keypath: &[u32], expected_coin: u32) -> bool {
    keypath.len() == 4 && &keypath[..4] == &[44 + HARDENED, expected_coin, 0 + HARDENED, 0]
}

/// Does limit checks the keypath, whitelisting bip44 purpose, account and change.
/// Returns true if the keypath is valid, false if it is invalid.
pub fn is_valid_keypath_address(keypath: &[u32], expected_coin: u32) -> bool {
    if keypath.len() != 5 {
        return false;
    }
    if &keypath[..4] != &[44 + HARDENED, expected_coin, 0 + HARDENED, 0] {
        return false;
    }
    if keypath[4] > ACCOUNT_MAX {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_keypath_xpub() {
        let expected_coin = 60 + HARDENED;
        assert!(is_valid_keypath_xpub(
            &[44 + HARDENED, expected_coin, 0 + HARDENED, 0],
            expected_coin
        ));
        // wrong coin.
        assert!(!is_valid_keypath_xpub(
            &[44 + HARDENED, expected_coin, 0 + HARDENED, 0],
            expected_coin + 1,
        ));
        // too short
        assert!(!is_valid_keypath_xpub(
            &[44 + HARDENED, expected_coin, 0 + HARDENED],
            expected_coin + 1,
        ));
        // too long
        assert!(!is_valid_keypath_xpub(
            &[44 + HARDENED, expected_coin, 0 + HARDENED, 0, 0],
            expected_coin + 1,
        ));
    }

    #[test]
    fn test_is_valid_keypath_address() {
        let expected_coin = 60 + HARDENED;
        let keypath_for_account =
            |account| [44 + HARDENED, expected_coin, 0 + HARDENED, 0, account];

        // 100 good paths.
        for account in 0..100 {
            assert!(is_valid_keypath_address(
                &keypath_for_account(account),
                expected_coin
            ));
            // wrong coin
            assert!(!is_valid_keypath_address(
                &keypath_for_account(account),
                expected_coin + 1
            ));
        }
        assert!(!is_valid_keypath_address(
            &keypath_for_account(100),
            expected_coin
        ));

        // too short
        assert!(!is_valid_keypath_address(
            &[44 + HARDENED, expected_coin, 0 + HARDENED, 0],
            expected_coin
        ));
        // too long
        assert!(!is_valid_keypath_address(
            &[44 + HARDENED, expected_coin, 0 + HARDENED, 0, 0, 0],
            expected_coin
        ));
        // tweak keypath elements
        for i in 0..4 {
            let mut keypath = keypath_for_account(0);
            keypath[i] += 1;
            assert!(!is_valid_keypath_address(&keypath, expected_coin));
        }
    }
}
