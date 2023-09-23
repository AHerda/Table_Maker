use std::io::{self, Write};

#[derive(PartialEq, Debug)]
enum Dir {
    Row,
    Column,
}

struct Table {
    flow_direction: Dir,
    columns: u16,
    rows: u16,
    headers: Vec<String>,
    names: Vec<String>,
    values: Vec<Vec<String>>,
}

impl Table {
    fn new() -> Table {
        Table {
            flow_direction: Dir::Row,
            columns: 0,
            rows: 0,
            headers: Vec::new(),
            names: Vec::new(),
            values: Vec::new(),
        }
    }

    fn change_flow_dir(&mut self) -> Dir {
        self.flow_direction = match self.flow_direction {
            Dir::Row => Dir::Column,
            Dir::Column => Dir::Row,
        };

        match self.flow_direction {
            Dir::Row => Dir::Row,
            Dir::Column => Dir::Column,
        }
    }

    fn set_flow_dir(&mut self, dir: Dir) {
        self.flow_direction = dir;
    }

    fn set_columns(&mut self, columns: u16) -> Result<u16, String> {
        if self.columns == 0 {
            self.columns = columns;
            return Ok(columns);
        }
        Err(String::from("Columns already set"))
    }

    fn set_headers(&mut self, headers: Vec<String>) -> Result<u16, String> {
        let len: u16 = u16::try_from(headers.len()).unwrap();
        if self.columns == 0 {
            self.columns = len;
        } else if self.columns != len || self.headers != Vec::<String>::new() {
            return Err(String::from(
                "Columns or headers already set or wrong number of headers",
            ));
        }

        self.headers = headers;
        Ok(len)
    }

    fn add_object(&mut self, name: String, values: Vec<String>) -> Result<String, String> {
        if self.columns != u16::try_from(values.len()).unwrap() {
            return Err(format!(
                "Wrong number of values\nShould be: {}",
                self.columns
            ));
        }

        self.names.push(name.clone());
        self.values.push(values);
        self.rows += 1;

        Ok(name)
    }

    fn get_as_string_row(&self, horizontal_lines: bool) -> String {
        let mut result = String::new();
        let mut break_line = String::from("+");
        let mut header_line: String = String::from("|");
        let mut column_lengths = vec![0; (self.columns + 1).into()];

        column_lengths[0] = self.names
            .iter()
            .max_by(|s1, s2| s1.len().cmp(&s2.len()))
            .unwrap()
            .len();

        self.headers.iter().enumerate().for_each(|(i, s)| {
            column_lengths[i + 1] = s.len();
        });

        for i in 0..self.columns {
            for j in 0..self.rows {
                let len = self.values[j as usize][i as usize].len();
                if column_lengths[(i + 1) as usize] < len {
                    column_lengths[(i + 1) as usize] = len;
                }
            }
        }
        for len in &mut column_lengths {
            *len += 2;
        }

        column_lengths.iter().for_each(|len| {
            break_line.push_str(format!("{data:-^length$}+", data = "-", length = len).as_str());
        });
        break_line.push('\n');

        header_line
            .push_str(format!("{data:^len$}|", len = column_lengths[0], data = " ").as_str());
        column_lengths[1..]
            .iter()
            .enumerate()
            .for_each(|(index, len)| {
                header_line.push_str(format!("{:^len$}|", self.headers[index]).as_str());
            });
        header_line.push('\n');

        // Header part
        result.push_str(&break_line);
        result.push_str(&header_line);
        result.push_str(&break_line);

        for row in 0..self.rows {
            let mut line: String = String::from("|");
            line.push_str(&format!(
                "{data:^len$}|",
                len = column_lengths[0],
                data = self.names[row as usize]
            ));
            column_lengths[1..]
                .iter()
                .enumerate()
                .for_each(|(column, len)| {
                    line.push_str(&format!("{:^len$}|", self.values[row as usize][column]));
                });
            line.push('\n');

            result.push_str(&line);
            if horizontal_lines || row == self.rows - 1 { result.push_str(&break_line); };
        }

        result
    }

