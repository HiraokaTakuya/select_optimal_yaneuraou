fn is_zen1(fi: &raw_cpuid::FeatureInfo) -> bool {
    fi.family_id() == 0x17 && fi.model_id() <= 0x2F
}
fn is_zen2(fi: &raw_cpuid::FeatureInfo) -> bool {
    fi.family_id() == 0x17 && fi.model_id() >= 0x30
}
fn is_zen3(fi: &raw_cpuid::FeatureInfo) -> bool {
    fi.family_id() == 0x19
}

#[derive(Clone, Copy)]
enum Target {
    Avx2,
    Avx512,
    Avx512Vnni,
    NoSse,
    Sse2,
    Sse41,
    Sse42,
    Ssse3,
    Zen1,
    Zen2,
    Zen3,
}

impl Target {
    fn to_nnue_normal_clang_file_name(self) -> &'static str {
        match self {
            Self::Avx2 => "YaneuraOu_NNUE-normal-clang++-avx2.exe",
            Self::Avx512 => "YaneuraOu_NNUE-normal-clang++-avx512.exe",
            Self::Avx512Vnni => "YaneuraOu_NNUE-normal-clang++-avx512vnni.exe",
            Self::NoSse => "YaneuraOu_NNUE-normal-clang++-no_sse.exe",
            Self::Sse2 => "YaneuraOu_NNUE-normal-clang++-sse2.exe",
            Self::Sse41 => "YaneuraOu_NNUE-normal-clang++-sse41.exe",
            Self::Sse42 => "YaneuraOu_NNUE-normal-clang++-sse42.exe",
            Self::Ssse3 => "YaneuraOu_NNUE-normal-clang++-ssse3.exe",
            Self::Zen1 => "YaneuraOu_NNUE-normal-clang++-zen1.exe",
            Self::Zen2 => "YaneuraOu_NNUE-normal-clang++-zen2.exe",
            Self::Zen3 => "YaneuraOu_NNUE-normal-clang++-zen3.exe",
        }
    }
}

impl std::convert::From<raw_cpuid::CpuId> for Target {
    fn from(cpuid: raw_cpuid::CpuId) -> Self {
        match cpuid.get_feature_info() {
            None => Self::NoSse,
            Some(fi) => {
                if is_zen3(&fi) {
                    Self::Zen3
                } else if is_zen2(&fi) {
                    Self::Zen2
                } else if is_zen1(&fi) {
                    Self::Zen1
                } else {
                    match cpuid.get_extended_feature_info() {
                        Some(efi) if efi.has_avx512vnni() => Self::Avx512Vnni,
                        Some(efi) if efi.has_avx512f() => Self::Avx512,
                        Some(efi) if efi.has_bmi2() => Self::Avx2,
                        Some(_) | None => {
                            if fi.has_sse42() {
                                Self::Sse42
                            } else if fi.has_sse41() {
                                Self::Sse41
                            } else if fi.has_ssse3() {
                                Self::Ssse3
                            } else if fi.has_sse2() {
                                Self::Sse2
                            } else {
                                Self::NoSse
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let target = Target::from(raw_cpuid::CpuId::new()).to_nnue_normal_clang_file_name();
    let mut path_buf = std::env::current_exe().expect("failed to get current execute file path");
    path_buf.set_file_name(target);
    let path = path_buf
        .as_path()
        .to_str()
        .expect("failed to encode file path to UTF-8");
    let (cmd, arg0) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
    std::process::Command::new(cmd)
        .current_dir(".")
        .args([arg0, path])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("process wasn't running");
}
