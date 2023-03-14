use crate::NowPlaying;

pub fn parse_duration(ms: i32) -> String {
    let d = ms / 1000;
    let h = d / 60;
    if (d / 60) / 60 == 0 {
        format!("{:02}:{:02}", (h % 60), (d % 60))
    } else {
        format!("{:02}:{:02}:{:02}", (h / 60), (h % 60), (d % 60))
    }
}

pub async fn parse_volume(amount: String) -> Result<String, Box<dyn std::error::Error>> {
    let volume = NowPlaying::new().await?.volume;
    if amount.starts_with('+') {
        let current = (volume * 100.0) as u32;
        let res = current + amount.trim_start_matches('+').parse::<u32>()?;
        Ok(res.to_string())
    } else if amount.starts_with('-') {
        let current = (volume * 100.0) as u32;
        let res = current - amount.trim_start_matches('-').parse::<u32>()?;
        Ok(res.to_string())
    } else {
        Ok(amount)
    }
}

pub async fn parse_position(amount: String) -> Result<i32, Box<dyn std::error::Error>> {
    let np = NowPlaying::new().await?;
    let (pos, total) = (np.position, np.duration);
    if amount.ends_with('%') {
        let percentage = amount.trim_end_matches('%');
        // 5%
        let amount = percentage.parse::<i32>()?;
        let res = (total * amount) / 100;
        Ok(res)
    } else {
        // +5 (seconds)
        if amount.starts_with('+') {
            let amount = amount.trim_start_matches('+').parse::<i32>()? * 1000;
            let res = pos + amount;
            Ok(res)
        // -5 (seconds)
        } else if amount.starts_with('-') {
            let amount = amount.trim_start_matches('-').parse::<i32>()? * 1000;
            let res = pos - amount;
            Ok(res)
        // 5 (treated the same as +5)
        } else {
            let amount = amount.parse::<i32>()? * 1000;
            let res = pos + amount;
            Ok(res)
        }
    }
}