    fn get_as_string_column(&self, horizontal_lines: bool) -> String {
        let mut result = String::new();
        let mut break_line = String::from("+");
        let mut name_line: String = String::from("|");
        let mut column_lengths = vec![0; (self.rows + 1).into()];

        column_lengths[0] = self.headers
            .iter()
            .map(|header| header.len())
            .max()
            .unwrap_or(0) + 2;

        self.values
            .iter()
            .enumerate()
            .for_each(|(index, values)| {
                column_lengths[index + 1] = values
                    .iter()
                    .map(|value| value.len())
                    .max()
                    .unwrap_or(0);
            });
        
        self.names
            .iter()
            .map(|name| name.len())
            .enumerate()
            .for_each(|(index, len)| {
                if len > column_lengths[index + 1] {
                    column_lengths[index + 1] = len;
                }
                column_lengths[index + 1] += 2;
            });
        
        // Creating break line
        column_lengths
            .iter()
            .for_each(|len| {
                break_line.push_str(format!("{data:-^length$}+", data = "-", length = len).as_str());
            });
        break_line.push('\n');

        // Creating name line
        name_line
            .push_str(format!("{data:^len$}|", len = column_lengths[0], data = " ").as_str());
        column_lengths[1..]
            .iter()
            .enumerate()
            .for_each(|(index, len)| {
                name_line.push_str(format!("{:^len$}|", self.names[index]).as_str());
            });
        name_line.push('\n');

        // Name part
        result.push_str(&break_line);
        result.push_str(&name_line);
        result.push_str(&break_line);

        for column_u16 in 0..self.columns {
            let mut line: String = String::from("|");
            let column_usize: usize = usize::from(column_u16);

            line.push_str(&format!("{data:^len$}|", len = column_lengths[0], data = self.headers[column_usize]));

            column_lengths[1..]
                .iter()
                .enumerate()
                .for_each(|(row, len)| {
                    line.push_str(&format!("{data:^len$}|", data = self.values[row][column_usize]))
                });
            line.push('\n');
            result.push_str(&line);
            if horizontal_lines || column_u16 == self.columns - 1 { result.push_str(&break_line); };
        }

        result
    }

    fn get_as_string(&self, horizontal_lines: bool) -> String {
        if self.flow_direction == Dir::Row {
            self.get_as_string_row(horizontal_lines)
        } else if self.flow_direction == Dir::Column {
            self.get_as_string_column(horizontal_lines)
        } else {
            String::from("The fuck you did?!?!")
        }
    }

    fn print(&self) {
        _ = write!(io::stdout(), "{}", self.get_as_string(true));
    }

    fn println(&self) {
        _ = writeln!(io::stdout(), "{}", self.get_as_string(true));
    }

    fn print_no_hl(&self) {
        _ = write!(io::stdout(), "{}", self.get_as_string(false));
    }

    fn println_no_hl(&self) {
        _ = writeln!(io::stdout(), "{}", self.get_as_string(false));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let result: Table = Table::new();
        assert_eq!(result.flow_direction, Dir::Row);
        assert_eq!(result.columns, 0);
        assert_eq!(result.rows, 0);
        assert_eq!(result.headers, Vec::<String>::new());
        assert_eq!(result.names, Vec::<String>::new());
    }

    #[test]
    fn change_flow_dir_test() {
        let mut tab: Table = Table::new();
        assert_eq!(tab.flow_direction, Dir::Row);
        let flow: Dir = tab.change_flow_dir();
        assert_eq!(flow, Dir::Column);
        assert_eq!(tab.flow_direction, Dir::Column);
    }

    #[test]
    fn set_flow_dir_test() {
        let mut tab: Table = Table::new();
        assert_eq!(tab.flow_direction, Dir::Row);

        tab.set_flow_dir(Dir::Row);
        assert_eq!(tab.flow_direction, Dir::Row);

        tab.set_flow_dir(Dir::Column);
        assert_eq!(tab.flow_direction, Dir::Column);

        tab.set_flow_dir(Dir::Row);
        assert_eq!(tab.flow_direction, Dir::Row);
    }

    #[test]
    fn set_columns_test() {
        let mut tab: Table = Table::new();
        let len = tab.set_columns(5);

        assert_eq!(len, Ok(5));
        assert_eq!(tab.columns, 5);

        let result = tab.set_columns(10);
        match result {
            Ok(_) => panic!("Wrong?!?!"),
            Err(err) => assert_eq!("Columns already set", err),
        }

        assert_eq!(tab.columns, 5);
    }

    #[test]
    fn set_headers_test() {
        let mut tab: Table = Table::new();
        _ = tab.set_columns(3);

        let result = tab.set_headers(vec![
            String::from("h1"),
            String::from("h2"),
            String::from("h3"),
        ]);
        match result {
            Ok(len) => {
                assert_eq!(len, 3);
                assert_eq!(
                    tab.headers,
                    vec![String::from("h1"), String::from("h2"), String::from("h3")]
                )
            }
            Err(_) => panic!("WTF?!?!"),
        }

        let result = tab.set_headers(vec![
            String::from("h3"),
            String::from("h3"),
            String::from("h3"),
        ]);
        match result {
            Ok(_) => panic!("WTF!!??"),
            Err(err) => {
                assert_eq!(
                    err,
                    "Columns or headers already set or wrong number of headers"
                );
                assert_eq!(
                    tab.headers,
                    vec![String::from("h1"), String::from("h2"), String::from("h3")]
                );
            }
        }
    }

