
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, TOTP};

const SECRET: &str  = "471dbb99f6b7";
const ISSUER: &str  = "MyCoolOrganization";
const LABEL: &str   = "my@organization.org";

fn main() -> Result<(), ()> {

    let totp = TOTP::new(
        Algorithm::SHA512,
        6,
        1,
        30,
        SECRET,
    );

    let qr = totp.get_qr(LABEL, ISSUER).unwrap();
    let code = totp.get_url(LABEL, ISSUER);

    println!("|QrCode| {}", qr);
    println!("|Code  | {}", code);
    println!("Input code: ");
    let mut code = String::new();
    std::io::stdin().read_line(&mut code).unwrap();
    
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let token = totp.generate(time);
    let checked = totp.check(&code, time);
    println!("Time {} ", &time);
    println!("Verifying {} ", &code);
    println!("Checked? {}", checked);

    let token2 = totp.generate(time);
    println!("{}\n{}", token, token2);


    Ok(())

}
