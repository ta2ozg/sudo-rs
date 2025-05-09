use crate::{
    OG_SUDO_STANDARD_LECTURE, PASSWORD, SUDOERS_ALWAYS_LECTURE, SUDOERS_NO_LECTURE,
    SUDOERS_ONCE_LECTURE, SUDOERS_ROOT_ALL, SUDOERS_USER_ALL_ALL, SUDOERS_USER_ALL_NOPASSWD,
    USERNAME,
};
use sudo_test::{Command, Env, User};

#[test]
#[ignore = "gh399"]
fn default_lecture_shown_once() {
    let env = Env([SUDOERS_ROOT_ALL, SUDOERS_ONCE_LECTURE, SUDOERS_USER_ALL_ALL])
        .user(User(USERNAME).password(PASSWORD))
        .build();

    let output = Command::new("sudo")
        .args(["-S", "true"])
        .as_user(USERNAME)
        .stdin(PASSWORD)
        .output(&env);
    output.assert_success();

    assert_contains!(output.stderr(), OG_SUDO_STANDARD_LECTURE);

    let second_sudo = Command::new("sudo")
        .as_user(USERNAME)
        .stdin(PASSWORD)
        .args(["-S", "echo", "Yeah!"])
        .output(&env);

    second_sudo.assert_success();
    assert_not_contains!(second_sudo.stderr(), OG_SUDO_STANDARD_LECTURE);
}

#[test]
#[ignore = "gh399"]
fn lecture_in_stderr() {
    let env = Env([SUDOERS_ROOT_ALL, SUDOERS_ONCE_LECTURE, SUDOERS_USER_ALL_ALL])
        .user(User(USERNAME).password(PASSWORD))
        .build();

    let output = Command::new("sudo")
        .args(["-S", "true"])
        .as_user(USERNAME)
        .stdin(PASSWORD)
        .output(&env);
    output.assert_success();

    assert_contains!(output.stderr(), OG_SUDO_STANDARD_LECTURE);

    assert_not_contains!(output.stdout(), OG_SUDO_STANDARD_LECTURE);
}

#[test]
#[ignore = "gh399"]
fn lecture_always_shown() {
    let env = Env([
        SUDOERS_ROOT_ALL,
        SUDOERS_ONCE_LECTURE,
        SUDOERS_ALWAYS_LECTURE,
    ])
    .user(User(USERNAME).password(PASSWORD))
    .build();

    let output = Command::new("sudo")
        .as_user(USERNAME)
        .stdin(PASSWORD)
        .args(["-S", "true"])
        .output(&env);
    assert!(!output.status().success());

    assert_contains!(output.stderr(), OG_SUDO_STANDARD_LECTURE);

    let second_sudo = Command::new("sudo")
        .as_user(USERNAME)
        .stdin(PASSWORD)
        .args(["-S", "ls"])
        .output(&env);
    assert!(!output.status().success());

    assert_contains!(second_sudo.stderr(), OG_SUDO_STANDARD_LECTURE);
}

#[test]
fn lecture_never_shown() {
    let env = Env([SUDOERS_ROOT_ALL, SUDOERS_USER_ALL_ALL, SUDOERS_NO_LECTURE])
        .user(User(USERNAME).password(PASSWORD))
        .build();

    let output = Command::new("sudo")
        .as_user(USERNAME)
        .stdin(PASSWORD)
        .args(["-S", "echo", "Yeah!"])
        .output(&env);

    output.assert_success();
    assert_not_contains!(output.stderr(), OG_SUDO_STANDARD_LECTURE);
}

#[test]
fn negation_equals_never() {
    let env = Env([SUDOERS_ROOT_ALL, SUDOERS_USER_ALL_ALL, "Defaults  !lecture"])
        .user(User(USERNAME).password(PASSWORD))
        .build();

    let output = Command::new("sudo")
        .as_user(USERNAME)
        .stdin(PASSWORD)
        .args(["-S", "echo", "Yeah!"])
        .output(&env);

    output.assert_success();
    assert_not_contains!(output.stderr(), OG_SUDO_STANDARD_LECTURE);
}

/// Lectures are only shown when password is asked for
#[test]
fn root_user_lecture_not_shown() {
    let env = Env(SUDOERS_ROOT_ALL).build();

    let output = Command::new("sudo")
        .as_user("root")
        .stdin(PASSWORD)
        .args(["-S", "true"])
        .output(&env);

    output.assert_success();
    assert_not_contains!(output.stderr(), OG_SUDO_STANDARD_LECTURE);
}

#[test]
fn nopasswd_lecture_not_shown() {
    let env = Env(SUDOERS_USER_ALL_NOPASSWD)
        .user(User(USERNAME).password(PASSWORD))
        .build();

    let output = Command::new("sudo")
        .as_user(USERNAME)
        .stdin(PASSWORD)
        .args(["-S", "true"])
        .output(&env);

    output.assert_success();
    assert_not_contains!(output.stderr(), OG_SUDO_STANDARD_LECTURE);
}
