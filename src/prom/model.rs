pub enum Metric {
    GaugeMetric(GaugeMetric),
    CounterMetric(CounterMetric),
    HistogramMetric(HistogramMetric),
}

pub struct GaugeMetric {
    pub name: String,
    pub description: String,
    pub value: f64,
}

pub struct CounterMetric {
    pub name: String,
    pub description: String,
    pub value: f64,
}

pub struct HistogramMetric {
    pub name: String,
    pub description: String,
}