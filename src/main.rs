use crate::{cli::QrUrl, daemon::Daemon};
use clap::Parser;

mod cli;
mod daemon;
mod generate;

fn main() {
    let qr_args = QrUrl::parse();

    println!(
        "url: {:?}, format: PNG",
        qr_args.url,
        // qr_args.format.to_string()
    );
    if Daemon::should_run_in_daemon() {
        Daemon::run_in_daemon();
        return;
    }
    generate::Generate::png(qr_args.url)
}
