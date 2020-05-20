use hex_literal::hex;
use schnorr_fun::{Schnorr, Signature};
use secp256kfun::{hash::Derivation, marker::*, Scalar, XOnly};

lazy_static::lazy_static! {
    pub static ref BIP340: Schnorr = Schnorr::from_tag(b"BIP340");
}

fn signing_test_vector(
    secret_key: [u8; 32],
    public_key: [u8; 32],
    aux_rand: [u8; 32],
    message: [u8; 32],
    target_sig: [u8; 64],
) {
    let secret_key = Scalar::from_bytes_mod_order(secret_key)
        .mark::<NonZero>()
        .unwrap();
    let keypair = BIP340.new_keypair(secret_key);

    assert_eq!(keypair.public_key().as_bytes(), &public_key);
    let signature = BIP340.sign(&keypair, &message, Derivation::Aux(aux_rand));
    assert_eq!(signature.to_bytes().as_ref(), target_sig.as_ref());
    assert!(BIP340.verify(&keypair.verification_key(), &message[..], &signature));
}

enum Outcome {
    /// The signature was valid
    Success,
    /// The signature was invalid
    Failure,
    /// The public key was not on the curve
    BadPublicKey,
    /// The signature nonce was not on the curve
    BadSignatureFormat,
}

fn verification_test_vector(
    public_key: [u8; 32],
    message: [u8; 32],
    sig: [u8; 64],
    expected_outcome: Outcome,
) {
    use Outcome::*;
    let public_key = {
        let public_key = XOnly::from_bytes(public_key);
        match expected_outcome {
            BadPublicKey => return assert!(public_key.is_none()),
            _ => public_key.unwrap(),
        }
    };

    let signature = {
        let signature = Signature::from_bytes(sig);
        match expected_outcome {
            BadSignatureFormat => return assert!(signature.is_none()),
            _ => signature.unwrap(),
        }
    };

    assert_eq!(
        BIP340.verify(&public_key.into(), &message, &signature),
        match expected_outcome {
            Success => true,
            Failure => false,
            _ => unreachable!(),
        },
    )
}

