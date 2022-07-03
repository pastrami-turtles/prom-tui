use chrono::{DateTime, Local, TimeZone};

use crate::prom::{Metric, Sample};

pub struct GraphData {
    pub data: Vec<(f64, f64)>,
    pub first_time: DateTime<Local>,
    pub last_time: DateTime<Local>,
    pub x_max: f64,
    pub x_min: f64,
    pub y_max: f64,
    pub y_min: f64,
}

impl GraphData {
    pub fn parse(metric: &Metric, selected_label: &str) -> Option<Self> {
        let samples = &metric
            .time_series
            .get(selected_label)
            .expect("values for selected label")
            .samples;
        let data: Vec<(f64, f64)> = samples
            .iter()
            .map(|entry| {
                let (timestamp, value) = match entry {
                    Sample::GaugeSample(single_value) => {
                        (single_value.timestamp, single_value.value)
                    }
                    Sample::CounterSample(single_value) => {
                        (single_value.timestamp, single_value.value)
                    }
                    _ => unimplemented!(),
                };
                (timestamp as f64, value)
            })
            .collect();
        if data.len() < 2 {
            return None;
        }

        let first_time = Local.timestamp(data.first().unwrap().0 as i64, 0);
        let last_time = Local.timestamp(data.last().unwrap().0 as i64, 0);
        let x_min = data.first().unwrap().0;
        let x_max = data.last().unwrap().0;

        let y_min = data.iter().map(|(_, v)| *v).fold(f64::MAX, f64::min);
        let y_max = data
            .iter()
            .map(|(_, v)| *v)
            .fold(f64::NEG_INFINITY, f64::max);

        Some(Self {
            data,
            first_time,
            last_time,
            x_max,
            x_min,
            y_max,
            y_min,
        })
    }
}
