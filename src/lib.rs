use std::fmt::Write;
use std::str::FromStr;

const INVALID_INPUT: &str = "Invalid input.";

#[derive(Debug, Clone)]
pub struct CliOption {
    pub text: String,
    pub func: fn(),
}

impl CliOption {
    pub fn new(cli_option: (&str, fn())) -> CliOption {
        CliOption {
            text: cli_option.0.to_string(),
            func: cli_option.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub name: String,
    pub options: Vec<CliOption>,
    pub menu_type: MenuType,
}

#[derive(Debug, Clone)]
pub enum MenuType {
    Main,
    Sub,
}

impl MenuType {
    fn bye_msg(&self) -> String {
        match *self {
            MenuType::Main => "Quit".to_string(),
            MenuType::Sub => "Back".to_string(),
        }
    }
}

impl Menu {
    pub fn run(&self) {
        println!("{}", self.name);
        loop {
            let mut prompt = String::new();
            for i in 0..self.options.len() {
                write!(
                    &mut prompt,
                    "\t{}) {}\n",
                    i + 1,
                    self.options.get(i).unwrap().text
                )
                .unwrap();
            }
            write!(
                &mut prompt,
                "\t{}) {}",
                self.options.len() + 1,
                self.menu_type.bye_msg()
            )
            .unwrap();
            let choice: usize =
                parse_std_in(std::io::stdin().lock(), std::io::stdout(), prompt.as_str());
            if choice > 0 && choice <= self.options.len() {
                (self.options.get(choice - 1).unwrap().func)();
            } else if choice == self.options.len() + 1 {
                println!("{}", self.menu_type.bye_msg());
                break;
            } else {
                println!("{}", INVALID_INPUT);
            }
        }
    }

    pub fn with_options(&self, options: Vec<(&str, fn())>) -> Self {
        let mut updated = self.clone();
        updated.options = options.iter().map(|&x| CliOption::new(x)).collect();
        return updated;
    }
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            name: "Main Menu".to_string(),
            options: Vec::new(),
            menu_type: MenuType::Sub,
        }
    }
}

// Inspiration: https://stackoverflow.com/questions/28370126/how-can-i-test-stdin-and-stdout
pub fn parse_std_in<R, W, T>(mut reader: R, mut writer: W, prompt: &str) -> T
where
    R: std::io::BufRead,
    W: std::io::Write,
    T: FromStr,
{
    let mut line: String;
    loop {
        write!(&mut writer, "{}\n", prompt).unwrap();
        line = String::new();
        reader.read_line(&mut line).unwrap();
        match line.trim().parse::<T>() {
            Ok(x) => return x,
            Err(..) => write!(&mut writer, "{}\n", INVALID_INPUT).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_i32_like_a_champ() {
        let input = b"3";
        let mut output = Vec::new();
        let actual: i32 = parse_std_in(&input[..], &mut output, "i32 plz");
        assert_eq!(actual, 3);
        let expected_output = "i32 plz\n";
        assert_eq!(expected_output, String::from_utf8(output).unwrap());
    }

    #[test]
    fn parses_i32_on_2nd_try() {
        let input = b"sup\n8\n9";
        let mut output = Vec::new();
        let actual: i32 = parse_std_in(&input[..], &mut output, "i32 plz");
        assert_eq!(actual, 8);
        let expected_output = "i32 plz\nInvalid input.\ni32 plz\n";
        assert_eq!(expected_output, String::from_utf8(output).unwrap());
    }
}
