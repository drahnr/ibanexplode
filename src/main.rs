use eyre::{Result,eyre};
use iban::*;

fn main() -> Result<()> {
	let iban = std::env::args().skip(1).take(1).next().ok_or_else(|| { eyre!("Must provide an IBAN as the first argument")})?;
	let account = iban.parse::<Iban>()?;
	println!("Country: {}", account.country_code());
	println!("Check digits: {}", account.check_digits());
	println!("BBAN: {}", account.bban());
	println!("Electronic: {}", account.electronic_str());
	println!("IBAN: {}", account.to_string());
	let bank_identifier = account.bank_identifier().expect("A IBAN _includes_ the bank identifier. qed");
	println!("Bank Identifier: {}", bank_identifier);
	println!("Brank Branch: {}", account.branch_identifier().unwrap_or("unknown Bank Branch"));

	let bank = fints_institute_db::get_bank_by_bank_code(bank_identifier).ok_or_else(|| eyre!("No such bank known"))?;
	println!("BIC: {}", bank.bic);
	Ok(())
}
