use crate::NowPlaying;

pub(crate) fn parse_duration(ms: u32) -> String {
    let d = ms / 1000;
    let h = d / 60;
    match (d / 60) / 60 {
        0 => format!("{:02}:{:02}", (h % 60), (d % 60)),
        _ => format!("{:02}:{:02}:{:02}", (h / 60), (h % 60), (d % 60)),
    }
}

pub(crate) async fn parse_volume(
    input: impl ToString,
) -> Result<impl ToString, Box<dyn std::error::Error>> {
    let amount = input.to_string();
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

pub(crate) async fn parse_position(
    input: impl ToString,
) -> Result<impl ToString, Box<dyn std::error::Error>> {
    let amount = input.to_string();
    let np = NowPlaying::new().await?;
    let (pos, total) = (np.position, np.duration);
    if amount.ends_with('%') {
        let percentage = amount.trim_end_matches('%');
        // +5%
        if percentage.starts_with('+') {
            let amount = percentage.trim_start_matches('+').parse::<u32>()?;
            let current = (pos / total) * 100;
            let desired = current + amount;
            let res = (desired * total) / 100;
            Ok(res.to_string())
        // -5%
        } else if percentage.starts_with('-') {
            let amount = percentage.trim_start_matches('-').parse::<u32>()?;
            let current = (pos / total) * 100;
            let desired = current - amount;
            let res = (desired * total) / 100;
            Ok(res.to_string())
        // 5%
        } else {
            let amount = percentage.parse::<u32>()?;
            let res = (total * amount) / 100;
            Ok(res.to_string())
        }
    } else {
        // +5 (seconds)
        if amount.starts_with('+') {
            let amount = amount.trim_start_matches('+').parse::<u32>()? * 1000;
            let res = pos + amount;
            Ok(res.to_string())
        // -5 (seconds)
        } else if amount.starts_with('-') {
            let amount = amount.trim_start_matches('-').parse::<u32>()? * 1000;
            let res = pos - amount;
            Ok(res.to_string())
        // 5 (treated the same as +5)
        } else {
            let amount = amount.parse::<u32>()? * 1000;
            let res = pos + amount;
            Ok(res.to_string())
        }
    }
}
