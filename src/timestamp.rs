pub fn sec_to_hms(sec: &i32) -> String {
    let hours = sec / 3600;
    let minutes = (sec - hours * 3600) / 60;
    let seconds = sec % 60;

    if hours > 0 {
        format!("{}:{:0>2}:{:0>2}", hours, minutes, seconds)
    } else {
        format!("{}:{:0>2}", minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long() {
        let seconds = 6643;
        assert_eq!(sec_to_hms(&seconds), "1:50:43");
    }

    #[test]
    fn test_short() {
        let seconds = 43;
        assert_eq!(sec_to_hms(&seconds), "0:43");
    }

    #[test]
    fn test_very_short() {
        let seconds = 3;
        assert_eq!(sec_to_hms(&seconds), "0:03");
    }
}
