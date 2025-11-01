use std::fs;
use std::io::{Read, Write};
use std::iter::Peekable;
use std::str::Lines;

pub struct Doc {
    sections: Vec<Section>,
}

impl Doc {
    pub fn write_to_file(&self, fp: &str) -> Result<(), String> {
        let mut f = fs::File::create(fp).map_err(|e| e.to_string())?;
        for (_, s) in self.sections.iter().enumerate() {
            let lines = match s {
                Section::Comment { lines } => lines,
                Section::DocComment { lines } => lines,
                Section::Code { lines } => lines,
            };

            for l in lines {
                f.write_all(l.as_bytes()).map_err(|e| e.to_string())?;
                f.write("\n".as_bytes()).map_err(|e| e.to_string())?;
            }

            if !lines.is_empty() {
                f.write("\n".as_bytes()).map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Section {
    Comment { lines: Vec<String> },
    DocComment { lines: Vec<String> },
    Code { lines: Vec<String> },
}

pub fn compile(fp: &str) -> Result<Doc, String> {
    let mut f = fs::File::open(fp).map_err(|e| e.to_string())?;
    let mut s: String = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut sections: Vec<Section> = Vec::new();
    let mut lines = s.lines().peekable();

    loop {
        let line = match lines.peek() {
            None => break,
            Some(l) => *l,
        };

        let section: Section;
        if line.starts_with("///") {
            section = parse_doc_comment(&mut lines)?;
        } else if line.starts_with("//t") {
            section = parse_comment(&mut lines)?;
        } else {
            section = parse_code(&mut lines)?;
        }

        sections.push(section);
    }

    // format code
    for s in sections.iter_mut() {
        match s {
            Section::Code { lines } => format_code(lines),
            Section::Comment { .. } => {}
            Section::DocComment { .. } => {}
        }
    }

    Ok(Doc { sections })
}

fn parse_doc_comment(lines: &mut Peekable<Lines>) -> Result<Section, String> {
    let mut ls = Vec::new();
    loop {
        let line = match lines.peek() {
            Some(l) if l.starts_with("///") => lines.next().unwrap(),
            _ => break,
        };

        let trimmed = line.trim();
        ls.push(trimmed.to_string());
    }

    if ls.is_empty() {
        Err("empty doc comment".to_string())
    } else {
        Ok(Section::DocComment { lines: ls })
    }
}

fn parse_comment(lines: &mut Peekable<Lines>) -> Result<Section, String> {
    let mut ls = Vec::new();
    loop {
        let line = match lines.peek() {
            Some(l) if l.starts_with("//t") => lines.next().unwrap(),
            _ => break,
        };

        let trimmed = line.strip_prefix("//t").unwrap_or(line);
        let trimmed = trimmed.strip_prefix(" ").unwrap_or(trimmed).trim_end();

        ls.push(trimmed.to_string());
    }

    if ls.is_empty() {
        Err("empty comment".to_string())
    } else {
        Ok(Section::Comment { lines: ls })
    }
}

fn parse_code<'a, I>(lines: &mut Peekable<I>) -> Result<Section, String>
where
    I: Iterator<Item = &'a str>,
{
    let mut ls = Vec::new();
    loop {
        let line = match lines.peek() {
            Some(l) if l.starts_with("//t") => break,
            Some(l) if l.starts_with("///") => break,
            Some(_) => lines.next().unwrap(),
            None => break,
        };

        ls.push(line.to_string());
    }

    if ls.is_empty() {
        Err("empty code".to_string())
    } else {
        Ok(Section::Code { lines: ls })
    }
}

fn format_code(lines: &mut Vec<String>) {
    let mut i = 0;
    let mut prev_empty = false;
    let mut started = false;

    // remove groups of empty lines
    while i < lines.len() {
        let is_empty = lines[i].trim().is_empty();
        if is_empty && !started {
            lines.remove(i);
            continue;
        }
        started = true;

        if is_empty && prev_empty {
            lines.remove(i);
        } else {
            prev_empty = is_empty;
            i += 1;
        }
    }

    if !lines.is_empty() {
        if lines.last().unwrap().is_empty() {
            lines.pop();
        };
    }

    if !lines.is_empty() {
        lines.insert(0, "```rust".to_string());
        lines.push("```".to_string());
    }
}
