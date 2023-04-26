use nym_sdk::mixnet;
pub type Address = mixnet::Recipient;

#[cfg(test)]
pub mod testing {
    use log::Level;
    extern crate testing_logger;

    pub fn before_each() {
        testing_logger::setup();
    }

    pub fn validate_logs(trace_target: String, expected_logs: &[(&str, Level)]) {
        testing_logger::validate(|all_logs| {
            let captured_logs: Vec<&testing_logger::CapturedLog> = all_logs
                .iter()
                .filter(|x| x.target == trace_target)
                .collect();

            assert_eq!(captured_logs.len(), expected_logs.len());

            captured_logs
                .iter()
                .enumerate()
                .map(|(i, log)| (log.body.as_str(), log.level, i))
                .zip(
                    expected_logs
                        .iter()
                        .enumerate()
                        .map(|(i, ex)| (ex.0, ex.1, i)),
                )
                .for_each(|(log_tup, expected_tup)| assert_eq!(log_tup, expected_tup.clone()));
        });
    }
}
