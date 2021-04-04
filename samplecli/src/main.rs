use anyhow::Result;

mod rpn_calculator;

use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;


#[derive(Clap, Debug)]
#[clap(
    name = "My Rpn program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]

struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = rpn_calculator::RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ok() {
        let calc = rpn_calculator::RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    #[test]
    fn test_ng() {
        let calc = rpn_calculator::RpnCalculator::new(false);
        assert!(calc.eval("").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}