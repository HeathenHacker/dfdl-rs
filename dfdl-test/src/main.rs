use std::collections::HashMap;
use std::error::Error;
use std::f64::consts::PI;
use std::fmt::Debug;

use dfdl::serde::{from_infoset, to_infoset};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Teststruct {
    pre_elem: u64,
    middle_map: HashMap<u64, String>,
    post_elem: f64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestB {
    nested: Vec<Vec<i32>>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct NestedMap {
    map_vec: HashMap<String, Vec<u64>>,
    vec_map: Vec<HashMap<String, u64>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MiddleArray {
    pre: u64,
    arr: Vec<u64>,
    post: u64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum TestEnum {
    Hello,
    World(u8),
    Multi(u8, MiddleArray),
    Other{first: u8, second: char},
}
fn main() -> Result<(), Box<dyn Error>> {


    fn roundtrip<T: Serialize + DeserializeOwned + PartialEq + Debug>(val: &T) -> Result<(), Box<dyn Error>> {
        let is = to_infoset(&val)?;
        println!("is: {is:#?}");
        
        let de: T = from_infoset(&is)?;
        //assert_eq!(*val, de, "element mismatch:\n{val:#?}\n\nvs\n\n{de:#?}");
        assert_eq!(*val, de);
        Ok(())
    }
    let val = 42;
    roundtrip(&val)?;

    let val = vec![6, 6, 6];
    roundtrip(&val)?;
    let val = MiddleArray {
        pre: 42,
        arr: vec![1,1,2,3,5,8],
        post: 666,
    };
    roundtrip(&val)?;
    let e_a = TestEnum::Hello;
    let e_b = TestEnum::World(5);
    let e_c = TestEnum::Multi(66, val);
    let e_d = TestEnum::Other { first: 42, second: 'f' };
    roundtrip(&e_a)?;
    roundtrip(&e_b)?;
    roundtrip(&e_c)?;
    roundtrip(&e_d)?;
    roundtrip(&vec![e_a, e_b, e_c, e_d])?;
    println!("hello");
    
    let val = TestB {
        nested: vec![vec![2; 2]; 2],
    };
    roundtrip(&val)?;
    let val = NestedMap {
        map_vec: HashMap::from([
            ("a".to_string(), vec![6, 6, 6]),
            ("b".to_string(), vec![4, 2]),
        ]),
        vec_map: vec![HashMap::from([
            ("a".to_string(), 1),
            ("b".to_string(), 1),
        ]), HashMap::from([
            ("c".to_string(), 2),
            ("d".to_string(), 3),
        ])],
    };
    roundtrip(&val)?;
    let val = Teststruct {
        pre_elem: 42,
        middle_map: HashMap::from([
            (66, "hello".to_string()),
            (324, "world".to_string()),
        ]),
        post_elem: 6.66,
    };
    roundtrip(&val)?;
    let val = Some(666);
    roundtrip(&val)?;
    
    let val = None::<i32>;
    roundtrip(&val)?;
    let val = (55, 66, 77);
    roundtrip(&val)?;
    let val = ();
    roundtrip(&val)?;
    println!("finished");
    /*let mut xml_src = String::new();
    let mut file = BufReader::new(File::open("test.xsd")?);
    file.read_to_string(&mut xml_src)?;
    let schema = Schema::new(&xml_src)?;*/
    Ok(())
}
