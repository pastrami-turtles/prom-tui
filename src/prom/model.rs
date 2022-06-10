use std::collections::HashMap;

pub struct Metric {
    pub details: MetricDetails,
    pub time_series: HashMap<String, TimeSeries>,
}

pub struct MetricDetails {
    pub name: String,
    pub docstring: String,
}

pub struct TimeSeries {
    pub labels: HashMap<String, String>,
    pub samples: Vec<Sample>,
}

pub enum Sample {
    GaugeSample(SingleValueSample),
    CounterSample(SingleValueSample),
    HistogramSample(HistogramSample),
}

pub struct SingleValueSample {
    pub timestamp: u64,
    pub value: f64,
}

pub struct HistogramSample {
    pub timestamp: u32,
    pub values: Vec<f64>,
}