secp256kfun::test_plus_wasm! {

fn bip340_signing_vectors() {
    signing_test_vector(
        hex!("0000000000000000000000000000000000000000000000000000000000000003"),
        hex!("F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9"),
        hex!("0000000000000000000000000000000000000000000000000000000000000000"),
        hex!("0000000000000000000000000000000000000000000000000000000000000000"),
        hex!("067E337AD551B2276EC705E43F0920926A9CE08AC68159F9D258C9BBA412781C9F059FCDF4824F13B3D7C1305316F956704BB3FEA2C26142E18ACD90A90C947E")
    );

    signing_test_vector(
        hex!("B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF"),
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("0000000000000000000000000000000000000000000000000000000000000001"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("0E12B8C520948A776753A96F21ABD7FDC2D7D0C0DDC90851BE17B04E75EF86A47EF0DA46C4DC4D0D1BCB8668C2CE16C54C7C23A6716EDE303AF86774917CF928")
    );

    signing_test_vector(
        hex!("C90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B14E5C9"),
        hex!("DD308AFEC5777E13121FA72B9CC1B7CC0139715309B086C960E18FD969774EB8"),
        hex!("C87AA53824B4D7AE2EB035A2B5BBBCCC080E76CDC6D1692C4B0B62D798E6D906"),
        hex!("7E2D58D8B3BCDF1ABADEC7829054F90DDA9805AAB56C77333024B9D0A508B75C"),
        hex!("FC012F9FB8FE00A358F51EF93DCE0DC0C895F6E9A87C6C4905BC820B0C3677616B8737D14E703AF8E16E22E5B8F26227D41E5128F82D86F747244CC289C74D1D")
    );

    signing_test_vector(
        hex!("0B432B2677937381AEF05BB02A66ECD012773062CF3FA2549E44F58ED2401710"),
        hex!("25D1DFF95105F5253C4022F628A996AD3A0D95FBF21D468A1B33F8C160D8F517"),
        hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"),
        hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"),
        hex!("FC132D4E426DFF535AEC0FA7083AC5118BC1D5FFFD848ABD8290C23F271CA0DD11AEDCEA3F55DA9BD677FE29C9DDA0CF878BCE43FDE0E313D69D1AF7A5AE8369")
    );
}

fn bip340_verification_vectors() {
    verification_test_vector(
        hex!("D69C3509BB99E412E68B0FE8544E72837DFA30746D8BE2AA65975F29D22DC7B9"),
        hex!("4DF3C3F68FCC83B27E9D42C90431A72499F17875C81A599B566C9889B9696703"),
        hex!("00000000000000000000003B78CE563F89A0ED9414F5AA28AD0D96D6795F9C630EC50E5363E227ACAC6F542CE1C0B186657E0E0D1A6FFE283A33438DE4738419"),
        Outcome::Success,
    );

    verification_test_vector(
        hex!("EEFDEA4CDB677750A420FEE807EACF21EB9898AE79B9768766E4FAA04A2D4A34"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("7036D6BFE1837AE919631039A2CF652A295DFAC9A8BBB0806014B2F48DD7C807941607B563ABBA414287F374A332BA3636DE009EE1EF551A17796B72B68B8A24"),
        Outcome::BadPublicKey,
    );

    verification_test_vector(
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F995A579DA959FA739FCE39E8BD16FECB5CDCF97060B2C73CDE60E87ABCA1AA5D9"),
        Outcome::Failure,
    );

    verification_test_vector(
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("F8704654F4687B7365ED32E796DE92761390A3BCC495179BFE073817B7ED32824E76B987F7C1F9A751EF5C343F7645D3CFFC7D570B9A7192EBF1898E1344E3BF"),
        Outcome::Failure
    );

    verification_test_vector(
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("7036D6BFE1837AE919631039A2CF652A295DFAC9A8BBB0806014B2F48DD7C8076BE9F84A9C5445BEBD780C8B5CCD45C883D0DC47CD594B21A858F31A19AAB71D"),
        Outcome::Failure,
    );

    verification_test_vector(
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("74556372D3369E8C53E6B84B5D7EE9AE0220EB37A6EA5501EF828FBFBA90A864F6D108D88692535AEEE74170428F4C126A5B6E80D3D965007FF159F46E34B63F"),
        Outcome::Failure,
    );

    verification_test_vector(
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("00000000000000000000000000000000000000000000000000000000000000009915EE59F07F9DBBAEDC31BFCC9B34AD49DE669CD24773BCED77DDA36D073EC8"),
        Outcome::BadSignatureFormat,
    );

    verification_test_vector(
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("0000000000000000000000000000000000000000000000000000000000000001C7EC918B2B9CF34071BB54BED7EB4BB6BAB148E9A7E36E6B228F95DFA08B43EC"),
        Outcome::Failure,
    );

    verification_test_vector(
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("4A298DACAE57395A15D0795DDBFD1DCB564DA82B0F269BC70A74F8220429BA1D941607B563ABBA414287F374A332BA3636DE009EE1EF551A17796B72B68B8A24"),
        Outcome::BadSignatureFormat,
    );

    verification_test_vector(
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F941607B563ABBA414287F374A332BA3636DE009EE1EF551A17796B72B68B8A24"),
        Outcome::BadSignatureFormat,
    );

    verification_test_vector(
        hex!("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("7036D6BFE1837AE919631039A2CF652A295DFAC9A8BBB0806014B2F48DD7C807FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141"),
        Outcome::BadSignatureFormat,
    );

    verification_test_vector(
        hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC30"),
        hex!("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89"),
        hex!("74556372D3369E8C53E6B84B5D7EE9AE0220EB37A6EA5501EF828FBFBA90A864092EF727796DACA51118BE8FBD70B3EC50536E65DB6F3B3B3FE1049862018B02"),
        Outcome::BadPublicKey,
    )
}
}
