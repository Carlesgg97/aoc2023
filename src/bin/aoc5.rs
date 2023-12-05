use std::{io, error::Error, fmt, vec};

type AlmanacSeed = u32;

#[derive(Debug)]
struct AlmanacSeeds(Vec<AlmanacSeed>);

impl AlmanacSeeds {
    const SEEDS_PREFIX: &'static str = "seeds: ";

    fn from_reader(reader: &mut dyn io::BufRead) -> Result<Self, Box<dyn Error>> {
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let seeds: Result<Vec<AlmanacSeed>, String> = line
            .strip_prefix(Self::SEEDS_PREFIX).ok_or(format!("failed to strip seeds prefix: {line}"))?
            .split_whitespace()
            .map(|n| n.parse::<AlmanacSeed>().map_err(|pie| format!("failed to parse seed: {}", pie).into()))
            .collect();

        line.clear();
        reader.read_line(&mut line)?;
        if !line.trim().is_empty() {
            return Err("expected an empty line".into());
        }

        Ok(Self(seeds?))
    }
    
}

#[derive(Debug)]
enum AlmanacMapError {
    Eof,
    ParseError(String)
}

impl fmt::Display for AlmanacMapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlmanacMapError::Eof => write!(f, "reached end of file"),
            AlmanacMapError::ParseError(s) => write!(f, "failed to parse: {}", s)
        }
    }
}

#[derive(Debug)]
struct CategoryMapping {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32
}

#[derive(Debug)]
#[allow(dead_code)]
struct AlmanacMap {
    source_category: String,
    destination_category: String,
    mappings: Vec<CategoryMapping>
}

impl AlmanacMap {
    const CATEGORY_SEPARATOR: &'static str = "-to-";
    const CATEGORY_SUFFIX: &'static str = " map:\n";

    fn from_reader(reader: &mut dyn io::BufRead) -> Result<Self, AlmanacMapError> {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => return Err(AlmanacMapError::Eof),
            Err(e) => return Err(AlmanacMapError::ParseError(format!("failed to read line: {}", e))),
            _ => {}
        };

        let (source_category, destination_category) = line
            .strip_suffix(Self::CATEGORY_SUFFIX)
            .ok_or(AlmanacMapError::ParseError(format!("failed to strip category suffix: {line}")))?
            .split_once(Self::CATEGORY_SEPARATOR)
            .ok_or(AlmanacMapError::ParseError(format!("failed to split category: {line}")))?;

        let mut mappings = Vec::new();

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) | Ok(1) => break,
                Err(e) => return Err(AlmanacMapError::ParseError(format!("failed to read line: {}", e))),
                _ => {}
            };

            let mut parts = line.split_ascii_whitespace();
            let dest_range_start = parts.next().ok_or(AlmanacMapError::ParseError("failed to get destination range start".to_string()))?.parse::<u32>().map_err(|e| AlmanacMapError::ParseError(format!("failed to parse destination range start: {}", e)))?;
            let src_range_start = parts.next().ok_or(AlmanacMapError::ParseError("failed to get source range start".to_string()))?.parse::<u32>().map_err(|e| AlmanacMapError::ParseError(format!("failed to parse source range start: {}", e)))?;
            let range_length = parts.next().ok_or(AlmanacMapError::ParseError("failed to get range length".to_string()))?.parse::<u32>().map_err(|e| AlmanacMapError::ParseError(format!("failed to parse range length: {}", e)))?;
            if parts.next().is_some() {
                return Err(AlmanacMapError::ParseError("Expected three parts".to_string()));
            }

            mappings.push(CategoryMapping {
                destination_range_start: dest_range_start,
                source_range_start: src_range_start,
                range_length: range_length
            });
        }

        mappings.sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));

        Ok(Self {
            source_category: source_category.to_string(),
            destination_category: destination_category.to_string(),
            mappings
        })
    }

    fn lowest_location_number(&self, seed: AlmanacSeed) -> u32 {
        
        for mapping in &self.mappings {
            if seed >= mapping.source_range_start && seed < mapping.source_range_start + mapping.range_length {
                let offset = seed - mapping.source_range_start;
                return mapping.destination_range_start + offset;
            }

            if mapping.source_range_start > seed {
                break;
            }
        }

        seed
    }
}

fn main() {
    let mut reader = io::stdin().lock();

    let seeds = AlmanacSeeds::from_reader(&mut reader).expect("failed to parse seeds");
    let mut almanac_maps = vec![];

    loop {
        match AlmanacMap::from_reader(&mut reader) {
            Ok(map) => {
                almanac_maps.push(map);
            },
            Err(AlmanacMapError::Eof) => break,
            Err(AlmanacMapError::ParseError(s)) => panic!("{}", s)
        }
    }

    let result = seeds.0.iter().map(|seed| {
        let mut seed = *seed;
        for map in &almanac_maps {
            seed = map.lowest_location_number(seed);
        }
        seed
    }).min();

    println!("{:?}", result.unwrap());
}