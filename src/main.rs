#[cfg(target_os = "linux")]
use tdx::Tdx;

fn main() {
    tee_attestation_example()
}

#[cfg(target_os = "linux")]
pub fn tee_attestation_example() {
    // Initialise a TDX object
    let tdx = Tdx::new();

    // Retrieve an attestation report with default options passed to the hardware device
    let raw_report = tdx.get_attestation_report_raw().unwrap();
    let attestation_report = QuoteV4::from_bytes(&raw_report);
    println!(
        "Attestation Report raw bytes: 0x{}",
        hex::encode(raw_report)
    );
    println!("Operating System: {}", std::env::consts::OS);
}

#[cfg(not(target_os = "linux"))]
pub fn tee_attestation_example() {
    println!("Operating System: {}", std::env::consts::OS);
}
