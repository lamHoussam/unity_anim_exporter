
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::Deserialize;
use serde_yaml::{Value, Mapping};

// @NOTE: Still needs more data to be complete and robust

#[derive(Debug)]
enum MotionType {
    TRANSLATION,
    ROTATION,
    SCALING,
}

#[derive(Debug)]
struct AnimationCurvePoint {
    time: f32,
    value: f32,
    in_slope: f32,
    out_slope: f32,
    in_weight: f32,
    out_weight: f32,
}

#[derive(Debug)]
struct AnimationClipSettings {
    start_time: f32,
    stop_time: f32,
}


#[derive(Debug)]
struct AnimationCurveData {
    attribute: String,
    motion_type: MotionType,
    axis: String,
    animation_curves: Vec<AnimationCurvePoint>,
}

#[derive(Debug)]
struct AnimationData {
    name: String,
    animation_clip_settings: AnimationClipSettings,
    animation_curves_data: Vec<AnimationCurveData>,
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

            if let Value::Mapping(m) = animation_clip {
                let animation_name = match animation_clip.get("m_Name") {
                    Some(Value::String(data)) => data,
                    _ => panic!("Failed to read m_Name"),
                };

                println!("Animation name: {}", animation_name);

                let float_curves = match animation_clip.get("m_FloatCurves") {
                    Some(Value::Sequence(data)) => data,
                    _ => panic!("Failed to read m_FloatCurves"),
                };

                for float_curve in float_curves {
                    let float_curve_mapping = match float_curve {
                        Value::Mapping(data) => data,
                        _ => panic!("Failed to read Float curve mapping"),
                    };

                    // @TODO: Read attribute from each float curve
                    let curve_graph = match float_curve_mapping.get("curve") {
                        Some(Value::Mapping(data)) => data,
                        _ => panic!("Failed to read curve"),
                    };
                    // println!("Float curve: {:?}", curve_graph);
                    let curve_points = match curve_graph.get("m_Curve") {
                        Some(Value::Sequence(data)) => data,
                        _ => panic!("Failed to read m_Curve"),
                    };

                    for curve_point in curve_points {
                        let curve_point_mapping = match curve_point {
                            Value::Mapping(data) => data,
                            _ => panic!("Failed to read curve point mapping"),
                        };
                        println!("Time: {:?}", curve_point_mapping.get("time"));
                    }


                    // for curve_point in curve_points {
                    //     println!("Point: {:?}", curve_point);
                    // }
                }
            }
        }
        // println!("{:?}", value);
    }

    println!("Count {}", count);
}