    #[test]
    fn set_headers_test_2() {
        let mut tab: Table = Table::new();

        let result = tab.set_headers(vec![
            String::from("h1"),
            String::from("h2"),
            String::from("h3"),
        ]);
        match result {
            Ok(len) => {
                assert_eq!(len, 3);
                assert_eq!(
                    tab.headers,
                    vec![String::from("h1"), String::from("h2"), String::from("h3")]
                );
            }
            Err(_) => panic!("WTF?!?!"),
        }

        let result = tab.set_headers(vec![
            String::from("h3"),
            String::from("h3"),
            String::from("h3"),
            String::from("h3"),
        ]);
        match result {
            Ok(_) => panic!("WTF!!??"),
            Err(err) => {
                assert_eq!(
                    err,
                    "Columns or headers already set or wrong number of headers"
                );
                assert_eq!(
                    tab.headers,
                    vec![String::from("h1"), String::from("h2"), String::from("h3")]
                );
            }
        }
    }

    #[test]
    fn add_object_test() {
        let mut tab: Table = Table::new();

        _ = tab.set_headers(vec![
            String::from("h1"),
            String::from("h2"),
            String::from("h3"),
        ]);
        let name1 = match tab.add_object(
            String::from("name1"),
            vec![String::from("h1"), String::from("h2"), String::from("h3")],
        ) {
            Ok(name1) => name1,
            Err(err) => panic!("WTF?!?! {}", err),
        };

        assert_eq!(tab.rows, 1);
        assert_eq!(name1, String::from("name1"));
        assert_eq!(tab.names[0], String::from("name1"));
        assert_eq!(
            tab.values[0],
            vec![String::from("h1"), String::from("h2"), String::from("h3")]
        );
        for i in 0..3 {
            assert_eq!(tab.values[0][i], format!("h{}", i + 1));
        }
    }

    #[test]
    fn print_test() {
        let mut tab: Table = Table::new();
        _ = tab.set_headers(vec![
            String::from("h1"),
            String::from("h2"),
            String::from("h3"),
        ]);

        _ = tab.add_object(
            String::from("o1"),
            vec!["o1h1".to_string(), "o1h2".to_string(), "o1h3".to_string()],
        );
        _ = tab.add_object(
            String::from("o2"),
            vec!["o2h1".to_string(), "o2h2".to_string(), "o2h3".to_string()],
        );
        _ = tab.add_object(
            String::from("o3"),
            vec!["o3h1".to_string(), "o3h2".to_string(), "o3h3".to_string()],
        );
        _ = tab.add_object(
            String::from("o4"),
            vec!["o4h1".to_string(), "o4h2".to_string(), "o4h3".to_string()],
        );
        _ = tab.add_object(
            String::from("o5"),
            vec!["o5h1".to_string(), "o5h2".to_string(), "o5h3".to_string()],
        );

        assert_eq!(
            tab.get_as_string(true),
"+----+------+------+------+
|    |  h1  |  h2  |  h3  |
+----+------+------+------+
| o1 | o1h1 | o1h2 | o1h3 |
+----+------+------+------+
| o2 | o2h1 | o2h2 | o2h3 |
+----+------+------+------+
| o3 | o3h1 | o3h2 | o3h3 |
+----+------+------+------+
| o4 | o4h1 | o4h2 | o4h3 |
+----+------+------+------+
| o5 | o5h1 | o5h2 | o5h3 |
+----+------+------+------+\n"
        );
        assert_eq!(
            tab.get_as_string(false),
"+----+------+------+------+
|    |  h1  |  h2  |  h3  |
+----+------+------+------+
| o1 | o1h1 | o1h2 | o1h3 |
| o2 | o2h1 | o2h2 | o2h3 |
| o3 | o3h1 | o3h2 | o3h3 |
| o4 | o4h1 | o4h2 | o4h3 |
| o5 | o5h1 | o5h2 | o5h3 |
+----+------+------+------+\n"
        );
        
        _ = tab.change_flow_dir();
        assert_eq!(
            tab.get_as_string(true),
"+----+------+------+------+------+------+
|    |  o1  |  o2  |  o3  |  o4  |  o5  |
+----+------+------+------+------+------+
| h1 | o1h1 | o2h1 | o3h1 | o4h1 | o5h1 |
+----+------+------+------+------+------+
| h2 | o1h2 | o2h2 | o3h2 | o4h2 | o5h2 |
+----+------+------+------+------+------+
| h3 | o1h3 | o2h3 | o3h3 | o4h3 | o5h3 |
+----+------+------+------+------+------+\n"
        );
        assert_eq!(
            tab.get_as_string(false),
"+----+------+------+------+------+------+
|    |  o1  |  o2  |  o3  |  o4  |  o5  |
+----+------+------+------+------+------+
| h1 | o1h1 | o2h1 | o3h1 | o4h1 | o5h1 |
| h2 | o1h2 | o2h2 | o3h2 | o4h2 | o5h2 |
| h3 | o1h3 | o2h3 | o3h3 | o4h3 | o5h3 |
+----+------+------+------+------+------+\n"
        );
    }
}
