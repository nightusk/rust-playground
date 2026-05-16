use std::io;
use std::io::prelude::*;

fn main() {
    real_print(io::stdin().lock(), io::stdout().lock()).unwrap();
}

fn real_print<R, W>(r: R, mut w: W) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    for line in r.lines() {
        writeln!(w, "{}", line?)?;
    }
    Ok(())
}
