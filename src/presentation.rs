use std::env;
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

pub struct Presentation {
    source_md: PathBuf,
    target_html: PathBuf,
    base_dir: Option<PathBuf>,
}

impl Presentation {
    pub fn new(
        source_md: PathBuf,
        target_html: PathBuf,
        base_dir: Option<PathBuf>,
    ) -> Presentation {
        Presentation {
            source_md,
            target_html,
            base_dir,
        }
    }

    pub fn build(&self) {
        let _ = match &self.base_dir {
            Some(dir_path) => env::set_current_dir(dir_path),
            None => Ok(()),
        };
        // Check if pandoc is available
        let _pandoc = Command::new("pandoc")
            .arg("--version")
            .output()
            .expect("Failed to run pandoc");

        let _build = Command::new("pandoc")
            .arg("--to=revealjs")
            .arg("--slide-level=2")
            //.arg("--css=um.css")
            .arg("--standalone")
            .arg(format!("--output={}", &self.target_html.display()))
            .arg(&self.source_md)
            .output()
            .expect("Failed to build");
        // TODO: check if build is successful
        println!("{:?}", _build);
    }

    pub fn edit(&self) {
        let _ = match &self.base_dir {
            Some(dir_path) => env::set_current_dir(dir_path),
            None => Ok(()),
        };
        let editor = std::env::var("EDITOR").unwrap_or("vim".to_string());
        let _edit = Command::new(editor)
            .arg(&self.source_md)
            .status()
            .expect("Failed to edit");
    }

    pub fn serve(&self) {
        let _ = match &self.base_dir {
            Some(dir_path) => env::set_current_dir(dir_path),
            None => Ok(()),
        };

        self.build();
        println!("Successfully built");

        let _live_server_check = Command::new("live-server")
            .arg("--version")
            .output()
            .expect("Failed to run live-server");

        let live_server = Command::new("live-server")
            .arg(&self.target_html)
            .spawn()
            .expect("Failed to start live-server");
        println!("{:?}", live_server);

        // stop live-server when pelp stops

        let _inotifywait_check = Command::new("inotifywait")
            .arg("--help")
            .output()
            .expect("Failed to run inotifywait");

        loop {
            let _inotifywait = Command::new("inotifywait")
                .arg("-e")
                .arg("modify")
                .arg(&self.source_md)
                .output()
                .expect("Failed to run inotifywait");
            self.build();
        }
    }
}

impl fmt::Display for Presentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(
            f,
            "Markdown: {:?}\nHTML:     {:?}\nBase Dir: {:?}",
            self.source_md, self.target_html, self.base_dir
        )
    }
}
