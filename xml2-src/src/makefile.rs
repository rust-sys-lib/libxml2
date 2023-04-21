use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

#[derive(Debug)]
struct MultiLines<B> {
    lines: Lines<B>,
}

impl<B: BufRead> Iterator for MultiLines<B> {
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = vec![];
        loop {
            let line = self.lines.next();
            match line {
                Some(line_res) => match line_res {
                    Ok(line) => {
                        if let Some(line) = line.strip_suffix('\\') {
                            buf.push(line.to_string());
                        } else {
                            buf.push(line);
                            return Some(Ok(buf.join("")));
                        }
                    }
                    Err(err) => return Some(Err(err)),
                },
                None => {
                    return if !buf.is_empty() {
                        Some(Ok(buf.join("")))
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl MultiLines<BufReader<File>> {
    pub fn read_from_path<P: AsRef<Path>>(value: P) -> Result<Self, std::io::Error> {
        let file = File::open(value)?;
        let reader = BufReader::new(file);
        Ok(Self {
            lines: reader.lines(),
        })
    }
}

#[derive(Debug)]
pub struct Rule {
    commands: Vec<String>,
    dependencies: Vec<String>,
}

pub fn parse_makefile<P: AsRef<Path>>(filename: P) -> HashMap<String, Rule> {
    let file = File::open(filename).expect("readable file");
    let reader = BufReader::new(file);
    let lines = MultiLines {
        lines: reader.lines(),
    };
    let mut rules: HashMap<String, Rule> = HashMap::new();
    let mut current_rule: Option<String> = None;

    for line in lines {
        let line = line.expect("line from file").trim().to_owned();

        if line.starts_with('#') || line.is_empty() {
            // Skip comments and empty lines
            continue;
        }

        if line.starts_with('\t') {
            // We are parsing commands for the current rule
            if let Some(rule_name) = current_rule.clone() {
                if let Some(rule) = rules.get_mut(&rule_name) {
                    rule.commands.push(line);
                }
            }
        } else if let Some((rule_name, dependencies)) = parse_rule(&line) {
            // We are parsing a new rule
            let rule = Rule {
                dependencies,
                commands: Vec::new(),
            };
            rules.insert(rule_name.to_owned(), rule);
            current_rule = Some(rule_name.to_owned());
        } else {
            panic!("Invalid Makefile line: {}", line);
        }
    }

    rules
}

fn parse_rule(line: &str) -> Option<(String, Vec<String>)> {
    let mut parts = line.split(':');
    let rule_name = parts.next()?.trim();
    let dependencies = parts
        .next()
        .map(|deps| deps.split_whitespace().map(|dep| dep.to_owned()).collect())
        .unwrap_or_default();

    Some((rule_name.to_owned(), dependencies))
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_makefile() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/./vendor/libxml2/Makefile.am");
        let rules = super::parse_makefile(path);
        for (var, rule) in rules {
            dbg!(var, rule);
        }
    }

    #[test]
    fn check_multilines() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/./vendor/libxml2/Makefile.am");
        let lines = super::MultiLines::read_from_path(path).expect("multiline file");
        for line in lines {
            dbg!(line.unwrap());
        }
        dbg!("done");
    }
}
