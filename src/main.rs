
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::Deserialize;
use serde_yaml::Value;


// @NOTE: Still needs more data to be complete and robust


#[derive(Debug)]
enum MotionType {
    TRANSLATION,
    ROTATION,
    SCALING,
}

#[derive(Debug)]
struct AnimationCurve {
    time: f32,
    value: f32,
    inSlope: f32,
    outSlope: f32,
    inWeight: f32,
    outWeight: f32,
}

#[derive(Debug)]
struct AnimationClipSettings {
    start_time: f32,
    stop_time: f32,
}


#[derive(Debug)]
struct BoneCurveData {
    attribute: String,
    motion_type: MotionType,
}

#[derive(Debug)]
struct AnimationData {
    name: String,
    animationClipSettings: AnimationClipSettings,
    animation_curves: Vec<AnimationCurve>,
}


fn main() {

    let path = Path::new("resources\\Jog.anim");

    let mut file = match File::open(&path) {
        Err(_err) => panic!("Couldnt open file"),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_err) => panic!("Couldnt read file"),
        Ok(_) => {
            // println!("File contains: {}", s),
        }
    };


    let mut count: u32 = 0;

    // @NOTE: Cleanup
    for document in serde_yaml::Deserializer::from_str(&s) {
        let value = Value::deserialize(document);
        if let Ok(Value::Mapping(m)) = value {
            let animation_clip = match m.get("AnimationClip") {
                None => panic!("Failed to read AnimationClip"),
                Some(data) => data,
            };

            if let Ok(Value::Mapping(m)) = animation_clip {
            }
        }
        // println!("{:?}", value);
    }

    println!("Count {}", count);
}
