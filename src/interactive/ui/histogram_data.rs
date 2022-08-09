use chrono::{DateTime, Local, TimeZone};

use crate::prom::{Metric, Sample};

pub struct BucketData {
  pub bucket: String,
  pub value: f64,
  pub percentage: f64,
  pub distribution: f64,
}

impl BucketData {
  pub fn new(bucket: String, value: f64, percentage: f64, distribution: f64) -> Self {
    Self { bucket, value, percentage, distribution}
  }
}

pub struct HistogramData {
  pub data: Vec <BucketData>,
  pub time: DateTime<Local>,
  pub count: u64,
  pub sum: f64
}

impl HistogramData {
  pub fn parse(metric: &Metric, selected_label: &str) -> Option<Self> {
    let last_sample = &metric
        .time_series
        .get(selected_label)
        .expect("values for selected label")
        .samples.last();

      let mut data = vec![];
      let mut timestamp = 0;
      let mut count = 0;
      let mut sum = 0.0;

      if let Some(sample) = *last_sample {
        if let Sample::HistogramSample(histogram) = sample {
            timestamp = histogram.timestamp;
            count = histogram.count;
            sum = histogram.sum;
            for (index, bucket) in histogram.bucket_values.iter().enumerate() {
                let dist;
                if index == 0 {
                    dist = (bucket.value as f64 / histogram.count as f64) * 100.0;
                } else {
                    dist = (bucket.value as f64 - histogram.bucket_values[index - 1].value as f64)
                        / histogram.count as f64
                        * 100.0;
                }
                let percentage = (bucket.value as f64 / histogram.count as f64) * 100.0;
                data.push(BucketData::new(bucket.name.clone(), bucket.value as f64, percentage, dist))
            }
        }
    }
    if data.len() < 2 {
        return None;
    }

    if timestamp == 0 {
      return None;
    }

    let time = Local.timestamp(timestamp as i64, 0);


    Some(Self {
        data,
        time,
        count,
        sum
    })
}

}