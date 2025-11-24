use anyhow::Result;
use std::fmt::Write;

use crate::cpu_features::CpuFeatures;
use crate::cpu_info::CpuInfo;

pub fn format_output(cpu_info: &CpuInfo, features: &[CpuFeatures]) -> Result<String> {
    let mut output = String::new();

    // Format CPU information
    print!("\n");
    writeln!(output, "{}", "CPU INFORMATION")?;
    writeln!(output, "CPU Name: {}", cpu_info.name)?;
    writeln!(output, "Cores: {}", cpu_info.physical_cores)?;
    writeln!(output, "Threads: {}", cpu_info.logical_cores)?;
    writeln!(output, "Base Speed: {} MHz", cpu_info.base_speed)?;
    writeln!(output, "Boost Speed: {} MHz (estimated)", cpu_info.boost_speed)?;

    // Format CPU features
    writeln!(output, "\n{}", "CPU FEATURES")?;

    // Find the longest feature name to align the output
    let max_width = features.iter()
        .map(|f| f.name.len())
        .max()
        .unwrap_or(0);

    for feature in features {
        writeln!(output, "{}", feature.format(max_width))?;
    }

    // Format summary
    let total = features.len();
    let available = features.iter().filter(|f| f.supported).count();
    let unavailable = total - available;

    writeln!(output, "\n{}", "SUMMARY")?;
    writeln!(output, "Total functions: {}", total)?;
    writeln!(output, "Available functions: {}", available)?;
    writeln!(output, "Unavailable functions: {}", unavailable)?;

    Ok(output)
}