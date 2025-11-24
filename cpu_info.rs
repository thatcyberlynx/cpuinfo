use anyhow::Result;
use raw_cpuid::CpuId;
use sysinfo::System;

use crate::cpu_features::CpuFeatures;

pub struct CpuInfo {
    pub name: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub base_speed: u64,
    pub boost_speed: u64,
}

pub fn get_cpu_info() -> Result<CpuInfo> {
    let mut system = System::new_all();
    system.refresh_all();

    // Basic CPU information
    if let Some(cpu) = system.cpus().first() {
        let physical_cores = num_cpus::get_physical();
        let logical_cores = num_cpus::get();
        let cpu_freq = cpu.frequency();
        let boost_speed = (cpu_freq as f64 * 1.2) as u64;

        Ok(CpuInfo {
            name: cpu.brand().to_string(),
            physical_cores,
            logical_cores,
            base_speed: cpu_freq,
            boost_speed,
        })
    } else {
        anyhow::bail!("Could not retrieve CPU information")
    }
}

pub fn get_cpu_features() -> Result<Vec<CpuFeatures>> {
    let cpuid = CpuId::new();
    let mut features = Vec::new();

    if let Some(feature_info) = cpuid.get_feature_info() {
        // Basic instruction sets
        features.push(CpuFeatures::new("MMX", feature_info.has_mmx()));
        features.push(CpuFeatures::new("SSE", feature_info.has_sse()));
        features.push(CpuFeatures::new("SSE2", feature_info.has_sse2()));
        features.push(CpuFeatures::new("SSE3", feature_info.has_sse3()));
        features.push(CpuFeatures::new("SSSE3", feature_info.has_ssse3()));
        features.push(CpuFeatures::new("SSE4.1", feature_info.has_sse41()));
        features.push(CpuFeatures::new("SSE4.2", feature_info.has_sse42()));
        features.push(CpuFeatures::new("POPCNT", feature_info.has_popcnt()));
        features.push(CpuFeatures::new("PCLMULQDQ", feature_info.has_pclmulqdq()));
        
        // Virtualization and security
        features.push(CpuFeatures::new("NX", feature_info.has_cnxtid()));
        features.push(CpuFeatures::new("RDTSCP", feature_info.has_tsc()));
        features.push(CpuFeatures::new("TSC", feature_info.has_tsc()));
        features.push(CpuFeatures::new("SMX", feature_info.has_smx()));
        features.push(CpuFeatures::new("VMX", feature_info.has_vmx()));
        
        // Advanced features
        features.push(CpuFeatures::new("AES", feature_info.has_aesni()));
        features.push(CpuFeatures::new("AVX", feature_info.has_avx()));
        features.push(CpuFeatures::new("F16C", feature_info.has_f16c()));
        features.push(CpuFeatures::new("FMA", feature_info.has_fma()));
        features.push(CpuFeatures::new("XSAVE", feature_info.has_xsave()));
        features.push(CpuFeatures::new("OSXSAVE", feature_info.has_oxsave()));
        
        // Legacy and special purpose
        features.push(CpuFeatures::new("FPU", feature_info.has_fpu()));
        features.push(CpuFeatures::new("DE", feature_info.has_de()));
        features.push(CpuFeatures::new("PSN", feature_info.has_psn()));
        features.push(CpuFeatures::new("MSR", feature_info.has_msr()));
        features.push(CpuFeatures::new("CX8", feature_info.has_cmpxchg8b()));
        features.push(CpuFeatures::new("APIC", feature_info.has_apic()));
    }

    if let Some(extended_feature_info) = cpuid.get_extended_feature_info() {
        // Extended instruction sets
        features.push(CpuFeatures::new("AVX2", extended_feature_info.has_avx2()));
        features.push(CpuFeatures::new("BMI1", extended_feature_info.has_bmi1()));
        features.push(CpuFeatures::new("BMI2", extended_feature_info.has_bmi2()));
        features.push(CpuFeatures::new("ADX", extended_feature_info.has_adx()));
        features.push(CpuFeatures::new("SHA", extended_feature_info.has_sha()));
        
        // Advanced capabilities
        features.push(CpuFeatures::new("RDRAND", extended_feature_info.has_rdta()));
        features.push(CpuFeatures::new("RDSEED", extended_feature_info.has_rdseed()));
        features.push(CpuFeatures::new("INVPCID", extended_feature_info.has_invpcid()));
        features.push(CpuFeatures::new("PREFETCHWT1", extended_feature_info.has_prefetchwt1()));
        features.push(CpuFeatures::new("UMIP", extended_feature_info.has_umip()));
        features.push(CpuFeatures::new("FSGSBASE", extended_feature_info.has_fsgsbase()));
        
        // Security extensions
        features.push(CpuFeatures::new("SMEP", extended_feature_info.has_smep()));
        features.push(CpuFeatures::new("SMAP", extended_feature_info.has_smap()));
        features.push(CpuFeatures::new("PKU", extended_feature_info.has_pku()));
        
        // AVX512 features
        features.push(CpuFeatures::new("AVX512F", extended_feature_info.has_avx512f()));
        features.push(CpuFeatures::new("AVX512DQ", extended_feature_info.has_avx512dq()));
        features.push(CpuFeatures::new("AVX512IFMA", extended_feature_info.has_avx512_ifma()));
        features.push(CpuFeatures::new("AVX512PF", extended_feature_info.has_avx512pf()));
        features.push(CpuFeatures::new("AVX512ER", extended_feature_info.has_avx512er()));
        features.push(CpuFeatures::new("AVX512CD", extended_feature_info.has_avx512cd()));
        features.push(CpuFeatures::new("AVX512BW", extended_feature_info.has_avx512bw()));
        features.push(CpuFeatures::new("AVX512VL", extended_feature_info.has_avx512vl()));
        features.push(CpuFeatures::new("AVX512VBMI", extended_feature_info.has_avx512vbmi()));
        features.push(CpuFeatures::new("AVX512VNNI", extended_feature_info.has_avx512vnni()));
    }

    if let Some(extended_function) = cpuid.get_extended_processor_and_feature_identifiers() {
        // Page size / memory extensions
        features.push(CpuFeatures::new("PAE", extended_function.has_1gib_pages()));
        features.push(CpuFeatures::new("PSE36", extended_function.has_sse4a()));
        features.push(CpuFeatures::new("PGE", extended_function.has_1gib_pages()));
    }

    Ok(features)
}