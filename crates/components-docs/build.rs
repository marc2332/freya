use tokio::fs::{
    read_dir,
    read_to_string,
    write,
};

#[tokio::main]
async fn main() {
    println!("cargo::rerun-if-changed=../components/src/*.rs");

    let mut files = read_dir("../components/src").await.unwrap();

    let mut run_file = "#[rustfmt::skip]
#[tokio::main]
async fn main(){"
        .to_string();

    while let Ok(Some(file)) = files.next_entry().await {
        if file.file_type().await.unwrap().is_file() {
            let content = read_to_string(file.path()).await.unwrap();
            let mut code_block = String::new();
            let mut found_code_block = false;
            let mut size = String::new();
            let mut name = String::new();

            for line in content.lines() {
                if found_code_block {
                    if line.starts_with("/// -->") {
                        run_file.push_str(&format!(
                            r#"
    {{
        mod preview {{
            use freya_testing::prelude::*;

            pub async fn run(){{
{code_block}
                let mut utils = launch_test(app);
                utils.resize({size}.into());
                utils.wait_for_update().await;
                utils.save_snapshot("./crates/components/images/{name}.png");
            }}
        }}

        preview::run().await;
    }}
"#
                        ));

                        found_code_block = false;
                        name.clear();
                        size.clear();
                        code_block.clear();
                        continue;
                    }

                    if line == "///" {
                        continue;
                    }

                    let mut line_sanitized = line.replace("/// # ", "                ");
                    line_sanitized = line_sanitized.replace("/// ", "                ");
                    line_sanitized.push('\n');
                    code_block.push_str(&line_sanitized);
                }

                if line.starts_with("/// <!--- PREVIEW") {
                    found_code_block = true;
                    let preview_config = line.replace("/// <!--- PREVIEW", "");
                    let preview_config = preview_config.split_whitespace().collect::<Vec<_>>();
                    name = preview_config
                        .first()
                        .expect("Could not find name for preview")
                        .to_string();
                    size = preview_config
                        .get(1)
                        .expect("Could not find size for preview")
                        .to_string();
                }
            }
        }
    }

    run_file.push('}');

    write("./src/main.rs", &run_file).await.unwrap();
}
