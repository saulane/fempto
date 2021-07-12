use crate::Row;
use crate::Position;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Default)]
pub struct Document{
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document{
    pub fn open(filename: &str) -> Result<Self, std::io::Error>{
        let content = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in content.lines(){
            rows.push(Row::from(value));
        }
        Ok(Self{
            rows,
            file_name: Some(filename.to_string()),
        })
    }

    pub fn insert(&mut self,at: &Position, c: char){
        if c == '\n'{
            self.insert_new_line(at);
        }else{
            if at.y == self.len(){
                let mut row = Row::default();
                row.insert(0, c);
                self.rows.push(row);
    
            }else if at.y < self.len(){
                let row = self.rows.get_mut(at.y).unwrap();
                row.insert(at.x, c);
            }
        }
    }

    pub fn insert_new_line(&mut self, at: &Position){
        if at.x == self.rows[at.y].len(){
            let row = Row::default();
            self.rows.insert(at.y, row);
        }else{
            let row: &str = &self.rows.get_mut(at.y).unwrap().string.split_off(at.x);
            self.rows.get_mut(at.y).unwrap().update_len();

            self.rows.insert(at.y.saturating_add(1), Row::from(row));
        }
    }

    pub fn delete(&mut self, at: &Position){
        if at.y >= self.len(){
            return;
        }
        let row = self.rows.get_mut(at.y).unwrap();
        row.delete(at.x);
    }

    pub fn save(&self){
        match &self.file_name{
            Some(name) => {
                let mut file = File::create(name).expect("Failed to save file");
                for i in &self.rows{
                    writeln!(file, "{}", i.string).expect("Failed saving to file");
                }
            },
            None => {
                todo!();
            }
        }
    }

    pub fn row(&self, index:usize) -> Option<&Row>{
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool{
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize{
        self.rows.len()
    }
}