use sudo_test::{Command, Env};

use crate::{helpers, EnvList, SUDOERS_ROOT_ALL_NOPASSWD, SUDO_RS_IS_UNSTABLE};

// see 'environment' section in `man sudo`
// "SUDO_PS1: If set, PS1 will be set to its value for the program being run."
#[test]
fn ps1_env_var_is_set_when_sudo_ps1_is_set() {
    let ps1 = "abc";
    let env = Env(SUDOERS_ROOT_ALL_NOPASSWD).build();

    let sudo_abs_path = Command::new("which").arg("sudo").output(&env).stdout();
    let env_abs_path = Command::new("which").arg("env").output(&env).stdout();

    // run sudo in an empty environment
    let stdout = Command::new("env")
        .args(["-i", SUDO_RS_IS_UNSTABLE])
        .arg(format!("SUDO_PS1={ps1}"))
        .args([&sudo_abs_path, &env_abs_path])
        .output(&env)
        .stdout();
    let sudo_env = helpers::parse_env_output(&stdout);

    assert_eq!(Some(ps1), sudo_env.get("PS1").copied());
    assert!(!sudo_env.contains_key("SUDO_PS1"));
}

// sudo removes env vars whose values start with `()` but that does not affect the SUDO_PS1 feature
#[test]
fn can_start_with_parentheses() {
    let ps1 = "() abc";
    let env = Env(SUDOERS_ROOT_ALL_NOPASSWD).build();

    let sudo_abs_path = Command::new("which").arg("sudo").output(&env).stdout();
    let env_abs_path = Command::new("which").arg("env").output(&env).stdout();

    // run sudo in an empty environment
    let stdout = Command::new("env")
        .args(["-i", SUDO_RS_IS_UNSTABLE])
        .arg(format!("SUDO_PS1={ps1}"))
        .args([&sudo_abs_path, &env_abs_path])
        .output(&env)
        .stdout();
    let sudo_env = helpers::parse_env_output(&stdout);

    assert_eq!(Some(ps1), sudo_env.get("PS1").copied());
    assert!(!sudo_env.contains_key("SUDO_PS1"));
}

fn preserved_when_in_env_list(env_list: &EnvList) {
    let ps1 = "abc";
    let env = Env([
        SUDOERS_ROOT_ALL_NOPASSWD,
        &format!("Defaults {env_list} = SUDO_PS1"),
    ])
    .build();

    let stdout = Command::new("env")
        .arg(format!("SUDO_PS1={ps1}"))
        .args(["sudo", "env"])
        .output(&env)
        .stdout();
    let sudo_env = helpers::parse_env_output(&stdout);

    assert_eq!(Some(ps1), sudo_env.get("PS1").copied());
    assert_eq!(Some(ps1), sudo_env.get("SUDO_PS1").copied());
}

#[test]
fn preserved_when_in_env_keep() {
    preserved_when_in_env_list(&EnvList::Keep)
}

#[test]
fn preserved_when_in_env_check() {
    preserved_when_in_env_list(&EnvList::Check)
}

fn sudo_ps1_has_precedence_over_env_list_ps1(env_list: &EnvList) {
    let sudo_ps1 = "abc";
    let ps1 = "def";
    let env = Env([
        SUDOERS_ROOT_ALL_NOPASSWD,
        &format!("Defaults {env_list} = PS1"),
    ])
    .build();

    let stdout = Command::new("env")
        .arg(format!("PS1={ps1}"))
        .arg(format!("SUDO_PS1={sudo_ps1}"))
        .args(["sudo", "env"])
        .output(&env)
        .stdout();
    let sudo_env = helpers::parse_env_output(&stdout);

    assert_eq!(Some(sudo_ps1), sudo_env.get("PS1").copied());
    assert_eq!(None, sudo_env.get("SUDO_PS1").copied());
}

#[test]
fn sudo_ps1_has_precedence_over_env_keep_ps1() {
    sudo_ps1_has_precedence_over_env_list_ps1(&EnvList::Keep)
}

#[test]
fn sudo_ps1_has_precedence_over_env_check_ps1() {
    sudo_ps1_has_precedence_over_env_list_ps1(&EnvList::Check)
}

#[test]
fn ps1_is_set_even_if_sudo_ps1_fails_the_env_check_check() {
    let sudo_ps1 = "a%c";
    let env = Env([SUDOERS_ROOT_ALL_NOPASSWD, "Defaults env_check = SUDO_PS1"]).build();

    let stdout = Command::new("env")
        .arg(format!("SUDO_PS1={sudo_ps1}"))
        .args(["sudo", "env"])
        .output(&env)
        .stdout();
    let sudo_env = helpers::parse_env_output(&stdout);

    assert_eq!(Some(sudo_ps1), sudo_env.get("PS1").copied());
    assert_eq!(None, sudo_env.get("SUDO_PS1").copied());
}

#[test]
fn ps1_is_set_even_if_sudo_ps1_fails_the_env_keep_check() {
    let sudo_ps1 = "() ab";
    let env = Env([SUDOERS_ROOT_ALL_NOPASSWD, "Defaults env_keep = SUDO_PS1"]).build();

    let stdout = Command::new("env")
        .arg(format!("SUDO_PS1={sudo_ps1}"))
        .args(["sudo", "env"])
        .output(&env)
        .stdout();
    let sudo_env = helpers::parse_env_output(&stdout);

    assert_eq!(Some(sudo_ps1), sudo_env.get("PS1").copied());
    assert_eq!(None, sudo_env.get("SUDO_PS1").copied());
}
