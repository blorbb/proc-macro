let project_name = variable::get("project-name");

if variable::get("include_readmes") {
    file::rename("README-template.md", "README.md");
} else {
    file::delete("README-template.md");
    file::delete({project_name} + "-macro/README.md");
    file::delete({project_name} + "-core/README.md");
}
