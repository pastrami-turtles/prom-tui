use chrono::{DateTime, Local, TimeZone};

use crate::prom::{Metric, Sample};

pub struct BucketData {
  bucket: String,
  value: u64,
  percentage: f64,
  inc_per_bucket: u64,
  inc_per_bucket_percentage: f64,
}

impl BucketData {
  pub fn new(bucket: String, value: u64, percentage: f64,inc_per_bucket: u64, inc_per_bucket_percentage: f64) -> Self {
    Self { bucket, value, percentage, inc_per_bucket, inc_per_bucket_percentage }
  }

  pub fn get_bucket(&self) -> &String {
    &self.bucket
  }

  pub fn get_value(&self) -> u64 {
    self.value
  }

  pub fn get_percentage(&self) -> f64 {
    self.percentage
  }

  pub fn get_inc_per_bucket(&self) -> u64 {
    self.inc_per_bucket
  }

  pub fn get_inc_per_bucket_percentage(&self) -> f64 {
    self.inc_per_bucket_percentage
  }
}

pub struct HistogramData {
  pub data: Vec<BucketData>,
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
                let inc_per_bucket;
                if index == 0 {
                    inc_per_bucket = bucket.value;
                } else {
                    inc_per_bucket = bucket.value - histogram.bucket_values[index - 1].value;
                }
                let percentage = (bucket.value as f64 / histogram.count as f64) * 100.0;
                let inc_per_bucket_percentage = (inc_per_bucket as f64) / (histogram.count as f64)* 100.0;
                data.push(BucketData::new(bucket.name.clone(), bucket.value, percentage, inc_per_bucket, inc_per_bucket_percentage))
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