use glib_build_tools;

fn main() {
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "smartshreds_template.gresource",
    );
}