pub struct Percentage {
    pub total: i16,
    pub value: i16,
    pub percentage: i8,
}

impl Percentage {
    pub fn from_total_and_value(total: i16, value: i16) -> Percentage {
        let percentage_result: f32 = value as f32 / total as f32 * 100.0;
        println!("{:?}", percentage_result);
        let percentage = percentage_result as i8;
        Percentage{total: total as i16, value: value as i16, percentage}
    }

    pub fn from_total_and_percentage(total: i16, percentage: i8) -> Percentage {
        let value = total as f32 / 100.0 * percentage as f32;
        Percentage{total: total as i16, value: value as i16, percentage}
    }
}

#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn from_total_and_value() {
       let p = Percentage::from_total_and_value(200, 50);
       assert_eq!(25, p.percentage);
   }

   #[test]
   fn from_total_and_percentage() {
       let p = Percentage::from_total_and_percentage(200, 25);
       assert_eq!(50, p.value);
   }
}
