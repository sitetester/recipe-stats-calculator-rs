use std::fs::File;
use std::io::{self, BufReader, Read};
use std::sync::mpsc::Sender;

use serde::de::DeserializeOwned;
use serde_json::{self, Deserializer};

use crate::recipe::calculator::RecipeData;

pub(crate) mod ExpectedOutput;
pub(crate) mod calculator;

pub struct StatsLoader {}

impl StatsLoader {
    pub fn load(path: &str, tx: Sender<RecipeData>) -> Result<(), Box<dyn std::error::Error>> {
        let reader = BufReader::new(File::open(path)?);
        for recipe in Self::iter_json_array(reader) {
            let recipe: RecipeData = recipe.unwrap();
            tx.send(recipe).unwrap();
        }

        Ok(())
    }

    fn read_skipping_ws(mut reader: impl Read) -> io::Result<u8> {
        loop {
            let mut byte = 0u8;
            reader.read_exact(std::slice::from_mut(&mut byte))?;
            if !byte.is_ascii_whitespace() {
                return Ok(byte);
            }
        }
    }

    fn iter_json_array<T: DeserializeOwned, R: Read>(
        mut reader: R,
    ) -> impl Iterator<Item = Result<T, io::Error>> {
        let mut started = false;

        std::iter::from_fn(move || {
            if !started {
                started = true;
                match Self::read_skipping_ws(&mut reader) {
                    Ok(b'[') => (),
                    Ok(_) => return Some(Err(io::Error::new(io::ErrorKind::Other, "[ not found"))),
                    Err(e) => return Some(Err(e)),
                }
            } else {
                match Self::read_skipping_ws(&mut reader) {
                    Ok(b',') => (),
                    Ok(b']') => return None,
                    Ok(_) => {
                        return Some(Err(io::Error::new(
                            io::ErrorKind::Other,
                            ", or ] not found",
                        )));
                    }
                    Err(e) => return Some(Err(e)),
                }
            }

            let mut stream = Deserializer::from_reader(&mut reader).into_iter::<T>();
            match stream.next() {
                Some(result) => Some(result.map_err(Into::into)),
                None => Some(Err(io::Error::new(io::ErrorKind::Other, "premature EOF"))),
            }
        })
    }
}
