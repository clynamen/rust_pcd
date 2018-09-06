// #[macro_use]
// #[macro_use] extern crate derive_builder;
  //  FIELDS x y z rgb imX imY
  //  SIZE 4 4 4 4 4 4
  //  TYPE F F F F F F
  //  COUNT 1 1 1 1 1 1
  //  WIDTH 3547
  //  HEIGHT 1
  //  VIEWPOINT 0 0 0 1 0 0 0
  //  POINTS 3547
  //  DATA ascii

extern crate itertools;
use std::io::Lines;
use std::io::BufRead;
use std::io::Error;
use std::num::ParseIntError;
use std::num::ParseFloatError;
use self::itertools::multizip;

#[derive(Debug, Clone, PartialEq)]
enum FieldType {
    F, 
    D
}

impl Default for FieldType {
    fn default() -> FieldType { FieldType::F }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct PcdField {
    name: String,
    size: u32,
    type_: FieldType,
    count: u32,
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Viewpoint {
    ar: Vec<f32>
}

#[derive(Debug, Clone, PartialEq)]
enum PcdFileDataFormat {
    BINARY,
    ASCII
}

impl Default for PcdFileDataFormat {
    fn default() -> PcdFileDataFormat { PcdFileDataFormat::ASCII }
}


#[derive(Default, Builder, Debug, Clone, PartialEq)]
#[builder(setter(into))]
struct PcdHeader {
    fields: Vec<PcdField>,
    width: u64,
    height: u64,
    viewpoint: Viewpoint,
    points: u64,
    data: PcdFileDataFormat
}

// impl PcdHeader {
//     fn empty() -> PcdHeader {
//         PcdHeader {
//             fields: Vec::new(),
//             width: 0,
//             height: 0,
//             viewpoint: Viewpoint{ ar: Vec::new() } ,
//             points: 0,
//             data: PcdFileDataFormat::BINARY
//         }
//     }
// }

#[derive(Debug)]
pub enum HeaderError {
    IntError,
    FloatError,
    ErrorDesc(String),
    FieldIncomplete
}

impl From<ParseIntError> for HeaderError {
    fn from(error: ParseIntError) -> HeaderError { HeaderError::IntError }
}

impl From<String> for HeaderError {
    fn from(error: String) -> HeaderError { HeaderError::IntError }
}

// impl From<std::option::NoneError> for HeaderError {
//     fn from(error: String) -> HeaderError { HeaderError::FieldIncomplete }
// }

fn read_pcd_header_from_lines<T>(lines: Lines<T>) -> Result<PcdHeader, HeaderError> 
where T: BufRead  
{
    let mut header = PcdHeaderBuilder::default();

    let tmp_fields : Vec<PcdField> = Vec::new();

    let mut field_names : Vec<String> = Vec::new();
    let mut field_sizes : Vec<u32> = Vec::new();
    let mut field_types : Vec<FieldType> = Vec::new();
    let mut field_counts : Vec<u32> = Vec::new();

    for line in lines {
        if line.is_ok() {
            let line = line.unwrap();
            let mut words = line.split_whitespace();
            let first_word  = words.nth(0);
            match first_word {
                Some(first_word) => {
                    match first_word {
                        "FIELDS" => {
                            words.for_each(|x| field_names.push( String::from(x) )); 
                        }
                        "SIZE" => {
                            for word in words {
                                let size_ : u32 = word.parse::<u32>()?;
                                field_sizes.push(size_); 
                            }
                        }
                        "TYPE" => {
                            fn to_type(x: &str) -> Result<FieldType, HeaderError>  {
                                match x {
                                    "F" => Ok(FieldType::F),
                                    "D" => Ok(FieldType::D),
                                    _ =>   Err(HeaderError::FieldIncomplete)
                                } 
                            };
                            let parsed_words : Result<Vec<FieldType>, HeaderError> = words.map(|x| to_type(x)).collect();
                            match parsed_words {
                                Ok(parsed_field_types )  => {parsed_field_types.iter().for_each( |x| field_types.push(x.clone()) ) }
                                _ => {return Err(HeaderError::FieldIncomplete)}
                            } 
                        }
                        "COUNT" => {
                            let mut counts_res : Result<Vec<u32>, ParseIntError> = words.map(|x| x.parse::<u32>()).collect();
                            match counts_res {
                                Ok(counts) => counts.iter().for_each(|x| field_counts.push(x.clone())),
                                _ => {return Err(HeaderError::IntError)}
                            }
                            
                        }
                        "VIEWPOINT" => {
                            let viewpoint_ar : Result<Vec<f32>, ParseFloatError> = words.map(|w| w.parse::<f32>() ).collect();
                            match viewpoint_ar {
                                Ok(ar) => { header.viewpoint(Viewpoint{ar:ar}); }
                                _ => {return Err(HeaderError::FloatError)}
                            }
                        }
                        "POINTS" => {
                            match words.nth(0) {
                                Some(ref s) => {
                                    let points = s.parse::<u32>()?;
                                    header.points(points);
                                }
                                _ => {
                                    return Err(HeaderError::FieldIncomplete);
                                }
                            }
                        }

                        "WIDTH" => {
                            match words.nth(0) {
                                Some(ref s) => {
                                    let width = s.parse::<u32>()?;
                                    header.width(width);
                                }
                                _ => {
                                    return Err(HeaderError::FieldIncomplete);
                                }
                            }
                        }
                        "HEIGHT" => {
                            match words.nth(0) {
                                Some(ref s) => {
                                    let height = s.parse::<u32>()?;
                                    header.height(height);
                                }
                                _ => {
                                    return Err(HeaderError::FieldIncomplete);
                                }
                            }
                        }
                        "DATA" => {
                            match words.nth(0) {
                                Some("ascii") => {
                                    header.data(PcdFileDataFormat::ASCII);
                                }
                                _ => {
                                    return Err(HeaderError::FieldIncomplete);
                                }
                            }
                        }

                        
                        _ => {

                        }
                    }

                },
                None => continue
            }

        } else {
            break;
        }

    }


    let mut field_tmp_vec : Vec<PcdField> = Vec::new();

    println!("{:?}", field_names);
    println!("{:?}", field_sizes);
    println!("{:?}", field_types);
    println!("{:?}", field_counts);

    for (name, size, type_, count) in multizip( (field_names, field_sizes, field_types, field_counts) ) {
        field_tmp_vec.push( PcdField {
            name: name,
            size: size,
            type_: type_,
            count: count
        })
    }

    header.fields(field_tmp_vec);

    match  header.build() {
        Ok(header) => Ok(header),
        Err(s)  => Err(HeaderError::ErrorDesc(s))
    }
}

mod tests {
    use super::*;

    #[test]
    fn empty_lines() {
        assert!(read_pcd_header_from_lines(b"".lines()).is_err(), "")
    }

    #[test]
    fn header() {
        let HEADER = br#"
   FIELDS x y z rgb imX imY
   SIZE 4 4 4 4 4 4
   TYPE F F F F F F
   COUNT 1 1 1 1 1 1
   WIDTH 3547
   HEIGHT 1
   VIEWPOINT 0 0 0 1 0 0 0
   POINTS 3547
   DATA ascii
   "#;
        //assert!(read_pcd_header_from_lines(HEADER.lines()).is_ok(), "")

        //read_pcd_header_from_lines(HEADER.lines()).unwrap();
        let header = read_pcd_header_from_lines(HEADER.lines());
        assert!(header.is_ok(), "");
        let correct = PcdHeader {
            fields: vec![ 
                PcdField{name: String::from("x"), size: 4u32, type_: FieldType::F, count: 1u32 }  ,
                PcdField{name: String::from("y"), size: 4u32, type_: FieldType::F, count: 1u32 }  ,
                PcdField{name: String::from("z"), size: 4u32, type_: FieldType::F, count: 1u32 }  ,
                PcdField{name: String::from("rgb"), size: 4u32, type_: FieldType::F, count: 1u32 },
                PcdField{name: String::from("imX"), size: 4u32, type_: FieldType::F, count: 1u32 },
                PcdField{name: String::from("imY"), size: 4u32, type_: FieldType::F, count: 1u32 } 
                ],
            width: 3547u64,
            height: 1u64,
            viewpoint: Viewpoint {ar: vec![0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32]},
            points: 3547u64,
            data: PcdFileDataFormat::ASCII 
        };
        assert_eq!(header.unwrap(), correct)
    }


}