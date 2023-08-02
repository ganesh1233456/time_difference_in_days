#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod myapp {

    #[ink(storage)]
    pub struct Myapp {
        value: bool,
        initial_time: Timestamp,
        flip_time: Timestamp,
        days: u64,
    }

    impl Myapp {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let initial_time = Self::env().block_timestamp();
            Self {
                value: init_value,
                initial_time,
                flip_time: 0,
                days: 0,
            }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            let flip_time = self.env().block_timestamp();
            self.flip_time = flip_time;
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn calculate_days_based_on_time(&self) {
            let initial_time = self.initial_time;
            let flip_time = self.flip_time;

            // calculate the time difference between flip_time and initial_time
            let time_difference = flip_time.checked_sub(initial_time);

            // convert the time difference to the number of days
            let days = time_difference
                .as_secs()
                .checked_div(86400)
                .unwrap_or(0);

            self.days = days;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
