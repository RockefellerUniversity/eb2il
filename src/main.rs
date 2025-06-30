use std::io::{self, BufRead, Write}; // No flate2 or crossbeam-channel needed

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock(); // Lock stdin for efficient line-by-line reading
    let mut writer = stdout.lock(); // Lock stdout for efficient line-by-line writing

    let mut line_num = 0; // 1-based line number for processing logic

    for line_result in reader.lines() {
        let line = line_result?;
        line_num += 1;

        let processed_line = if line_num % 4 == 0 {
            // Process 4n+4 lines
            let mut chars: Vec<char> = Vec::with_capacity(line.len());
            for c in line.chars() {
                if (c as u32) > 75 { // Check if ASCII code is greater than 75
                    chars.push('K'); // Replace with 'K' (ASCII 75)
                } else {
                    chars.push(c);
                }
            }
            chars.into_iter().collect::<String>()
        } else {
            // Keep 4n+1, 4n+2, and 4n+3 lines as is
            line
        };

        writeln!(writer, "{}", processed_line)?;
    }

    Ok(())
}