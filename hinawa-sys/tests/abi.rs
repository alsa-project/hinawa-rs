// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

extern crate hinawa_sys;
extern crate shell_words;
extern crate tempfile;
use std::env;
use std::error::Error;
use std::path::Path;
use std::mem::{align_of, size_of};
use std::process::Command;
use std::str;
use tempfile::Builder;
use hinawa_sys::*;

static PACKAGES: &[&str] = &["hinawa"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}",
                               &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed,
            self.failed,
            self.failed_to_compile)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = Builder::new().prefix("abi").tempdir().expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!("1",
               get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
               "failed to obtain correct constant value for 1");

    let mut results : Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_value, c_value);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = Builder::new().prefix("abi").tempdir().expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(Layout {size: 1, alignment: 1},
               get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
               "failed to obtain correct layout for char type");

    let mut results : Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_layout, &c_layout);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<dyn Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout {size, alignment})
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<dyn Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let output = str::from_utf8(&output.stdout)?.trim();
    if !output.starts_with("###gir test###") ||
       !output.ends_with("###gir test###") {
        return Err(format!("command {:?} return invalid output, {:?}",
                           &abi_cmd, &output).into());
    }

    Ok(String::from(&output[14..(output.len() - 14)]))
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("HinawaFwFcp", Layout {size: size_of::<HinawaFwFcp>(), alignment: align_of::<HinawaFwFcp>()}),
    ("HinawaFwFcpClass", Layout {size: size_of::<HinawaFwFcpClass>(), alignment: align_of::<HinawaFwFcpClass>()}),
    ("HinawaFwFcpError", Layout {size: size_of::<HinawaFwFcpError>(), alignment: align_of::<HinawaFwFcpError>()}),
    ("HinawaFwNode", Layout {size: size_of::<HinawaFwNode>(), alignment: align_of::<HinawaFwNode>()}),
    ("HinawaFwNodeClass", Layout {size: size_of::<HinawaFwNodeClass>(), alignment: align_of::<HinawaFwNodeClass>()}),
    ("HinawaFwNodeError", Layout {size: size_of::<HinawaFwNodeError>(), alignment: align_of::<HinawaFwNodeError>()}),
    ("HinawaFwRcode", Layout {size: size_of::<HinawaFwRcode>(), alignment: align_of::<HinawaFwRcode>()}),
    ("HinawaFwReq", Layout {size: size_of::<HinawaFwReq>(), alignment: align_of::<HinawaFwReq>()}),
    ("HinawaFwReqClass", Layout {size: size_of::<HinawaFwReqClass>(), alignment: align_of::<HinawaFwReqClass>()}),
    ("HinawaFwResp", Layout {size: size_of::<HinawaFwResp>(), alignment: align_of::<HinawaFwResp>()}),
    ("HinawaFwRespClass", Layout {size: size_of::<HinawaFwRespClass>(), alignment: align_of::<HinawaFwRespClass>()}),
    ("HinawaFwTcode", Layout {size: size_of::<HinawaFwTcode>(), alignment: align_of::<HinawaFwTcode>()}),
    ("HinawaSndDg00x", Layout {size: size_of::<HinawaSndDg00x>(), alignment: align_of::<HinawaSndDg00x>()}),
    ("HinawaSndDg00xClass", Layout {size: size_of::<HinawaSndDg00xClass>(), alignment: align_of::<HinawaSndDg00xClass>()}),
    ("HinawaSndDice", Layout {size: size_of::<HinawaSndDice>(), alignment: align_of::<HinawaSndDice>()}),
    ("HinawaSndDiceClass", Layout {size: size_of::<HinawaSndDiceClass>(), alignment: align_of::<HinawaSndDiceClass>()}),
    ("HinawaSndDiceError", Layout {size: size_of::<HinawaSndDiceError>(), alignment: align_of::<HinawaSndDiceError>()}),
    ("HinawaSndEfw", Layout {size: size_of::<HinawaSndEfw>(), alignment: align_of::<HinawaSndEfw>()}),
    ("HinawaSndEfwClass", Layout {size: size_of::<HinawaSndEfwClass>(), alignment: align_of::<HinawaSndEfwClass>()}),
    ("HinawaSndEfwStatus", Layout {size: size_of::<HinawaSndEfwStatus>(), alignment: align_of::<HinawaSndEfwStatus>()}),
    ("HinawaSndMotu", Layout {size: size_of::<HinawaSndMotu>(), alignment: align_of::<HinawaSndMotu>()}),
    ("HinawaSndMotuClass", Layout {size: size_of::<HinawaSndMotuClass>(), alignment: align_of::<HinawaSndMotuClass>()}),
    ("HinawaSndTscm", Layout {size: size_of::<HinawaSndTscm>(), alignment: align_of::<HinawaSndTscm>()}),
    ("HinawaSndTscmClass", Layout {size: size_of::<HinawaSndTscmClass>(), alignment: align_of::<HinawaSndTscmClass>()}),
    ("HinawaSndUnit", Layout {size: size_of::<HinawaSndUnit>(), alignment: align_of::<HinawaSndUnit>()}),
    ("HinawaSndUnitClass", Layout {size: size_of::<HinawaSndUnitClass>(), alignment: align_of::<HinawaSndUnitClass>()}),
    ("HinawaSndUnitError", Layout {size: size_of::<HinawaSndUnitError>(), alignment: align_of::<HinawaSndUnitError>()}),
    ("HinawaSndUnitType", Layout {size: size_of::<HinawaSndUnitType>(), alignment: align_of::<HinawaSndUnitType>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) HINAWA_FW_FCP_ERROR_LARGE_RESP", "1"),
    ("(gint) HINAWA_FW_FCP_ERROR_TIMEOUT", "0"),
    ("(gint) HINAWA_FW_NODE_ERROR_DISCONNECTED", "0"),
    ("(gint) HINAWA_FW_NODE_ERROR_FAILED", "3"),
    ("(gint) HINAWA_FW_NODE_ERROR_NOT_OPENED", "2"),
    ("(gint) HINAWA_FW_NODE_ERROR_OPENED", "1"),
    ("(gint) HINAWA_FW_RCODE_ADDRESS_ERROR", "7"),
    ("(gint) HINAWA_FW_RCODE_BUSY", "18"),
    ("(gint) HINAWA_FW_RCODE_CANCELLED", "17"),
    ("(gint) HINAWA_FW_RCODE_COMPLETE", "0"),
    ("(gint) HINAWA_FW_RCODE_CONFLICT_ERROR", "4"),
    ("(gint) HINAWA_FW_RCODE_DATA_ERROR", "5"),
    ("(gint) HINAWA_FW_RCODE_GENERATION", "19"),
    ("(gint) HINAWA_FW_RCODE_INVALID", "21"),
    ("(gint) HINAWA_FW_RCODE_NO_ACK", "20"),
    ("(gint) HINAWA_FW_RCODE_SEND_ERROR", "16"),
    ("(gint) HINAWA_FW_RCODE_TYPE_ERROR", "6"),
    ("(gint) HINAWA_FW_TCODE_CYCLE_START", "8"),
    ("(gint) HINAWA_FW_TCODE_LOCK_BOUNDED_ADD", "21"),
    ("(gint) HINAWA_FW_TCODE_LOCK_COMPARE_SWAP", "18"),
    ("(gint) HINAWA_FW_TCODE_LOCK_FETCH_ADD", "19"),
    ("(gint) HINAWA_FW_TCODE_LOCK_LITTLE_ADD", "20"),
    ("(gint) HINAWA_FW_TCODE_LOCK_MASK_SWAP", "17"),
    ("(gint) HINAWA_FW_TCODE_LOCK_REQUEST", "9"),
    ("(gint) HINAWA_FW_TCODE_LOCK_RESPONSE", "11"),
    ("(gint) HINAWA_FW_TCODE_LOCK_VENDOR_DEPENDENT", "23"),
    ("(gint) HINAWA_FW_TCODE_LOCK_WRAP_ADD", "22"),
    ("(gint) HINAWA_FW_TCODE_READ_BLOCK_REQUEST", "5"),
    ("(gint) HINAWA_FW_TCODE_READ_BLOCK_RESPONSE", "7"),
    ("(gint) HINAWA_FW_TCODE_READ_QUADLET_REQUEST", "4"),
    ("(gint) HINAWA_FW_TCODE_READ_QUADLET_RESPONSE", "6"),
    ("(gint) HINAWA_FW_TCODE_STREAM_DATA", "10"),
    ("(gint) HINAWA_FW_TCODE_WRITE_BLOCK_REQUEST", "1"),
    ("(gint) HINAWA_FW_TCODE_WRITE_QUADLET_REQUEST", "0"),
    ("(gint) HINAWA_FW_TCODE_WRITE_RESPONSE", "2"),
    ("(gint) HINAWA_SND_DICE_ERROR_TIMEOUT", "0"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD", "1"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD_CHANNEL", "10"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD_CLOCK", "9"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD_COMMAND", "2"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD_LED", "14"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD_MIRROR", "13"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD_PAN", "11"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD_PARAMETER", "15"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD_QUAD_COUNT", "4"),
    ("(gint) HINAWA_SND_EFW_STATUS_BAD_RATE", "8"),
    ("(gint) HINAWA_SND_EFW_STATUS_COMM_ERR", "3"),
    ("(gint) HINAWA_SND_EFW_STATUS_DSP_TIMEOUT", "7"),
    ("(gint) HINAWA_SND_EFW_STATUS_FLASH_BUSY", "12"),
    ("(gint) HINAWA_SND_EFW_STATUS_LARGE_RESP", "16"),
    ("(gint) HINAWA_SND_EFW_STATUS_OK", "0"),
    ("(gint) HINAWA_SND_EFW_STATUS_TIMEOUT", "6"),
    ("(gint) HINAWA_SND_EFW_STATUS_UNSUPPORTED", "5"),
    ("(gint) HINAWA_SND_UNIT_ERROR_DISCONNECTED", "0"),
    ("(gint) HINAWA_SND_UNIT_ERROR_FAILED", "7"),
    ("(gint) HINAWA_SND_UNIT_ERROR_LOCKED", "4"),
    ("(gint) HINAWA_SND_UNIT_ERROR_NOT_OPENED", "3"),
    ("(gint) HINAWA_SND_UNIT_ERROR_OPENED", "2"),
    ("(gint) HINAWA_SND_UNIT_ERROR_UNLOCKED", "5"),
    ("(gint) HINAWA_SND_UNIT_ERROR_USED", "1"),
    ("(gint) HINAWA_SND_UNIT_ERROR_WRONG_CLASS", "6"),
    ("(gint) HINAWA_SND_UNIT_TYPE_BEBOB", "3"),
    ("(gint) HINAWA_SND_UNIT_TYPE_DICE", "1"),
    ("(gint) HINAWA_SND_UNIT_TYPE_DIGI00X", "5"),
    ("(gint) HINAWA_SND_UNIT_TYPE_FIREFACE", "8"),
    ("(gint) HINAWA_SND_UNIT_TYPE_FIREWORKS", "2"),
    ("(gint) HINAWA_SND_UNIT_TYPE_MOTU", "7"),
    ("(gint) HINAWA_SND_UNIT_TYPE_OXFW", "4"),
    ("(gint) HINAWA_SND_UNIT_TYPE_TASCAM", "6"),
];


