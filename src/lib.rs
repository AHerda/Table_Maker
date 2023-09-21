use std::error::Error;

#[derive(PartialEq, Debug)]
enum Dir {
    ROW,
    COLUMN,
}

struct Table<'a> {
    flow_direction: Dir,
    columns: u16,
    rows: u16,
    headers: Vec<String>,
    names: Vec<String>,
    objects: Vec<&'a [String]>,
}

impl<'a> Table<'a> {
    fn new() -> Table<'a> {
        Table { flow_direction: Dir::COLUMN, columns: 0, rows: 0, headers: Vec::new(), names: Vec::new(), objects: Vec::new() }
    }

    fn set_columns(&mut self, columns: u16) -> Result<u16, &'static str> {
        if self.columns == 0 {
            self.columns = columns;
            return Ok(columns)
        }
        Err("Columns already set")
    }

    fn set_headers(&mut self, headers: Vec<String>) -> Result<u16, &'static str> {
        let len: u16 = headers.len() as u16;
        if self.columns == 0 {
            self.columns = len;
        }
        else if self.columns != len || self.headers != Vec::<String>::new() {
            return Err("Columns or headers already set or wrong number of headers")
        }
        self.headers = headers;
        Ok(len)
    }

    fn add_object(&mut self, name: String, ) {
        if self.flow_direction == Dir::COLUMN {
            self.rows += 1;
            self.names.push(name);
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let result: Table = Table::new();
        assert_eq!(result.flow_direction, Dir::COLUMN);
        assert_eq!(result.columns, 0);
        assert_eq!(result.rows, 0);
        assert_eq!(result.headers, Vec::<String>::new());
        assert_eq!(result.names, Vec::<String>::new());
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

        let result = tab.set_headers(vec![String::from("h1"), String::from("h2"), String::from("h3")]);
        match result {
            Ok(len) => {
                assert_eq!(len, 3);
                assert_eq!(tab.headers, vec![String::from("h1"), String::from("h2"), String::from("h3")])
            }
            Err(_) => panic!("WTF?!?!")
        }

        let result = tab.set_headers(vec![String::from("h3"), String::from("h3"), String::from("h3")]);
        match result {
            Ok(_) => panic!("WTF!!??"),
            Err(err) => {
                assert_eq!(err, "Columns or headers already set or wrong number of headers");
                assert_eq!(tab.headers, vec![String::from("h1"), String::from("h2"), String::from("h3")]);
            }
        }
    }

    #[test]
    fn set_headers_test_2() {
        let mut tab: Table = Table::new();

        let result = tab.set_headers(vec![String::from("h1"), String::from("h2"), String::from("h3")]);
        match result {
            Ok(len) => {
                assert_eq!(len, 3);
                assert_eq!(tab.headers, vec![String::from("h1"), String::from("h2"), String::from("h3")])
            }
            Err(_) => panic!("WTF?!?!")
        }

        let result = tab.set_headers(vec![String::from("h3"), String::from("h3"), String::from("h3"), String::from("h3")]);
        match result {
            Ok(_) => panic!("WTF!!??"),
            Err(err) => {
                assert_eq!(err, "Columns or headers already set or wrong number of headers");
                assert_eq!(tab.headers, vec![String::from("h1"), String::from("h2"), String::from("h3")]);
            }
        }
    }
}
