use crate::core::Deployable;

pub struct Setup {}

impl Setup {
    fn attempt_program_with_uv(&self, program: &str) -> Option<String> {
        let uv_install_msg = "{} no instalado. Corre: \n- uv tool install {}";
        let attempt = self
            .cmd()
            .execute_with_output("uv", &["run", program, "--version"]);
        if attempt.is_err() {
            Some(uv_install_msg.replace("{}", program))
        } else {
            None
        }
    }
}
impl Deployable for Setup {
    fn name(&self) -> &str {
        "setup"
    }

    fn deploy(&self) -> Result<(), std::io::Error> {
        let mut missing_programs: Vec<Option<String>> = Vec::new();
        let uv_attempt = self.cmd().execute_with_output("uv", &["--version"]);
        if uv_attempt.is_err() {
            missing_programs.push(Some("uv no instalado".to_string()));
        } else {
            missing_programs.push(None);
        }
        missing_programs.push(self.attempt_program_with_uv("git-cliff"));
        missing_programs.push(self.attempt_program_with_uv("commitizen"));
        missing_programs.retain(|x| !x.is_none());
        let missing_programs: Vec<String> = missing_programs.into_iter().flatten().collect();
        if missing_programs.is_empty() {
            println!("\x1b[32m✓ All programs installed\x1b[0m");
            Ok(())
        } else {
            println!("{}", missing_programs.join("\n"));
            Ok(())
        }
    }
}

// uv tool install bump-my-version
// winget install git-cliff
// mac: sudo port install git-cliff
